use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::mpsc::{self, Receiver, Sender};

#[allow(unused_imports)]
use hashbrown::{HashMap, HashSet};

use petgraph::graph::NodeIndex;
use petgraph::Graph;
use rayon::prelude::*;

/// A single entity in the simulation.
#[derive(PartialEq, Debug, Clone)]
pub struct Entity {
    pub resources: HashMap<String, f64>,
}

impl Entity {
    #[allow(dead_code)]
    pub fn get_resource(&self, resource_name: &String) -> f64 {
        *self
            .resources
            .get(resource_name)
            .expect("Resource {resource_name} not found")
    }
}

/// A possible state in the markov chain of the simulation, which is only dependent on
/// the configuration of the entities in the simulation.
#[derive(Clone, Debug)]
pub struct State {
    pub entities: HashMap<String, Entity>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (name, entity) in &self.entities {
            for (resource_name, amount) in &entity.resources {
                (name, resource_name, amount.to_bits()).hash(state);
            }
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        let self_hasher = &mut DefaultHasher::new();
        self.hash(self_hasher);
        let other_hasher = &mut DefaultHasher::new();
        other.hash(other_hasher);
        self_hasher.finish() == other_hasher.finish()
    }
}

impl Eq for State {}

impl State {
    #[allow(dead_code)]
    pub fn get_entity(&self, entity_name: &String) -> Entity {
        self.entities
            .get(entity_name)
            .expect("entity {entity_name} not found")
            .clone()
    }

    pub fn get_hash(&self) -> u64 {
        let mut hasher = &mut DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn apply_actions(
        &self,
        actions: &Vec<Action>,
        resources: &HashMap<String, Resource>,
    ) -> State {
        let mut new_state = self.clone();
        for action in actions {
            new_state
                .entities
                .get_mut(&action.entity)
                .expect("Entity {action.entity} not found in state")
                .resources
                .insert(action.resource.clone(), action.new_amount);

            let capacity_per_entity = &resources
                .get(&action.resource)
                .expect("Resource {action.resource} not found in resources")
                .capacity_per_entity;

            if let Capacity::Limited(limit) = capacity_per_entity {
                if action.new_amount > *limit {
                    panic!(
                        "Resource limit per entity exceeded for resource {resource_name}",
                        resource_name = action.resource
                    );
                }
            }
        }
        new_state
    }
}

/// An action a rule can take on a single entity and resource when its condition is met.
#[derive(PartialEq, Clone, Debug)]
pub struct Action {
    pub name: String,
    pub resource: String,
    pub entity: String,
    pub new_amount: f64,
}

/// An abstraction over the transition rates of the underlying markov chain.
#[derive(Clone)]
pub struct Rule {
    pub description: String,

    /// The conditions that must be met for the rule to be applied.
    pub condition: fn(&State) -> bool,

    /// A measure of how often the rule is applied when the condition is met.
    ///
    /// As two rules cannot be applied at the same time, first, the probability that no rule applies is calculated.
    /// The remaining probability is divived among the remaining rules according to their weights.
    pub probability_weight: f64,

    /// A function which specifies to which state the rule leads when applied.
    ///
    /// The function takes the current state as input and returns multiple actions.
    /// A new state is then created by applying all actions to the current state.
    pub actions: fn(&State) -> Vec<Action>,
}

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum Capacity {
    Limited(f64),
    Unlimited,
}

/// A resource in the simulation which may or may not have a capacity.
///
/// A resource is essentially a parameter an entity and thus ultimately a state can have.
/// The capacity is a constrain on the amount of the resource being distributed among the entities.
/// It is allowed that the sum of the amounts of a resource among all entities is lesser than the capacity.
/// It is assumed that the capacity is always greater than or equal to zero.
///
/// The capacity_per_entity is an additional constrain on the amount of the resource an individual entity can have.
/// This can again be unlimited.
#[derive(PartialEq, Clone, Debug)]
pub struct Resource {
    pub description: String,
    pub capacity: Capacity,
    pub capacity_per_entity: Capacity,
}

#[derive(PartialEq, Clone, Debug)]
struct RuleCache {
    condition: HashMap<u64, bool>,
    actions: HashMap<u64, u64>,
}

#[derive(PartialEq, Clone, Debug)]
struct Cache {
    pub rules: HashMap<String, RuleCache>,
}

#[derive(PartialEq, Clone, Debug)]
struct ConditionCacheUpdate {
    pub rule_name: String,
    pub base_state_hash: u64,
    pub result: bool,
}

#[derive(PartialEq, Clone, Debug)]
struct ActionCacheUpdate {
    pub rule_name: String,
    pub base_state_hash: u64,
    pub new_state_hash: u64,
}

/// All information and methods needed to run the simulation.
///
/// All information is managed by the methods of this struct.
/// Do not change properties manually.
#[derive(Clone)]
pub struct Simulation {
    /// All resources in the simulation.
    ///
    /// The key is the name of the resource, while the value the resource itself.
    /// This must not change after initialization.
    pub resources: HashMap<String, Resource>,

