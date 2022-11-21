use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub struct Entity {
    pub name: String,
    pub resources: HashMap<String, u64>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Resource {
    pub name: String,
    pub description: String,
    pub capacity: u64,
    pub must_be_held: bool,
    pub per_entity: bool,
    pub max_held: i64,
}

#[derive(PartialEq, Clone, Debug)]
pub struct State {
    pub entities: Vec<Entity>,
    pub probability: f64,
}

#[derive(Clone, Debug)]
pub struct Action {
    pub resource: String,
    pub entities: Vec<String>,
    pub amount: i64,
}

#[derive(Clone)]
pub struct Rule {
    pub name: String,
    pub description: String,
    pub condition: fn(&State) -> bool,
    pub probability: f64,
    pub actions: fn(&State) -> Vec<Action>,
}

#[derive(Clone)]
pub struct Simulation {
    pub resources: Vec<Resource>,
    pub initial_state: Box<State>,
    pub possible_states: Vec<Box<State>>,
    pub rules: Vec<Box<Rule>>,
    pub current_time: u64,
    pub entropy: f64,
}

impl Simulation {
    // TODO: Implement checks for input parameters
    pub fn new(
        resources: Vec<Resource>,
        initial_state: Box<State>,
        rules: Vec<Box<Rule>>,
    ) -> Simulation {
        Simulation {
            resources: resources.clone(),
            initial_state: initial_state.clone(),
            rules: rules.clone(),
            possible_states: vec![initial_state.clone()],
            current_time: 0,
            entropy: Simulation::get_entropy(&vec![initial_state], 0),
        }
    }

    fn get_entropy(possible_states: &Vec<Box<State>>, t: u64) -> f64 {
        let mut entropy: f64 = 0.0;
        for state in possible_states {
            entropy += state.probability * -state.probability.log2();
        }

        return entropy;
    }
}
