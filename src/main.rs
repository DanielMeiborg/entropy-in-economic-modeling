use std::collections::HashMap;
use rayon::prelude::*;

mod simulation;
use simulation::*;

fn main() {
    let resources = vec![Resource {
        name: "money".to_string(),
        description: "In dollars".to_string(),
        capacity: Capacity::Limited(Amount::Float(30.)),
        capacity_per_entity: CapacityPerEntity::Unlimited,
    }];

    let initial_state = vec![
        Entity {
            name: "A".to_string(),
            resources: HashMap::from([("money".to_string(), Amount::Float(1.))]),
        },
        Entity {
            name: "B".to_string(),
            resources: HashMap::from([("money".to_string(), Amount::Float(3.))]),
        },
        Entity {
            name: "C".to_string(),
            resources: HashMap::from([("money".to_string(), Amount::Float(6.))]),
        },
    ];

    let rules = vec![
        Box::new(Rule {
            name: "Socialism".to_string(),
            description: "Richer one gives 2 dollars to poorer one".to_string(),
            condition: |state: &State| {
                let mut money = Vec::new();
                for entity in &state.entities {
                    money.push(entity.resources.get("money").unwrap());
                }
                return money.par_iter().any(|&x| x == money[0]);
            },
            probability: 0.3,
            actions: |state: &State| {
                let mut money: Vec<Amount> = Vec::new();
                for entity in &state.entities {
                    money.push(entity.resources.get("money").unwrap().clone());
                }
                let max_index =  money
                    .par_iter()
                    .position_any(|x| *x == *money.par_iter().max().unwrap())
                    .unwrap();
                let min_index = money
                    .par_iter()
                    .position_any(|x| *x == *money.par_iter().min().unwrap())
                    .unwrap();

                let current_amount = state.entities[max_index].resources.get("money").unwrap();

                vec![
                    Action {
                        name: "Get".to_string(),
                        resource: "money".to_string(),
                        entities: vec![state.entities[min_index].name.clone()],
                        new_amount: current_amount.clone() + Amount::Float(1.),
                    },
                    Action {
                        name: "Give".to_string(),
                        resource: "money".to_string(),
                        entities: vec![state.entities[max_index].name.clone()],
                        new_amount: current_amount.clone() - Amount::Float(1.),
                    },
                ]
            },
        }),
        Box::new(Rule {
            name: "Capitalism".to_string(),
            description: "If somebody has 4 or more dollar, they double their wealth".to_string(),
            condition: |state: &State| {
                for entity in &state.entities {
                    let money = entity.resources.get("money").unwrap();
                    if money >= &Amount::Float(4.) {
                        return true;
                    }
                }
                return false;
            },
            probability: 0.5,
            actions: |state| {
                let mut actions = Vec::new();
                for entity in &state.entities {
                    let money = entity.resources.get("money").unwrap();
                    if money >= &Amount::Float(4.) {
                        actions.push(Action {
                            name: "Get".to_string(),
                            resource: "money".to_string(),
                            entities: vec![entity.name.clone()],
                            new_amount: money.clone() * Amount::Float(2.),
                        });
                    }
                }
                return actions;
            },
        }),
    ];

    let mut main = Simulation::new(resources, initial_state, rules);
    let mut entropies: Vec<f64> = Vec::new();
    for t in 1..5 {
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
            for entity in &state.entities {
                println!("    Entity {:?}", entity.name);
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