    /// The initial state of the simulation.
    ///
    /// This state has a starting probability of 1.
    /// This must not change after initialization.
    pub initial_state: State,

    /// All states which are possible at at some point during the simulation.
    ///
    /// The key is the hash of the state, while the value is the state itself.
    pub possible_states: HashMap<u64, State>,

    /// All states which are possible at the current timestep.
    ///
    /// The key is the hash of the state, while the value is the probability that this state occurs.
    pub reachable_states: HashMap<u64, f64>,

    /// All rules in the simulation.
    ///
    /// This must not change after initialization.
    pub rules: HashMap<String, Rule>,

    /// The current timestep of the simulation, starting at 0.
    pub time: u64,

    /// The current entropy of the probability distribution of the reachable_states.
    pub entropy: f64,

    /// The cache used for performance purposes.
    cache: Cache,
}

impl Simulation {
    /// Creates a new simulation with the given resources, initial state and rules.
    pub fn new(
        resources: HashMap<String, Resource>,
        initial_state: State,
        rules: HashMap<String, Rule>,
    ) -> Simulation {
        let initial_state_hash = initial_state.get_hash();

        let rule_caches: HashMap<String, RuleCache> = rules
            .par_iter()
            .map(|(name, _)| {
                (
                    name.clone(),
                    RuleCache {
                        condition: HashMap::new(),
                        actions: HashMap::new(),
                    },
                )
            })
            .collect();

        Simulation {
            resources,
            initial_state: initial_state.clone(),
            possible_states: HashMap::from([(initial_state_hash, initial_state)]),
            reachable_states: HashMap::from([(initial_state_hash, 1.0)]),
            rules,
            time: 0,
            entropy: 0.,
            cache: Cache { rules: rule_caches },
        }
    }

    /// Runs the simulation for one timestep.
    pub fn next_step(&mut self) {
        self.update_reachable_states();
        self.entropy = self.get_entropy();
        self.time += 1;
    }

    /// Appends a state to reachable_states or increases its probability if it already exists.
    ///
    /// It also adds the state to possible_states if it is not already there.
    fn append_reachable_state(&mut self, new_state_hash: u64, new_state_probability: f64) {
        match self.reachable_states.get_mut(&new_state_hash) {
            Some(probability) => {
                *probability += new_state_probability;
            }
            None => {
                self.reachable_states
                    .insert(new_state_hash, new_state_probability);
            }
        }
    }

    /// Checks if the given state satisfies all resource constrains.
    fn check_resource_capacities(&self, new_state: &State) {
        for (resource_name, resource) in &self.resources {
            match &resource.capacity {
                Capacity::Limited(limit) => {
                    let mut total_amount: f64 = 0.;
                    for (entity_name, entity) in &new_state.entities {
                        let entity_amount = entity
                            .resources
                            .get(resource_name)
                            .expect("Entity {entity_name} does not have resource {resource_name}");
                        if *entity_amount < 0. {
                            panic!(
                                "Entity {} has negative amount of resource {}",
                                entity_name, resource_name
                            );
                        }
                        total_amount += entity_amount;
                        if total_amount > *limit {
                            panic!(
                                "Resource limit exceeded for resource {resource_name}",
                                resource_name = resource_name
                            );
                        }
                    }
                }
                Capacity::Unlimited => {
                    for (entity_name, entity) in &new_state.entities {
                        let entity_amount = entity
                            .resources
                            .get(resource_name)
                            .expect("Entity {entity_name} does not have resource {resource_name}");
                        if entity_amount < &0. {
                            panic!(
                                "Entity {} has negative amount of resource {}",
                                entity_name, resource_name
                            );
                        }
                    }
                }
            }
        }
    }

    /// Checks if a given rule applies to the given state using or updating the cache respectively.
    fn check_if_rule_applies(
        &self,
        rule_name: &String,
        state_hash: &u64,
    ) -> (bool, Option<ConditionCacheUpdate>) {
        let rule_cache = self
            .cache
            .rules
            .get(rule_name)
            .expect("Rule {rule_name} not found in cache");
        let rule = self
            .rules
            .get(rule_name)
            .expect("Rule {rule_name} not found");
        if rule.probability_weight == 0. {
            return (false, None);
        }
        match rule_cache.condition.get(state_hash) {
            Some(result) => (*result, None),
            None => {
                let state = self
                    .possible_states
                    .get(state_hash)
                    .expect("State with hash {state_hash} not found in possible_states");
                let result = (rule.condition)(state);
                let cache = ConditionCacheUpdate {
                    rule_name: rule_name.clone(),
                    base_state_hash: *state_hash,
                    result,
                };
                (result, Some(cache))
            }
        }
    }

