use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::mpsc::{self, Sender};

#[allow(unused_imports)]
use hashbrown::{HashMap, HashSet};

use petgraph::graph::NodeIndex;
use petgraph::Graph;
use rayon::prelude::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Entity {
    pub resources: HashMap<String, f64>,
}

impl Entity {
    #[allow(dead_code)]
    pub fn get_resource(&self, resource_name: &String) -> f64 {
        *self.resources.get(resource_name).unwrap()
    }
}

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
        self.entities.get(entity_name).unwrap().clone()
    }

    pub fn get_hash(&self) -> u64 {
        let mut hasher = &mut DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Action {
    pub name: String,
    pub resource: String,
    pub entity: String,
    pub new_amount: f64,
}

#[derive(Clone)]
pub struct Rule {
    pub description: String,
    pub condition: fn(&State) -> bool,
    pub probability_weight: f64,
    pub actions: fn(&State) -> Vec<Action>,
}

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum Capacity {
    Limited(f64),
    Unlimited,
}

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

#[derive(Clone)]
pub struct Simulation {
    pub resources: HashMap<String, Resource>,
    pub initial_state: State,
    pub possible_states: HashMap<u64, State>,
    pub reachable_states: HashMap<u64, f64>,
    pub rules: HashMap<String, Rule>,
    pub time: u64,
    pub entropy: f64,
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
    fn check_resources(&self, new_state: &State) {
        for (resource_name, resource) in &self.resources {
            match &resource.capacity {
                Capacity::Limited(limit) => {
                    let mut total_amount: f64 = 0.;
                    for (entity_name, entity) in &new_state.entities {
                        let entity_amount = entity.resources.get(resource_name).unwrap();
                        if entity_amount < &0. {
                            panic!(
                                "Entity {} has negative amount of resource {}",
                                entity_name, resource_name
                            );
                        }
                        total_amount += entity.resources.get(resource_name).unwrap();
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
                        let entity_amount = entity.resources.get(resource_name).unwrap();
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
        cache_tx: &Sender<Cache>,
        rule_name: &String,
        state_hash: &u64,
    ) -> bool {
        let rule_cache = self.cache.rules.get(rule_name).unwrap();
        let rule = self.rules.get(rule_name).unwrap();
        if rule.probability_weight == 0. {
            return false;
        }
        match rule_cache.condition.get(state_hash) {
            Some(result) => *result,
            None => {
                let state = self.possible_states.get(state_hash).unwrap();
                let result = (rule.condition)(state);
                cache_tx
                    .send(Cache {
                        rules: HashMap::from([(
                            rule_name.clone(),
                            RuleCache {
                                condition: HashMap::from([(*state_hash, result)]),
                                actions: HashMap::new(),
                            },
                        )]),
                    })
                    .unwrap();
                result
            }
        }
    }

    /// Gets the state the given rule results in from the given state using or updating the cache respectively.
    fn get_new_state(
        &self,
        cache_tx: &Sender<Cache>,
        base_state_hash: &u64,
        rule_name: &String,
    ) -> State {
        let rule_cache = self.cache.rules.get(rule_name).unwrap();

        if let Some(state_hash) = rule_cache.actions.get(base_state_hash) {
            if let Some(new_state) = self.possible_states.get(state_hash) {
                return new_state.clone();
            }
        }

        let rule = self.rules.get(rule_name).unwrap();
        let base_state = self.possible_states.get(base_state_hash).unwrap();
        let actions = (rule.actions)(base_state);

        let mut new_state = self.possible_states.get(base_state_hash).unwrap().clone();
        for action in actions {
            new_state
                .entities
                .get_mut(&action.entity)
                .unwrap()
                .resources
                .insert(action.resource.clone(), action.new_amount);

            let capacity_per_entity = &self
                .resources
                .get(&action.resource)
                .unwrap()
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

        self.check_resources(&new_state);

        let new_state_hash = new_state.get_hash();
        cache_tx
            .send(Cache {
                rules: HashMap::from([(
                    rule_name.clone(),
                    RuleCache {
                        condition: HashMap::new(),
                        actions: HashMap::from([(*base_state_hash, new_state_hash)]),
                    },
                )]),
            })
            .unwrap();

        new_state
    }

    // Add all reachable states from the base state to reachable_states and possible_states while using or updating the cache respectively.
    fn add_reachable_states_from_base_state(
        &mut self,
        cache_tx: Sender<Cache>,
        base_state_hash: &u64,
        base_state_probability: &f64,
    ) {
        let mut new_base_state_probability = *base_state_probability;
        let mut applying_rules_probability_weight_sum = 0.;
        let mut reachable_states_from_base_state_by_rule: HashMap<u64, String> = HashMap::new();

        for (rule_name, rule) in &self.rules {
            let rule_applies = self.check_if_rule_applies(&cache_tx, rule_name, base_state_hash);
            if rule_applies {
                new_base_state_probability *= 1. - rule.probability_weight;
                applying_rules_probability_weight_sum += rule.probability_weight;
                let new_state = self.get_new_state(&cache_tx, base_state_hash, rule_name);
                let new_state_hash = new_state.get_hash();
                self.possible_states
                    .entry(new_state_hash)
                    .or_insert(new_state);
                reachable_states_from_base_state_by_rule.insert(new_state_hash, rule_name.clone());
            }
        }

        if new_base_state_probability > 0. {
            self.append_reachable_state(*base_state_hash, new_base_state_probability);
        }
        let probabilities_for_reachable_states_from_base_state = self
            .get_probabilities_for_reachable_states_from_base_state(
                reachable_states_from_base_state_by_rule,
                base_state_hash,
                *base_state_probability,
                new_base_state_probability,
                applying_rules_probability_weight_sum,
            );
        probabilities_for_reachable_states_from_base_state
            .iter()
            .for_each(|(new_state_hash, new_state_probability)| {
                self.append_reachable_state(*new_state_hash, *new_state_probability);
            })
    }

    // TODO: Documentation
    fn get_probabilities_for_reachable_states_from_base_state(
        &self,
        reachable_states_from_base_state_by_rule: HashMap<u64, String>,
        base_state_hash: &u64,
        old_base_state_probability: f64,
        new_base_state_probability: f64,
        applying_rules_probability_weight_sum: f64,
    ) -> HashMap<u64, f64> {
        HashMap::from_par_iter(
            reachable_states_from_base_state_by_rule
                .par_iter()
                .filter_map(|(new_reachable_state_hash, rule)| {
                    if new_reachable_state_hash != base_state_hash {
                        let rule_probability_weight =
                            self.rules.get(rule).unwrap().probability_weight;
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
        let (cache_tx, cache_rx) = mpsc::channel();

        let old_reachable_states = self.reachable_states.clone();
        self.reachable_states.clear();
        old_reachable_states
            .iter()
            .for_each(|(base_state_hash, base_state_probability)| {
                self.add_reachable_states_from_base_state(
                    cache_tx.clone(),
                    base_state_hash,
                    base_state_probability,
                );
            });

        while let Result::Ok(cache) = cache_rx.try_recv() {
            for (rule_name, rule_cache) in cache.rules {
                let own_rule_cache = self.cache.rules.get_mut(&rule_name).unwrap();
                own_rule_cache.condition.extend(rule_cache.condition);
                own_rule_cache.actions.extend(rule_cache.actions);
            }
        }

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

    // TODO: Rewrite this
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

    // TODO: See github issue #3
    pub fn is_doubly_statistical(&self) -> bool {
        let mut simulation = Simulation::new(
            self.resources.clone(),
            self.initial_state.clone(),
            self.rules.clone(),
        );
        let mut current_number_of_possible_states = 0;
        while simulation.possible_states.len() != current_number_of_possible_states {
            current_number_of_possible_states = simulation.possible_states.len();
            simulation.next_step();
            println!(
                "Time: {} Number of possible states: {}",
                simulation.time,
                simulation.possible_states.len()
            );
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
        println!(
            "Uniform entropy: {} Uniform entropy after step: {}",
            uniform_entropy, uniform_entropy_after_step
        );
        uniform_entropy == uniform_entropy_after_step
    }
}
