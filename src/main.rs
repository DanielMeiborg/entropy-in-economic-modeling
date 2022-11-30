use std::time::SystemTime;

use hashbrown::HashMap;
use petgraph::dot::Dot;
use rayon::prelude::*;

mod simulation;
use simulation::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use ordered_float::NotNan;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Graph;

#[allow(dead_code)]
pub fn get_max_index_f64(v: &Vec<f64>) -> usize {
    v.par_iter()
        .position_any(|x| {
            *x == v
                .par_iter()
                .copied()
                .map(NotNan::new)
                .flatten()
                .max()
                .map(NotNan::into_inner)
                .unwrap()
        })
        .unwrap()
}

#[allow(dead_code)]
pub fn get_min_index_f64(v: &Vec<f64>) -> usize {
    v.par_iter()
        .position_any(|x| {
            *x == v
                .par_iter()
                .copied()
                .map(NotNan::new)
                .flatten()
                .min()
                .map(NotNan::into_inner)
                .unwrap()
        })
        .unwrap()
}

#[allow(dead_code)]
pub fn get_subgraph<N, E>(nodes: Vec<NodeIndex>, graph: &Graph<N, E>) -> Graph<N, E>
where
    N: Clone + std::cmp::PartialEq,
    E: Clone,
{
    let mut sub_graph: Graph<N, E> = Graph::new();
    nodes.iter().for_each(|node| {
        sub_graph.add_node(graph[*node].clone());
    });
    for edge in graph.edge_references() {
        if nodes.contains(&edge.source()) && nodes.contains(&edge.target()) {
            let starting_node_index = sub_graph
                .node_indices()
                .find(|node| sub_graph[*node] == graph[edge.source()])
                .unwrap();
            let ending_node_index = sub_graph
                .node_indices()
                .find(|node| sub_graph[*node] == graph[edge.target()])
                .unwrap();
            sub_graph.add_edge(
                starting_node_index,
                ending_node_index,
                edge.weight().clone(),
            );
        }
    }
    sub_graph
}