    /// Gets the state the given rule results in from the given state using or updating the cache respectively.
    fn get_new_state(
        &self,
        base_state_hash: &u64,
        rule_name: &String,
    ) -> (State, Option<ActionCacheUpdate>) {
        let rule_cache = self
            .cache
            .rules
            .get(rule_name)
            .expect("Rule {rule_name} not found in cache");

        if let Some(state_hash) = rule_cache.actions.get(base_state_hash) {
            if let Some(new_state) = self.possible_states.get(state_hash) {
                return (new_state.clone(), None);
            }
        }

        let rule = self
            .rules
            .get(rule_name)
            .expect("Rule {rule_name} not found");
        let base_state = self
            .possible_states
            .get(base_state_hash)
            .expect("Base state {base_state_hash} not found in possible_states");
        let actions = (rule.actions)(base_state);

        let new_state = base_state.apply_actions(&actions, &self.resources);

        self.check_resource_capacities(&new_state);

        let new_state_hash = new_state.get_hash();
        let cache_update = ActionCacheUpdate {
            rule_name: rule_name.clone(),
            base_state_hash: *base_state_hash,
            new_state_hash,
        };
        (new_state, Some(cache_update))
    }

    // Add all reachable states from the base state to reachable_states and possible_states while using or updating the cache respectively.
    fn get_reachable_states_from_base_state(
        &self,
        base_state_hash: &u64,
        base_state_probability: &f64,
    ) -> (
        HashMap<u64, f64>,
        HashMap<u64, State>,
        Vec<ConditionCacheUpdate>,
        Vec<ActionCacheUpdate>,
    ) {
        let mut new_base_state_probability = *base_state_probability;
        let mut applying_rules_probability_weight_sum = 0.;
        let mut reachable_states_from_base_state_by_rule_probability_weight: HashMap<u64, f64> =
            HashMap::new();

        let mut condition_cache_updates = Vec::new();
        let mut action_cache_updates = Vec::new();

        let mut new_possible_states = HashMap::new();

        for (rule_name, rule) in &self.rules {
            let (rule_applies, condition_cache_update) =
                self.check_if_rule_applies(rule_name, base_state_hash);
            if let Some(cache) = condition_cache_update {
                condition_cache_updates.push(cache);
            }
            if rule_applies {
                new_base_state_probability *= 1. - rule.probability_weight;
                applying_rules_probability_weight_sum += rule.probability_weight;
                let (new_state, action_cache_update) =
                    self.get_new_state(base_state_hash, rule_name);
                if let Some(cache) = action_cache_update {
                    action_cache_updates.push(cache);
                }
                let new_state_hash = new_state.get_hash();
                new_possible_states.insert(new_state_hash, new_state);
                reachable_states_from_base_state_by_rule_probability_weight
                    .insert(new_state_hash, rule.probability_weight);
            }
        }

        let mut new_reachable_states: HashMap<u64, f64> = HashMap::new();

        if new_base_state_probability > 0. {
            new_reachable_states.insert(*base_state_hash, new_base_state_probability);
        }

        let probabilities_for_reachable_states_from_base_state =
            Simulation::get_probabilities_for_reachable_states_from_base_state(
                reachable_states_from_base_state_by_rule_probability_weight,
                base_state_hash,
                *base_state_probability,
                new_base_state_probability,
                applying_rules_probability_weight_sum,
            );
        probabilities_for_reachable_states_from_base_state
            .iter()
            .for_each(|(new_state_hash, new_state_probability)| {
                new_reachable_states.insert(*new_state_hash, *new_state_probability);
            });
        (
            new_reachable_states,
            new_possible_states,
            condition_cache_updates,
            action_cache_updates,
        )
    }

    fn get_probabilities_for_reachable_states_from_base_state(
        reachable_states_from_base_state_by_rule_probability_weight: HashMap<u64, f64>,
        base_state_hash: &u64,
        old_base_state_probability: f64,
        new_base_state_probability: f64,
        applying_rules_probability_weight_sum: f64,
    ) -> HashMap<u64, f64> {
        HashMap::from_par_iter(
            reachable_states_from_base_state_by_rule_probability_weight
                .par_iter()
                .filter_map(|(new_reachable_state_hash, rule_probability_weight)| {
                    if new_reachable_state_hash != base_state_hash {
                        let new_reachable_state_probability = rule_probability_weight
                            * old_base_state_probability
                            * (1. - new_base_state_probability)
                            / applying_rules_probability_weight_sum;
                        Option::Some((*new_reachable_state_hash, new_reachable_state_probability))
                    } else {
                        Option::None
                    }
                }),
        )
    }

