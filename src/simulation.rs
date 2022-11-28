use hashbrown::HashMap;
use rayon::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// TODO: construct an actual graph from the states and the connecting rules

#[derive(PartialEq, Debug, Clone)]
pub struct Entity {
    pub resources: HashMap<String, f64>,
}

#[derive(Clone, Debug)]
pub struct Data {
    pub entities: HashMap<String, Entity>,
}

impl Hash for Data {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (name, entity) in &self.entities {
            for (resource_name, amount) in &entity.resources {
                (name, resource_name, amount.to_bits()).hash(state);
            }
        }
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        let self_hasher = &mut DefaultHasher::new();
        self.hash(self_hasher);
        let other_hasher = &mut DefaultHasher::new();
        other.hash(other_hasher);
        self_hasher.finish() == other_hasher.finish()
    }
}

impl Eq for Data {}

#[derive(PartialEq, Clone, Debug)]
pub struct State {
    pub data: Data,
    pub probability: f64,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Action {
    pub name: String,
    pub resource: String,
    pub entities: Vec<String>,
    pub new_amount: f64,
}

#[derive(Clone)]
pub struct Rule {
    pub description: String,
    pub condition: fn(&State) -> bool,
    pub probability: f64,
    pub actions: fn(&State) -> Vec<Action>,
}

#[derive(PartialEq, Clone, Debug)]
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

// TODO: why box?
#[derive(Clone)]
pub struct Simulation {
    pub resources: HashMap<String, Resource>,
    pub initial_state: Box<State>,
    pub reachable_states: HashMap<u64, Box<State>>,
    pub rules: HashMap<String, Box<Rule>>,
    pub current_time: u64,
    pub entropy: f64,
    cache: Cache,
}

impl Simulation {
    // TODO: Implement checks for input parameters
    pub fn new(
        resources: HashMap<String, Resource>,
        data: Data,
        rules: HashMap<String, Box<Rule>>,
    ) -> Simulation {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        let initial_state = Box::new(State {
            data,
            probability: 1.0,
        });

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
            reachable_states: HashMap::from([(hasher.finish(), initial_state)]),
            rules,
            current_time: 0,
            entropy: 0.,
            cache: Cache { rules: rule_caches },
        }
    }

    pub fn next_step(&mut self) {
        self.reachable_states = self.get_next_reachable_states();
        self.entropy = self.get_entropy();
        self.current_time += 1;
    }

    fn append_reachable_state(
        new_state: Box<State>,
        next_reachable_states: &mut HashMap<u64, Box<State>>,
    ) {
        let mut hasher = DefaultHasher::new();
        new_state.data.hash(&mut hasher);
        let hash = hasher.finish();
        match next_reachable_states.get_mut(&hash) {
            Some(state) => {
                state.probability += new_state.probability;
            }
            None => {
                next_reachable_states.insert(hash, new_state);
            }
        }
    }

