use rayon::prelude::*;
use std::collections::HashMap;

mod simulation;
use simulation::*;

mod utils;
use utils::*;

fn main() {
    let resources = vec![Resource {
        name: "money".to_string(),
        description: "In dollars".to_string(),
        capacity: Capacity::Limited(30.),
        capacity_per_entity: Capacity::Unlimited,
    }];

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
                    resources: HashMap::from([("money".to_string(), 3.)]),
                },
            ),
        ]),
    };

    let rules = vec![
        Box::new(Rule {
            name: "Socialism".to_string(),
            description: "Richer one gives 2 dollars to poorer one".to_string(),
            condition: |state: &State| {
                let mut money = Vec::new();
                for (_, entity) in &state.data.entities {
                    money.push(entity.resources.get("money").unwrap());
                }
                return money.par_iter().any(|&x| x == money[0]);
            },
            probability: 0.3,
            actions: |state: &State| {
                let mut max_money = 0.;
                let mut richest_entity_name = "".to_string();
                for (name, entity) in &state.data.entities {
                    let money = entity.resources.get("money").unwrap();
                    if money > &max_money {
                        max_money = *money;
                        richest_entity_name = name.clone();
                    }
                }

                let mut min_money = 0.;
                let mut poorest_entity_name = "".to_string();
                for (name, entity) in &state.data.entities {
                    let money = entity.resources.get("money").unwrap();
                    if money > &min_money {
                        min_money = *money;
                        poorest_entity_name = name.clone();
                    }
                }

                vec![
                    Action {
                        name: "Get".to_string(),
                        resource: "money".to_string(),
                        entities: vec![],
                        new_amount: get_resource(
                            &get_entity(&state, &richest_entity_name),
                            &"money".to_string(),
                        ) + 1.,
                    },
                    Action {
                        name: "Give".to_string(),
                        resource: "money".to_string(),
                        entities: vec![richest_entity_name.clone()],
                        new_amount: get_resource(
                            &get_entity(&state, &poorest_entity_name),
                            &"money".to_string(),
                        ) - 1.,
                    },
                ]
            },
        }),
        Box::new(Rule {
            name: "Capitalism".to_string(),
            description: "If somebody has 4 or more dollar, they double their wealth".to_string(),
            condition: |state: &State| {
                for (_, entity) in &state.data.entities {
                    let money = get_resource(entity, &"money".to_string());
                    if money >= 4. {
                        return true;
                    }
                }
                return false;
            },
            probability: 0.5,
            actions: |state| {
                let mut actions = Vec::new();
                for (name, entity) in &state.data.entities {
                    let money = get_resource(entity, &"money".to_string());
                    if money >= 4. {
                        actions.push(Action {
                            name: "Get".to_string(),
                            resource: "money".to_string(),
                            entities: vec![name.clone()],
                            new_amount: money * 2.,
                        });
                    }
                }
                return actions;
            },
        }),
    ];

    let mut main = Simulation::new(resources, data, rules);
    let mut entropies: Vec<f64> = Vec::new();
    for t in 1..15 {
        main.next_step();
        println!("Time: {}", t);
        println!("Entropy: {:?}", main.entropy);
        println!(
            "Number of reachable states: {:?}",
            main.reachable_states.len()
        );
        entropies.push(main.entropy);
        println!("Reachable states:");
        for state in &main.reachable_states {
            println!(
                "  State {:?}, probability {:?}",
                state.hash, state.probability
            );
            for (name, entity) in &state.data.entities {
                println!("    Entity {:?}", name);
                for (resource, amount) in &entity.resources {
                    println!("      Resource {:?}: {:?}", resource, amount);
                }
            }
            println!("");
        }
        println!("================================================");
    }
    println!("Entropies: {:?}", entropies);
    println!(
        "Ratio of each entropy to the previous one: {:?}",
        entropies
            .windows(2)
            .map(|w| w[1] / w[0])
            .collect::<Vec<f64>>()
    );
}