    // TODO: Reimplement multithreading
    /// Update reachable_states and possible_states to the next time step.
    fn update_reachable_states(&mut self) {
        let (condition_cache_updates_tx, condition_cache_updates_rx) = mpsc::channel();
        let (action_cache_updates_tx, action_cache_updates_rx) = mpsc::channel();

        let old_reachable_states = self.reachable_states.clone();
        self.reachable_states.clear();
        old_reachable_states
            .iter()
            .for_each(|(base_state_hash, base_state_probability)| {
                let (
                    new_reachable_states,
                    new_possible_states,
                    condition_cache_updates,
                    action_cache_update,
                ) = self
                    .get_reachable_states_from_base_state(base_state_hash, base_state_probability);
                for cache_update in condition_cache_updates {
                    condition_cache_updates_tx.send(cache_update).unwrap();
                }
                for cache_update in action_cache_update {
                    action_cache_updates_tx.send(cache_update).unwrap();
                }
                self.possible_states.extend(new_possible_states);
                for (reachable_state_hash, reachable_state_probability) in new_reachable_states {
                    self.append_reachable_state(reachable_state_hash, reachable_state_probability)
                }
            });

        // TODO: Assert that the cache does not yet contain the cache update
        while let Result::Ok(condition_cache_update) = condition_cache_updates_rx.try_recv() {
            let own_rule_cache = self
                .cache
                .rules
                .get_mut(&condition_cache_update.rule_name)
                .expect("Rule {rule_name} not found in self.cache");
            own_rule_cache.condition.insert(
                condition_cache_update.base_state_hash,
                condition_cache_update.result,
            );
        }

        while let Result::Ok(action_cache_update) = action_cache_updates_rx.try_recv() {
            let own_rule_cache = self
                .cache
                .rules
                .get_mut(&action_cache_update.rule_name)
                .expect("Rule {rule_name} not found in self.cache");
            own_rule_cache.actions.insert(
                action_cache_update.base_state_hash,
                action_cache_update.new_state_hash,
            );
        }

        // TODO: Improve this
        let probability_sum = self.reachable_states.par_values().sum();
        if !(0.9999999 < probability_sum && probability_sum < 1.0000001) {
            panic!("Probability sum {:?} is not 1", probability_sum);
        }
    }

    /// Gets the entropy of the current probability distribution.
    fn get_entropy(&self) -> f64 {
        let entropy = self
            .reachable_states
            .par_iter()
            .map(|(_, probability)| {
                if *probability > 0. {
                    probability * -probability.log2()
                } else {
                    0.
                }
            })
            .sum();
        entropy
    }

    ///Gets a graph from the possible states with the nodes being the states and the directed edges being the rule names.
    pub fn get_graph_from_cache(&self) -> Graph<State, String> {
        let mut graph = Graph::<State, String>::new();
        let mut nodes: HashMap<u64, NodeIndex> = HashMap::new();
        for (state_hash, state) in &self.possible_states {
            let node_index = graph.add_node(state.clone());
            nodes.insert(*state_hash, node_index);
        }
        for (state_hash, state_node) in &nodes {
            for (rule_name, rule_cache) in &self.cache.rules {
                if rule_cache.condition.get(state_hash).is_some() {
                    if let Some(new_state_hash) = rule_cache.actions.get(state_hash) {
                        let new_state_node = nodes.get(new_state_hash).unwrap();
                        graph.add_edge(*state_node, *new_state_node, rule_name.clone());
                    }
                }
            }
        }
        graph
    }

    /// Checks if the uniform distribution is a steady state i.e. if the transition rate matrix is doubly statistical.
    pub fn is_doubly_statistical(&self) -> bool {
        let mut simulation = Simulation::new(
            self.resources.clone(),
            self.initial_state.clone(),
            self.rules.clone(),
        );
        let mut current_reachable_states = simulation.reachable_states.clone();
        while current_reachable_states.len() != self.reachable_states.len()
            && current_reachable_states
                .keys()
                .all(|k| self.reachable_states.contains_key(k))
        {
            current_reachable_states = simulation.reachable_states.clone();
            simulation.next_step();
        }
        let uniform_probability = 1. / simulation.possible_states.len() as f64;
        let uniform_distribution: HashMap<u64, f64> =
            HashMap::from_iter(simulation.possible_states.iter().map(|(state_hash, _)| {
                let prob: (u64, f64) = (*state_hash, uniform_probability);
                prob
            }));
        let mut uniform_simulation = simulation.clone();
        uniform_simulation.reachable_states = uniform_distribution;
        let uniform_entropy = uniform_simulation.get_entropy();
        uniform_simulation.next_step();
        let uniform_entropy_after_step = uniform_simulation.get_entropy();
        uniform_entropy == uniform_entropy_after_step
    }
}
