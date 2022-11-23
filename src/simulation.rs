use std::cmp::Ordering;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul, Sub};
use rayon::prelude::*;

// TODO: construct an actual graph from the states and the connecting rules

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum Amount {
    Integer(u64),
    Float(f64),
}

impl Eq for Amount {}

impl Ord for Amount {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Amount::Integer(a), Amount::Integer(b)) => a.cmp(b),
            (Amount::Float(a), Amount::Float(b)) => a.partial_cmp(b).unwrap(),
            (Amount::Integer(a), Amount::Float(b)) => (*a as f64).partial_cmp(b).unwrap(),
            (Amount::Float(a), Amount::Integer(b)) => a.partial_cmp(&(*b as f64)).unwrap(),
        }
    }
}

impl Hash for Amount {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Amount::Integer(x) => x.hash(state),
            Amount::Float(x) => x.to_bits().hash(state),
        }
    }
}

impl Add for Amount {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Amount::Integer(a), Amount::Integer(b)) => Amount::Integer(a + b),
            (Amount::Float(a), Amount::Float(b)) => Amount::Float(a + b),
            (Amount::Integer(_), Amount::Float(_)) => panic!("Cannot add integer to float"),
            (Amount::Float(_), Amount::Integer(_)) => panic!("Cannot add float to integer"),
        }
    }
}

impl Mul for Amount {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Amount::Integer(a), Amount::Integer(b)) => Amount::Integer(a * b),
            (Amount::Float(a), Amount::Float(b)) => Amount::Float(a * b),
            (Amount::Integer(_), Amount::Float(_)) => panic!("Cannot multiply integer by float"),
            (Amount::Float(_), Amount::Integer(_)) => panic!("Cannot multiply float by integer"),
        }
    }
}

impl Sub for Amount {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Amount::Integer(a), Amount::Integer(b)) => Amount::Integer(a - b),
            (Amount::Float(a), Amount::Float(b)) => Amount::Float(a - b),
            (Amount::Integer(_), Amount::Float(_)) => panic!("Cannot subtract integer from float"),
            (Amount::Float(_), Amount::Integer(_)) => panic!("Cannot subtract float from integer"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Entity {
    pub name: String,
    pub resources: HashMap<String, Amount>,
}

impl Hash for Entity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        for pair in &self.resources {
            pair.hash(state);
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct State {
    pub entities: Vec<Entity>,
    pub hash: u64,
    pub probability: f64,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Action {
    pub name: String,
    pub resource: String,
    pub entities: Vec<String>,
    pub new_amount: Amount,
}

#[derive(Clone)]
pub struct Rule {
    pub name: String,
    pub description: String,
    pub condition: fn(&State) -> bool,
    pub probability: f64,
    pub actions: fn(&State) -> Vec<Action>,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum UnlimitedCapacity {
    Integer,
    Float,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum Capacity {
    Limited(Amount),
    Unlimited(UnlimitedCapacity),
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum CapacityPerEntity {
    Limited(Amount),
    Unlimited,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Resource {
    pub name: String,
    pub description: String,
    pub capacity: Capacity,
    pub capacity_per_entity: CapacityPerEntity,
}

#[derive(Clone)]
pub struct Simulation {
    pub resources: Vec<Resource>,
    pub initial_state: Box<State>,
    pub reachable_states: Vec<Box<State>>,
    pub rules: Vec<Box<Rule>>,
    pub current_time: u64,
    pub entropy: f64,
}

impl Simulation {
    // TODO: Implement checks for input parameters
    pub fn new(
        resources: Vec<Resource>,
        entities: Vec<Entity>,
        rules: Vec<Box<Rule>>,
    ) -> Simulation {
        let mut hasher = DefaultHasher::new();
        entities.hash(&mut hasher);
        let initial_state = Box::new(State {
            entities,
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
        new_state.entities.hash(&mut hasher);
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
        for resource in &self.resources {
            match &resource.capacity {
                Capacity::Limited(amount) => match amount {
                    Amount::Integer(limit) => {
                        let mut total_amount: u64 = 0;
                        for entity in &new_state.entities {
                            let entity_amount =
                                entity.resources.get(&resource.name).unwrap();
                            match entity_amount {
                                Amount::Integer(amount) => {
                                    total_amount += amount;
                                }
                                Amount::Float(_) => {
                                    panic!("Cannot combine Float amount with Integer resource in entity");
                                }
                            }
                            if total_amount > *limit {
                                panic!("Resource limit exceeded for resource {resource_name}",
                                 resource_name = resource.name);
                            }
                        }
                    }
                    Amount::Float(limit) => {
                        let mut total_amount: f64 = 0.;
                        for entity in &new_state.entities {
                            let entity_amount =
                                entity.resources.get(&resource.name).unwrap();
                            match entity_amount {
                                Amount::Integer(_) => {
                                    panic!("Cannot combine Integer amount with Float resource in entity");
                                }
                                Amount::Float(amount) => {
                                    total_amount += amount;
                                }
                            }
                            if total_amount > *limit {
                                panic!("Resource limit exceeded for resource {resource_name}",
                                 resource_name = resource.name);
                            }
                        }
                    }
                },
                Capacity::Unlimited(_) => continue,
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
                            let mut entity = new_state
                                .entities
                                .par_iter()
                                .find_any(|x| x.name == entity_name)
                                .unwrap()
                                .clone();

                            let capacity_per_entity = &self
                                .resources
                                .iter()
                                .find(|&x| x.name == action.resource)
                                .unwrap()
                                .capacity_per_entity;

                            // TODO: implement maximum capacity per entity
                            entity
                                .resources
                                .insert(action.resource.clone(), action.new_amount.clone());
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
