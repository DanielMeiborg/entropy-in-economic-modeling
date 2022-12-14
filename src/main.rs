use std::time::SystemTime;

use hashbrown::HashMap;
use itertools::Itertools;
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

pub fn write_to_file<P>(path: P, content: String)
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
        ResourceName("point".to_string()),
        Resource {
            description: "a point a bin can have".to_string(),
            capacity: Capacity::Limited(Amount(1.)),
            capacity_per_entity: Capacity::Limited(Amount(1.)),
        },
    )]);

    let initial_state = State {
        entities: {
            let mut entities = HashMap::new();
            (1..NUMBER_OF_BINS).for_each(|place: i64| {
                entities.insert(
                    EntityName(format!("bin {place}")),
                    Entity {
                        resources: HashMap::from([(ResourceName("point".to_string()), Amount(0.))]),
                    },
                );
            });
            entities.insert(
                EntityName("bin 0".to_string()),
                Entity {
                    resources: HashMap::from([(ResourceName("point".to_string()), Amount(1.))]),
                },
            );
            entities
        },
    };

    let rules = HashMap::from([
        (
            RuleName("yield forward".to_string()),
            Rule {
                description: "a bin yields the point to the bin with the next higher place"
                    .to_string(),
                probability_weight: ProbabilityWeight(1.),
                condition: |_| RuleApplies(true),
                actions: |state: State| {
                    let current_point_owner = state
                        .entities
                        .iter()
                        .find(|(_, entity)| {
                            *entity
                                .resources
                                .get(&ResourceName("point".to_string()))
                                .unwrap()
                                > Amount(0.)
                        })
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
                    let next_point_owner = EntityName(format!("bin {}", next_point_owner_place));
                    vec![
                        Action {
                            name: "yield point".to_string(),
                            entity: current_point_owner,
                            resource: ResourceName("point".to_string()),
                            new_amount: Amount(0.),
                        },
                        Action {
                            name: "get point".to_string(),
                            entity: next_point_owner,
                            resource: ResourceName("point".to_string()),
                            new_amount: Amount(1.),
                        },
                    ]
                },
            },
        ),
        (
            RuleName("yield backward".to_string()),
            Rule {
                description: "a bin yields the point to the bin with the next lower place"
                    .to_string(),
                probability_weight: ProbabilityWeight(1.),
                condition: |_| RuleApplies(true),
                actions: |state: State| {
                    let current_point_owner = state
                        .entities
                        .iter()
                        .find(|(_, entity)| {
                            *entity
                                .resources
                                .get(&ResourceName("point".to_string()))
                                .unwrap()
                                > Amount(0.)
                        })
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
                    let next_point_owner = EntityName(format!("bin {}", next_point_owner_place));
                    vec![
                        Action {
                            name: "yield point".to_string(),
                            entity: current_point_owner,
                            resource: ResourceName("point".to_string()),
                            new_amount: Amount(0.),
                        },
                        Action {
                            name: "get point".to_string(),
                            entity: next_point_owner,
                            resource: ResourceName("point".to_string()),
                            new_amount: Amount(1.),
                        },
                    ]
                },
            },
        ),
    ]);

    let mut simulation = Simulation::create(resources, initial_state, rules);
    let mut entropies: Vec<Entropy> = Vec::new();
    let time = SystemTime::now();
    for time in 0..100 {
        entropies.push(simulation.entropy);
        println!(
            "Time: {} Number of reachable states: {}",
            time,
            simulation.reachable_states.len()
        );
        simulation.next_step();
    }
    let duration = time.elapsed().unwrap();
    println!("================================================");
    write_to_file("out/entropies.txt", format!("{:?}", &entropies));
    let probability_distribution: Vec<f64> = simulation
        .reachable_states
        .values()
        .map_into::<f64>()
        .collect();
    write_to_file(
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
        .find(|(_, probability)| **probability == highest_probability);

    println!("most probable state: {:#?}", most_likely_state);

    println!("The simulation took {} seconds", duration.as_secs_f64());

    let graph = simulation.get_graph_from_cache();
    println!("Nodes in graph: {}", graph.node_count());
    println!("Edges in graph {}", graph.edge_count());
    write_to_file(
        "out/graph.dot",
        format!("{:?}", Dot::with_config(&graph, &[])),
    );
    let simulation_is_doubly_statistical = simulation.is_doubly_statistical();
    println!(
        "Simulation is doubly statistical: {}",
        simulation_is_doubly_statistical
    );
}