pub fn write<P>(path: P, content: String)
where
    P: AsRef<Path>,
{
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

// Playground
fn main() {
    const NUMBER_OF_BINS: i64 = 7;
    let resources = HashMap::from([(
        "point".to_string(),
        Resource {
            description: "a point a bin can have".to_string(),
            capacity: Capacity::Limited(1.),
            capacity_per_entity: Capacity::Limited(1.),
        },
    )]);

    let initial_state = State {
        entities: {
            let mut entities = HashMap::new();
            (1..NUMBER_OF_BINS).for_each(|place: i64| {
                entities.insert(
                    format!("bin {place}"),
                    Entity {
                        resources: HashMap::from([("point".to_string(), 0.)]),
                    },
                );
            });
            entities.insert(
                "bin 0".to_string(),
                Entity {
                    resources: HashMap::from([("point".to_string(), 1.)]),
                },
            );
            entities
        },
    };

    let rules: HashMap<String, Rule> = HashMap::from([
        (
            "yield forward".to_string(),
            Rule {
                description: "a bin yields the point to the bin with the next higher place"
                    .to_string(),
                probability_weight: 1.,
                condition: |_| true,
                actions: |state: &State| {
                    let current_point_owner = state
                        .entities
                        .iter()
                        .find(|(_, entity)| entity.resources["point"] > 0.)
                        .unwrap()
                        .0
                        .clone();
                    let current_point_owner_place: i64 = current_point_owner
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse()
                        .unwrap();
                    let next_point_owner_place: i64 =
                        { current_point_owner_place + 1 }.rem_euclid(NUMBER_OF_BINS);
                    let next_point_owner = format!("bin {}", next_point_owner_place);
                    vec![
                        Action {
                            name: "yield point".to_string(),
                            entity: current_point_owner,
                            resource: "point".to_string(),
                            new_amount: 0.,
                        },
                        Action {
                            name: "get point".to_string(),
                            entity: next_point_owner,
                            resource: "point".to_string(),
                            new_amount: 1.,
                        },
                    ]
                },
            },
        ),
        (
            "yield backward".to_string(),
            Rule {
                description: "a bin yields the point to the bin with the next lower place"
                    .to_string(),
                probability_weight: 1.,
                condition: |_| true,
                actions: |state: &State| {
                    let current_point_owner = state
                        .entities
                        .iter()
                        .find(|(_, entity)| entity.resources["point"] > 0.)
                        .unwrap()
                        .0
                        .clone();
                    let current_point_owner_place: i64 = current_point_owner
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse()
                        .unwrap();
                    let next_point_owner_place: i64 =
                        { current_point_owner_place - 1 }.rem_euclid(NUMBER_OF_BINS);
                    let next_point_owner = format!("bin {}", next_point_owner_place);
                    vec![
                        Action {
                            name: "yield point".to_string(),
                            entity: current_point_owner,
                            resource: "point".to_string(),
                            new_amount: 0.,
                        },
                        Action {
                            name: "get point".to_string(),
                            entity: next_point_owner,
                            resource: "point".to_string(),
                            new_amount: 1.,
                        },
                    ]
                },
            },
        ),
    ]);

    let mut simulation = Simulation::new(resources, initial_state, rules);
    let mut entropies: Vec<f64> = Vec::new();
    let time = SystemTime::now();
    for time in 0..100 {
        entropies.push(simulation.entropy);
        println!(
            "Time: {} Number of reachable states: {}",
            time,
            simulation.reachable_states.len()
        );
        println!(
            "reachable states: {:?}\n",
            simulation
                .reachable_states
                .iter()
                .map(|(state_hash, probability)| (
                    simulation
                        .possible_states
                        .get(state_hash)
                        .unwrap()
                        .entities
                        .iter()
                        .find(|(_, entity)| entity.resources["point"] > 0.)
                        .unwrap()
                        .0
                        .clone(),
                    *probability
                ))
                .collect::<Vec<(String, f64)>>()
        );
        simulation.next_step();
    }
    let duration = time.elapsed().unwrap();
    println!("================================================");
    write("out/entropies.txt", format!("{:?}", &entropies));
    let probability_distribution: Vec<f64> =
        simulation.reachable_states.values().cloned().collect();
    write(
        "out/probability_distribution.txt",
        format!("{:?}", &probability_distribution),
    );
    println!("probability distribution: {:#?}", probability_distribution);

    let highest_probability = simulation
        .reachable_states
        .values()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let most_likely_state = simulation
        .reachable_states
        .iter()
        .find(|(_, probability)| *probability == highest_probability);

    println!("most probable state: {:#?}", most_likely_state);

    println!("The simulation took {} seconds", duration.as_secs_f64());

    let graph = simulation.get_graph_from_cache();
    println!("Nodes in graph: {}", graph.node_count());
    println!("Edges in graph {}", graph.edge_count());
    write(
        "out/graph.dot",
        format!("{:?}", Dot::with_config(&graph, &[])),
    );
    let simulation_is_doubly_statistical = simulation.is_doubly_statistical();
    println!(
        "Simulation is doubly statistical: {}",
        simulation_is_doubly_statistical
    );
}

// let resources = HashMap::from([(
//     "money".to_string(),
//     Resource {
//         description: "In dollars".to_string(),
//         capacity: Capacity::Unlimited,
//         capacity_per_entity: Capacity::Limited(100.),
//     },
// )]);

// let data = Data {
//     entities: HashMap::from([
//         (
//             "A".to_string(),
//             Entity {
//                 resources: HashMap::from([("money".to_string(), 1.)]),
//             },
//         ),
//         (
//             "B".to_string(),
//             Entity {
//                 resources: HashMap::from([("money".to_string(), 3.)]),
//             },
//         ),
//         (
//             "C".to_string(),
//             Entity {
//                 resources: HashMap::from([("money".to_string(), 5.)]),
//             },
//         ),
//     ]),
// };

// let rules = HashMap::from([
//     (
//         "Socialism".to_string(),
//         Box::new(Rule {
//             description: "Richer one gives 2 dollars to poorer one".to_string(),
//             condition: |state: &State| {
//                 let mut money = Vec::new();
//                 for entity in state.data.entities.values() {
//                     money.push(entity.resources.get("money").unwrap());
//                 }
//                 return money.par_iter().any(|x| **x > 2.);
//             },
//             probability_weight: 0.5,
//             actions: |state: &State| {
//                 let poorest_entity_name = state
//                     .data
//                     .entities
//                     .keys()
//                     .nth(get_min_index_f64(
//                         &state
//                             .data
//                             .entities
//                             .values()
//                             .map(|entity| get_resource(entity, &"money".to_string()))
//                             .collect(),
//                     ))
//                     .unwrap()
//                     .clone();

//                 let richest_entity_name = state
//                     .data
//                     .entities
//                     .keys()
//                     .nth(get_max_index_f64(
//                         &state
//                             .data
//                             .entities
//                             .values()
//                             .map(|entity| get_resource(entity, &"money".to_string()))
//                             .collect(),
//                     ))
//                     .unwrap()
//                     .clone();

//                 vec![
//                     Action {
//                         name: "Get".to_string(),
//                         resource: "money".to_string(),
//                         entity: poorest_entity_name.clone(),
//                         new_amount: get_resource(
//                             &get_entity(state, &poorest_entity_name),
//                             &"money".to_string(),
//                         ) + 1.,
//                     },
//                     Action {
//                         name: "Give".to_string(),
//                         resource: "money".to_string(),
//                         entity: richest_entity_name.clone(),
//                         new_amount: get_resource(
//                             &get_entity(state, &richest_entity_name),
//                             &"money".to_string(),
//                         ) - 1.,
//                     },
//                 ]
//             },
//         }),
//     ),
//     (
//         "Capitalism".to_string(),
//         Box::new(Rule {
//             description:
//                 "If somebody has 4 or more dollars and enough capacity, they double their wealth"
//                     .to_string(),
//             condition: |state: &State| {
//                 for entity in state.data.entities.values() {
//                     let money = get_resource(entity, &"money".to_string());
//                     if (4. ..50.).contains(&money) {
//                         return true;
//                     }
//                 }
//                 false
//             },
//             probability_weight: 0.5,
//             actions: |state| {
//                 let mut actions = Vec::new();
//                 for (name, entity) in &state.data.entities {
//                     let money = get_resource(entity, &"money".to_string());
//                     if (4. ..50.).contains(&money) {
//                         actions.push(Action {
//                             name: "Get".to_string(),
//                             resource: "money".to_string(),
//                             entity: name.clone(),
//                             new_amount: money * 2.,
//                         });
//                     }
//                 }
//                 actions
//             },
//         }),
//     ),
// ]);
