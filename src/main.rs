use std::time::SystemTime;

use hashbrown::HashMap;
use petgraph::dot::Dot;
use rayon::prelude::*;

mod simulation;
use simulation::*;
mod utils;
use utils::*;

// Playground
fn main() {
    const NUMBER_OF_BINS: u64 = 10;
    let resources = HashMap::from([(
        "point".to_string(),
        Resource {
            description: "a point a bin can have".to_string(),
            capacity: Capacity::Limited(1.),
            capacity_per_entity: Capacity::Limited(1.),
        },
    )]);

    let data: Data = Data {
        entities: {
            let mut entities = HashMap::new();
            (1..NUMBER_OF_BINS).for_each(|place: u64| {
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

    let rules: HashMap<String, Box<Rule>> = HashMap::from([
        (
            "yield forward".to_string(),
            Box::new(Rule {
                description: "a bin yields the point to the bin with the next higher place"
                    .to_string(),
                probability_weight: 1.,
                condition: |_| true,
                actions: |state: &State| {
                    let current_point_owner = state
                        .data
                        .entities
                        .iter()
                        .find(|(_, entity)| entity.resources["point"] > 0.)
                        .unwrap()
                        .0
                        .clone();
                    let current_point_owner_place: u64 = current_point_owner
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse()
                        .unwrap();
                    let next_point_owner_place: u64 =
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
            }),
        ),
        (
            "yield backward".to_string(),
            Box::new(Rule {
                description: "a bin yields the point to the bin with the next lower place"
                    .to_string(),
                probability_weight: 1.,
                condition: |_| true,
                actions: |state: &State| {
                    let current_point_owner = state
                        .data
                        .entities
                        .iter()
                        .find(|(_, entity)| entity.resources["point"] > 0.)
                        .unwrap()
                        .0
                        .clone();
                    let current_point_owner_place: u64 = current_point_owner
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse()
                        .unwrap();
                    let next_point_owner_place: u64 =
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
            }),
        ),
    ]);

    let mut simulation = Simulation::new(resources, data, rules);
    let mut entropies: Vec<f64> = Vec::new();
    let time = SystemTime::now();
    let mut current_number_of_states: usize;
    for time in 0..10 {
        current_number_of_states = simulation.possible_states.len();
        simulation.next_step();
        entropies.push(simulation.entropy);
        println!(
            "Time: {} Number of states: {}",
            time, current_number_of_states
        );
    }
    let duration = time.elapsed().unwrap();
    println!("================================================");
    write("out/entropies.txt", format!("{:?}", &entropies));
    let probability_distribution = simulation
        .possible_states
        .par_iter()
        .map(|(_, state)| state.probability)
        .collect::<Vec<f64>>();
    write(
        "out/probability_distribution.txt",
        format!("{:?}", &probability_distribution),
    );

    let most_probable_state = simulation
        .possible_states
        .par_iter()
        .max_by(|(_, a), (_, b)| a.probability.partial_cmp(&b.probability).unwrap())
        .unwrap();
    println!("most probable state: {:#?}", most_probable_state);

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
