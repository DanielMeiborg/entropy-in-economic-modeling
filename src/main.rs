use std::collections::HashMap;

mod simulation;
use simulation::{Action, Entity, Resource, Rule, Simulation, State};

fn main() {
    let resources = vec![Resource {
        name: "money".to_string(),
        description: "In millions of dollars".to_string(),
        capacity: 100,
        must_be_held: true,
        per_entity: false,
        max_held: 100,
    }];

    let initial_state = Box::new(simulation::State {
        entities: vec![
            Entity {
                name: "A".to_string(),
                resources: HashMap::from([(String::from("money"), 5)]),
            },
            Entity {
                name: "B".to_string(),
                resources: HashMap::from([(String::from("money"), 30)]),
            },
            Entity {
                name: "C".to_string(),
                resources: HashMap::from([(String::from("money"), 65)]),
            },
        ],
        probability: 1.0,
    });

    let rules = vec![Box::new(Rule {
        name: "Socialism".to_string(),
        description: "Richer one gives to poorer one".to_string(),
        condition: |state: &State| {
            let mut money = Vec::new();
            for entity in &state.entities {
                for (resource, amount) in &entity.resources {
                    if resource == "money" {
                        money.push(amount);
                    }
                }
            }
            !money.iter().all(|&x| x == money[0])
        },
        probability: 0.5,
        actions: |state: &State| {
            let mut money = Vec::new();
            for entity in &state.entities {
                for (resource, amount) in &entity.resources {
                    if resource == "money" {
                        money.push(amount);
                    }
                }
            }
            let max_index = money
                .iter()
                .position(|&x| x == *money.iter().max().unwrap())
                .unwrap();
            let min_index = money
                .iter()
                .position(|&x| x == *money.iter().min().unwrap())
                .unwrap();

            vec![
                Action {
                    resource: "money".to_string(),
                    entities: vec![state.entities[min_index].name.clone()],
                    amount: 1,
                },
                Action {
                    resource: "money".to_string(),
                    entities: vec![state.entities[max_index].name.clone()],
                    amount: -1,
                },
            ]
        },
    })];

    let main = Simulation::new(resources, initial_state, rules);
    println!("{}", main.entropy);
}
