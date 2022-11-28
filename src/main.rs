use std::time::SystemTime;

use hashbrown::HashMap;
use petgraph::algo;
use petgraph::dot::Dot;
use petgraph::graph::NodeIndex;
use rayon::prelude::*;

mod simulation;
use simulation::*;
mod utils;
use utils::*;

fn main() {
    let resources = HashMap::from([(
        "money".to_string(),
        Resource {
            description: "In dollars".to_string(),
            capacity: Capacity::Unlimited,
            capacity_per_entity: Capacity::Limited(100.),
        },
    )]);

    let data = Data {
        entities: HashMap::from([
            (
                "A".to_string(),
                Entity {
                    resources: HashMap::from([("money".to_string(), 1.)]),
                },
            ),
            (
                "B".to_string(),
                Entity {
                    resources: HashMap::from([("money".to_string(), 3.)]),
                },
            ),
            (
                "C".to_string(),
                Entity {
                    resources: HashMap::from([("money".to_string(), 5.)]),
                },
            ),
        ]),
    };

    let rules = HashMap::from([
        (
            "Socialism".to_string(),
            Box::new(Rule {
                description: "Richer one gives 2 dollars to poorer one".to_string(),
                condition: |state: &State| {
                    let mut money = Vec::new();
                    for entity in state.data.entities.values() {
                        money.push(entity.resources.get("money").unwrap());
                    }
                    return money.par_iter().any(|x| **x > 2.);
                },
                probability: 0.5,
                actions: |state: &State| {
                    let poorest_entity_name = state
                        .data
                        .entities
                        .keys()
                        .nth(get_min_index_f64(
                            &state
                                .data
                                .entities
                                .values()
                                .map(|entity| get_resource(entity, &"money".to_string()))
                                .collect(),
                        ))
                        .unwrap()
                        .clone();

                    let richest_entity_name = state
                        .data
                        .entities
                        .keys()
                        .nth(get_max_index_f64(
                            &state
                                .data
                                .entities
                                .values()
                                .map(|entity| get_resource(entity, &"money".to_string()))
                                .collect(),
                        ))
                        .unwrap()
                        .clone();

                    vec![
                        Action {
                            name: "Get".to_string(),
                            resource: "money".to_string(),
                            entity: poorest_entity_name.clone(),
                            new_amount: get_resource(
                                &get_entity(state, &poorest_entity_name),
                                &"money".to_string(),
                            ) + 1.,
                        },
                        Action {
                            name: "Give".to_string(),
                            resource: "money".to_string(),
                            entity: richest_entity_name.clone(),
                            new_amount: get_resource(
                                &get_entity(state, &richest_entity_name),
                                &"money".to_string(),
                            ) - 1.,
                        },
                    ]
                },
            }),
        ),
        (
            "Capitalism".to_string(),
            Box::new(Rule {
                description:
                    "If somebody has 4 or more dollars and enough capacity, they double their wealth"
                        .to_string(),
                condition: |state: &State| {
                    for entity in state.data.entities.values() {
                        let money = get_resource(entity, &"money".to_string());
                        if (4. ..50.).contains(&money) {
                            return true;
                        }
                    }
                    false
                },
                probability: 0.5,
                actions: |state| {
                    let mut actions = Vec::new();
                    for (name, entity) in &state.data.entities {
                        let money = get_resource(entity, &"money".to_string());
                        if (4. ..50.).contains(&money) {
                            actions.push(Action {
                                name: "Get".to_string(),
                                resource: "money".to_string(),
                                entity: name.clone(),
                                new_amount: money * 2.,
                            });
                        }
                    }
                    actions
                },
            }),
        ),
    ]);

    let mut main = Simulation::new(resources, data, rules);
    let mut entropies: Vec<f64> = Vec::new();
    let time = SystemTime::now();
    for t in 1..50 {
        main.next_step();
        println!("Time: {}", t);
        entropies.push(main.entropy);
    }
    let duration = time.elapsed().unwrap();
    println!("================================================");
    // println!("Entropies: {:?}", entropies);
    // println!("================================================");
    write("out/entropies.txt", format!("{:?}", &entropies));
    let probability_distribution = main
        .reachable_states
        .par_iter()
        .map(|(_, state)| state.probability)
        .collect::<Vec<f64>>();
    // println!("Probability distribution: {:?}", probability_distribution);
    write(
        "out/probability_distribution.txt",
        format!("{:?}", &probability_distribution),
    );

    let most_probable_state = main
        .reachable_states
        .par_iter()
        .max_by(|(_, a), (_, b)| a.probability.partial_cmp(&b.probability).unwrap())
        .unwrap();
    println!("most probable state: {:#?}", most_probable_state);

    println!("The simulation took {} seconds", duration.as_secs_f64());

    let graph = main.get_graph_from_cache();
    println!("Nodes in graph: {}", graph.node_count());
    println!("Edges in graph {}", graph.edge_count());
    let initial_node = graph
        .node_indices()
        .find(|node| graph[*node].data == main.initial_state.data)
        .unwrap();

    let most_probable_node = graph
        .node_indices()
        .find(|node| graph[*node] == *most_probable_state.1)
        .unwrap();

    // let nodes: Vec<NodeIndex> = algo::astar(
    //     &graph,
    //     initial_node,
    //     |n| n == most_probable_node,
    //     |_| 1,
    //     |_| 0,
    // )
    // .unwrap()
    // .1;

    let paths =
        algo::all_simple_paths::<Vec<_>, _>(&graph, initial_node, most_probable_node, 0, None)
            .collect::<Vec<Vec<NodeIndex>>>();
    println!("Number of paths: {}", paths.len());
    let min_len_path = &paths
        .par_iter()
        .min_by(|a, b| a.len().partial_cmp(&b.len()).unwrap())
        .unwrap()
        .len();
    let nodes = &paths
        .into_par_iter()
        .filter(|path| path.len() == *min_len_path)
        .collect::<Vec<Vec<NodeIndex>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<NodeIndex>>();
    // let nodes = graph
    //     .neighbors_undirected(initial_node)
    //     .collect::<Vec<NodeIndex>>();

    let subgraph = get_subgraph::<Box<State>, String>(nodes.clone(), &graph);
    write(
        "out/graph.dot",
        format!("{:?}", Dot::with_config(&graph, &[])),
    );
    println!("Nodes in subgraph: {}", subgraph.node_count());
    println!("Edges in subgraph {}", subgraph.edge_count());
}
