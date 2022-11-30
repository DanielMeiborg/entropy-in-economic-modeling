use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::mpsc::{self, Sender};

use hashbrown::HashMap;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use rayon::prelude::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Entity {
    pub resources: HashMap<String, f64>,
}

impl Entity {
    pub fn get_resource(&self, resource_name: &String) -> f64 {
        *self.resources.get(resource_name).unwrap()
    }
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

impl Data {
    pub fn get_hash(&self) -> u64 {
        let mut hasher = &mut DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct State {
    pub data: Data,
    pub probability: f64,
}

impl State {
    pub fn get_entity(&self, entity_name: &String) -> Entity {
        self.data.entities.get(entity_name).unwrap().clone()
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
    pub initial_state: Box<State>,
    pub possible_states: HashMap<u64, Box<State>>,
    pub rules: HashMap<String, Box<Rule>>,
    pub time: u64,
    pub entropy: f64,
    cache: Cache,
}

impl Simulation {
    pub fn new(
        resources: HashMap<String, Resource>,
        data: Data,
        rules: HashMap<String, Box<Rule>>,
    ) -> Simulation {
        let initial_state_hash = data.get_hash();
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
            possible_states: HashMap::from([(initial_state_hash, initial_state)]),
            rules,
            time: 0,
            entropy: 0.,
            cache: Cache { rules: rule_caches },
        }
    }

    pub fn next_step(&mut self) {
        self.possible_states = self.get_next_possible_states();
        self.entropy = self.get_entropy();
        self.time += 1;
    }

    fn append_possible_state(
        new_state: Box<State>,
        next_possible_states: &mut HashMap<u64, Box<State>>,
    ) {
        let hash = new_state.data.get_hash();
        match next_possible_states.get_mut(&hash) {
            Some(state) => {
                state.probability += new_state.probability;
            }
            None => {
                next_possible_states.insert(hash, new_state);
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
        &self,
        cache_tx: &Sender<Cache>,
        rule_name: &String,
        rule: &Rule,
        state_hash: &u64,
        state: &State,
    ) -> bool {
        let rule_cache = self.cache.rules.get(rule_name).unwrap();
        match rule_cache.condition.get(state_hash) {
            Some(result) => *result,
            None => {
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

    fn get_new_state(
        &self,
        cache_tx: &Sender<Cache>,
        state_hash: &u64,
        state: &State,
        rule_name: &String,
        rule: &Rule,
    ) -> Box<State> {
        let rule_cache = self.cache.rules.get(rule_name).unwrap();

        if let Some(state_hash) = rule_cache.actions.get(state_hash) {
            if let Some(new_state) = self.possible_states.get(state_hash) {
                return Box::new(State {
                    data: new_state.data.clone(),
                    probability: rule.probability_weight,
                });
            }
        }

        let mut new_state = State {
            data: state.data.clone(),
            probability: rule.probability_weight,
        };

        let actions = (rule.actions)(state);
        for action in actions {
            new_state
                .data
                .entities
                .get_mut(&action.entity)
                .unwrap_or_else(|| panic!("Entity {} does not exist", action.entity))
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

        self.check_resources(&new_state);

        let new_state_hash = new_state.data.get_hash();
        cache_tx
            .send(Cache {
                rules: HashMap::from([(
                    rule_name.clone(),
                    RuleCache {
                        condition: HashMap::new(),
                        actions: HashMap::from([(*state_hash, new_state_hash)]),
                    },
                )]),
            })
            .unwrap();

        Box::new(new_state)
    }

    fn apply_rules_to_state(
        &self,
        cache_tx: Sender<Cache>,
        state_hash: &u64,
        state: &State,
    ) -> (f64, f64, HashMap<u64, Box<State>>) {
        let mut new_base_state_probability = state.probability;
        let mut applying_rules_probability_weight_sum = 0.;
        let mut current_possible_states: HashMap<u64, Box<State>> = HashMap::new();

        for (rule_name, rule) in &self.rules {
            let rule_applies =
                self.check_rule_applies(&cache_tx, rule_name, rule, state_hash, state);
            if rule_applies && rule.probability_weight > 0. {
                new_base_state_probability *= 1. - rule.probability_weight;
                applying_rules_probability_weight_sum += rule.probability_weight;
                let new_state = self.get_new_state(&cache_tx, state_hash, state, rule_name, rule);
                Simulation::append_possible_state(new_state, &mut current_possible_states);
            }
        }

        if new_base_state_probability > 0. {
            let mut new_base_state = state.clone();
            new_base_state.probability = new_base_state_probability;
            Simulation::append_possible_state(
                Box::new(new_base_state),
                &mut current_possible_states,
            );
        }

        (
            new_base_state_probability,
            applying_rules_probability_weight_sum,
            current_possible_states,
        )
    }

    fn set_probabilities_for_current_possible_states(
        current_possible_states: &mut HashMap<u64, Box<State>>,
        state_hash: &u64,
        state: &State,
        new_base_state_probability: f64,
        applying_rules_probability_weight_sum: f64,
    ) {
        current_possible_states.par_iter_mut().for_each(
            |(new_possible_state_hash, new_possible_state)| {
                if new_possible_state_hash != state_hash {
                    new_possible_state.probability *= (state.probability
                        - new_base_state_probability)
                        / applying_rules_probability_weight_sum;
                }
            },
        );
    }

    // TODO: Implement intervention
    fn get_next_possible_states(&mut self) -> HashMap<u64, Box<State>> {
        let (cache_tx, cache_rx) = mpsc::channel();

        let all_current_possible_states: Vec<HashMap<u64, Box<State>>> = self
            .possible_states
            .par_iter()
            .map_with(cache_tx, |cache_tx, (state_hash, state)| {
                let (
                    new_base_state_probability,
                    applying_rules_probability_weight_sum,
                    mut current_possible_states,
                ) = self.apply_rules_to_state(cache_tx.clone(), state_hash, state);

                Simulation::set_probabilities_for_current_possible_states(
                    &mut current_possible_states,
                    state_hash,
                    state,
                    new_base_state_probability,
                    applying_rules_probability_weight_sum,
                );

                current_possible_states
            })
            .collect();

        let mut next_possible_states: HashMap<u64, Box<State>> = HashMap::new();
        all_current_possible_states
            .iter()
            .for_each(|current_possible_states| {
                current_possible_states.iter().for_each(|(_, state)| {
                    Simulation::append_possible_state(state.clone(), &mut next_possible_states);
                })
            });

        for cache in cache_rx {
            for (rule_name, rule_cache) in cache.rules {
                let own_rule_cache = self.cache.rules.get_mut(&rule_name).unwrap();
                own_rule_cache.condition.extend(rule_cache.condition);
                own_rule_cache.actions.extend(rule_cache.actions);
            }
        }

        let probability_sum = next_possible_states
            .values()
            .fold(0., |sum, state| sum + state.probability);

        if !(0.9999999 < probability_sum && probability_sum < 1.0000001) {
            panic!("Probability sum {:?} is not 1", probability_sum);
        }

        next_possible_states
    }

    fn get_entropy(&self) -> f64 {
        let entropy = self
            .possible_states
            .par_values()
            .map(|state| state.probability * -state.probability.log2())
            .sum();
        entropy
    }

    // TODO: sometimes there are states which are possible once but not later. Do not use possible_states but create a register for states, so rewrite everything
    // Note: This was not thought possible, but a base_state_probability of 0 does not imply that the simulation is not doubly statistical
    pub fn get_graph_from_cache(&self) -> Graph<Box<State>, String> {
        let mut graph = Graph::<Box<State>, String>::new();
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

    pub fn insert_probability_distribution(
        &mut self,
        probability_distribution: &HashMap<u64, f64>,
    ) {
        for (state_hash, probability) in probability_distribution {
            let state = self.possible_states.get_mut(state_hash).unwrap();
            state.probability = *probability;
        }
    }

    pub fn is_doubly_statistical(&self) -> bool {
        let mut simulation = Simulation::new(
            self.resources.clone(),
            self.initial_state.data.clone(),
            self.rules.clone(),
        );
        // TODO: this does not work for pure deterministic simulations. use something like hashset
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
        uniform_simulation.insert_probability_distribution(&uniform_distribution);
        let uniform_entropy = uniform_simulation.get_entropy();
        uniform_simulation.next_step();
        let uniform_entropy_after_step = uniform_simulation.get_entropy();
        println!(
            "Uniform entropy: {} Uniform entropy after step: {}",
            uniform_entropy, uniform_entropy_after_step
        );
        uniform_entropy == uniform_entropy_after_step
    }

    // while one could implement this function, an easier approach for validationg the doubly statistical property is by inserting
    // the uniform distribution and look at the change or not-change
    // pub fn get_transition_rate_matrix(&mut self) -> HashMap<u64, HashMap<u64, f64>> {
    //     let mut transition_rate_matrix: HashMap<u64, HashMap<u64, f64>> = HashMap::new();
    //     let graph = self.get_graph_from_cache();
    //     for node_index in graph.node_indices() {
    //         let mut row: HashMap<u64, f64> = HashMap::new();
    //         for outgoing_node_index in
    //             graph.neighbors_directed(node_index, petgraph::Direction::Outgoing)
    //         {
    //             let outgoing_state_hash = graph[outgoing_node_index].data.get_hash();
    //             let transition_rate: f64 = {
    //                 let current_state_hash = graph[node_index].data.get_hash();
    //                 let rule_to_outgoing_state = self
    //                     .cache
    //                     .rules
    //                     .par_iter()
    //                     .find_any(|(_, rule_cache)| {
    //                         if rule_cache.condition.get(&current_state_hash).is_some() {
    //                             let new_state_hash =
    //                                 rule_cache.actions.get(&current_state_hash).unwrap();
    //                             new_state_hash == &outgoing_state_hash
    //                         } else {
    //                             false
    //                         }
    //                     })
    //                     .unwrap()
    //                     .0;
    //                 let applying_rules_probability_weight_sum: f64 = graph
    //                     .neighbors_directed(node_index, petgraph::Direction::Outgoing)
    //                     .map(|neighbor_node_index| {
    //                         let connecting_edge =
    //                             graph.find_edge(node_index, neighbor_node_index).unwrap();
    //                         let connecting_rule_name = &graph[connecting_edge];
    //                         let connecting_rule = self.rules.get(connecting_rule_name).unwrap();

    //                         connecting_rule.probability_weight
    //                     })
    //                     .sum();
    //                 let rule_to_outgoing_state_weight = self
    //                     .rules
    //                     .get(rule_to_outgoing_state)
    //                     .unwrap()
    //                     .probability_weight;

    //                 // There's still stuff to do
    //             };
    //             row.insert(outgoing_state_hash, transition_rate);
    //         }
    //         let impossible_node_indices = graph
    //             .node_indices()
    //             .filter(|node_index| !row.contains_key(&graph[*node_index].data.get_hash()))
    //             .collect::<Vec<NodeIndex>>();
    //         for impossible_node_index in impossible_node_indices {
    //             row.insert(graph[impossible_node_index].data.get_hash(), 0.);
    //         }
    //         transition_rate_matrix.insert(graph[node_index].data.get_hash(), row);
    //     }
    //     {
    //         let transition_rate_matrix_len = transition_rate_matrix.len();
    //         transition_rate_matrix.par_iter().for_each(|(_, row)| {
    //             assert_eq!(row.len(), transition_rate_matrix_len);
    //         });
    //     }
    //     transition_rate_matrix
    // }
}
