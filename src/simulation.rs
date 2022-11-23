use rayon::prelude::*;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

// TODO: construct an actual graph from the states and the connecting rules

#[derive(PartialEq, Debug, Clone)]
pub struct Entity {
    pub resources: HashMap<String, f64>,
}

#[derive(PartialEq, Clone, Debug)]
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

#[derive(PartialEq, Clone, Debug)]
pub struct State {
    pub data: Data,
    pub hash: u64,
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
    pub name: String,
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

#[derive(Clone)]
pub struct Simulation {
    pub resources: HashMap<String, Resource>,
    pub initial_state: Box<State>,
    pub reachable_states: Vec<Box<State>>,
    pub rules: Vec<Box<Rule>>,
    pub current_time: u64,
    pub entropy: f64,
}

impl Simulation {
    // TODO: Implement checks for input parameters
    pub fn new(
        resources: HashMap<String, Resource>,
        data: Data,
        rules: Vec<Box<Rule>>,
    ) -> Simulation {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        let initial_state = Box::new(State {
            data,
            hash: hasher.finish(),
            probability: 1.0,
        });
        Simulation {
            resources: resources.clone(),
            initial_state: initial_state.clone(),
            reachable_states: vec![initial_state.clone()],
            rules: rules.clone(),
            current_time: 0,
            entropy: 0.,
        }
    }

    pub fn next_step(&mut self) {
        self.reachable_states = self.get_next_reachable_states();
        self.entropy = self.get_entropy();
        self.current_time += 1;
    }

    fn append_reachable_state(
        mut new_state: Box<State>,
        next_reachable_states: &mut Vec<Box<State>>,
    ) {
        let mut hasher = DefaultHasher::new();
        new_state.data.hash(&mut hasher);
        new_state.hash = hasher.finish();
        match next_reachable_states
            .par_iter()
            .position_any(|x| x.hash == new_state.hash)
        {
            Some(index) => {
                next_reachable_states[index].probability += new_state.probability;
            }
            None => {
                next_reachable_states.push(new_state);
            }
        }
    }

    fn check_resources(&self, new_state: &Box<State>) {
        for (resource_name, resource) in &self.resources {
            match &resource.capacity {
                Capacity::Limited(limit) => {
                    let mut total_amount: f64 = 0.;
                    for (_, entity) in &new_state.data.entities {
                        total_amount += entity.resources.get(resource_name).unwrap();
                        if total_amount > *limit {
                            panic!(
                                "Resource limit exceeded for resource {resource_name}",
                                resource_name = resource_name
                            );
                        }
                    }
                }
                Capacity::Unlimited => continue,
            }
        }
    }

    // TODO: implement caching
    fn get_next_reachable_states(&self) -> Vec<Box<State>> {
        let mut next_reachable_states: Vec<Box<State>> = Vec::new();
        for state in &self.reachable_states {
            let mut new_p_base_state = state.probability.clone();
            for rule in &self.rules {
                if (rule.condition)(state) && rule.probability > 0. {
                    new_p_base_state = new_p_base_state * (1. - rule.probability);
                    let mut new_state = state.clone();
                    new_state.probability = rule.probability * state.probability;
                    let actions = (rule.actions)(state);
                    for action in actions {
                        for entity_name in action.entities {
                            new_state
                                .data
                                .entities
                                .get_mut(&entity_name)
                                .unwrap()
                                .resources
                                .insert(action.resource.clone(), action.new_amount.clone());

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
                                Capacity::Unlimited => continue,
                            }
                        }
                    }

                    self.check_resources(&new_state);
                    Simulation::append_reachable_state(new_state, &mut next_reachable_states);
                }
            }
            if new_p_base_state > 0. {
                let mut new_base_state = state.clone();
                new_base_state.probability = new_p_base_state;
                Simulation::append_reachable_state(new_base_state, &mut next_reachable_states);
            }
        }

        return next_reachable_states;
    }

    fn get_entropy(&self) -> f64 {
        let mut entropy: f64 = 0.0;
        for state in &self.reachable_states {
            entropy += state.probability * -state.probability.log2();
        }

        return entropy;
    }
}