    fn check_resources(&self, new_state: &State) {
        for (resource_name, resource) in &self.resources {
            match &resource.capacity {
                Capacity::Limited(limit) => {
                    let mut total_amount: f64 = 0.;
                    for (entity_name, entity) in &new_state.data.entities {
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
                    for (entity_name, entity) in &new_state.data.entities {
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

    fn check_rule_applies(
        cache: &mut Cache,
        rule_name: &String,
        rule: &Rule,
        state_hash: &u64,
        state: &State,
    ) -> bool {
        let rule_cache: &mut RuleCache = cache.rules.get_mut(rule_name).unwrap();
        match rule_cache.condition.get(state_hash) {
            Some(result) => *result,
            None => {
                let result = (rule.condition)(state);
                rule_cache.condition.insert(*state_hash, result);
                result
            }
        }
    }

    fn get_new_state(
        &self,
        cache: &mut Cache,
        state_hash: &u64,
        state: &State,
        rule_name: &String,
        rule: &Rule,
    ) -> Box<State> {
        let rule_cache: &mut RuleCache = cache.rules.get_mut(rule_name).unwrap();

        if let Some(state_hash) = rule_cache.actions.get(state_hash) {
            if let Some(new_state) = self.reachable_states.get(state_hash) {
                return Box::new(State {
                    data: new_state.data.clone(),
                    probability: rule.probability,
                });
            }
        }

        let mut new_state = State {
            data: state.data.clone(),
            probability: rule.probability,
        };

        let actions = (rule.actions)(state);
        for action in actions {
            for entity_name in action.entities {
                new_state
                    .data
                    .entities
                    .get_mut(&entity_name)
                    .unwrap()
                    .resources
                    .insert(action.resource.clone(), action.new_amount);

                let capacity_per_entity = &self
                    .resources
                    .get(&action.resource)
                    .unwrap()
                    .capacity_per_entity;

                match capacity_per_entity {
                    Capacity::Limited(limit) => {
                        if action.new_amount > *limit {
                            panic!(
                                "Resource limit per entity exceeded for resource {resource_name}",
                                resource_name = action.resource
                            );
                        }
                    }
                    Capacity::Unlimited => {}
                }
            }
        }

        self.check_resources(&new_state);

        let mut hasher = DefaultHasher::new();
        new_state.data.hash(&mut hasher);
        rule_cache.actions.insert(*state_hash, hasher.finish());

        Box::new(new_state)
    }

    fn apply_rules_to_state(
        &self,
        new_cache: &mut Cache,
        state_hash: &u64,
        state: &State,
    ) -> (f64, f64, HashMap<u64, Box<State>>) {
        let mut new_base_state_probability = state.probability;
        let mut applying_rules_probability_sum = 0.;
        let mut current_reachable_states: HashMap<u64, Box<State>> = HashMap::new();

        for (rule_name, rule) in &self.rules {
            let rule_applies =
                Simulation::check_rule_applies(new_cache, rule_name, rule, state_hash, state);
            if rule_applies && rule.probability > 0. {
                new_base_state_probability *= 1. - rule.probability;
                applying_rules_probability_sum += rule.probability;
                let new_state = self.get_new_state(new_cache, state_hash, state, rule_name, rule);
                Simulation::append_reachable_state(new_state, &mut current_reachable_states);
            }
        }

        if new_base_state_probability > 0. {
            let mut new_base_state = state.clone();
            new_base_state.probability = new_base_state_probability;
            Simulation::append_reachable_state(
                Box::new(new_base_state),
                &mut current_reachable_states,
            );
        }

        (
            new_base_state_probability,
            applying_rules_probability_sum,
            current_reachable_states,
        )
    }

    fn set_probabilities_for_current_reachable_states(
        current_reachable_states: &mut HashMap<u64, Box<State>>,
        state_hash: &u64,
        state: &State,
        new_base_state_probability: f64,
        applying_rules_probability_sum: f64,
    ) {
        current_reachable_states.par_iter_mut().for_each(
            |(new_reachable_state_hash, new_reachable_state)| {
                if new_reachable_state_hash != state_hash {
                    new_reachable_state.probability *= (state.probability
                        - new_base_state_probability)
                        / applying_rules_probability_sum;
                }
            },
        );
    }

    // TODO: Multithreading
    // TODO: The reverse rules for the doubly statistical property
    fn get_next_reachable_states(&mut self) -> HashMap<u64, Box<State>> {
        let mut next_reachable_states: HashMap<u64, Box<State>> = HashMap::new();
        let mut new_cache = self.cache.clone();
        for (state_hash, state) in &self.reachable_states {
            let (
                new_base_state_probability,
                applying_rules_probability_sum,
                mut current_reachable_states,
            ) = self.apply_rules_to_state(&mut new_cache, state_hash, state);

            Simulation::set_probabilities_for_current_reachable_states(
                &mut current_reachable_states,
                state_hash,
                state,
                new_base_state_probability,
                applying_rules_probability_sum,
            );

            current_reachable_states.iter().for_each(|(_, new_state)| {
                Simulation::append_reachable_state(new_state.clone(), &mut next_reachable_states)
            });
        }
        let probability_sum = next_reachable_states
            .values()
            .fold(0., |sum, state| sum + state.probability);

        if !(0.9999999 < probability_sum && probability_sum < 1.0000001) {
            panic!("Probability sum {:?} is not 1", probability_sum);
        }

        self.cache = new_cache;
        next_reachable_states
    }

    fn get_entropy(&self) -> f64 {
        let entropy = self
            .reachable_states
            .par_values()
            .map(|state| state.probability * -state.probability.log2())
            .sum();
        entropy
    }
}
