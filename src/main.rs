use rayon::prelude::*;
use hashbrown::HashMap;


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
                    resources: HashMap::from([("money".to_string(), 10.)]),
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
                    let mut max_money = 0.;
                    let mut richest_entity_name = "C".to_string();
                    for (name, entity) in &state.data.entities {
                        let money = entity.resources.get("money").unwrap();
                        if money > &max_money {
                            max_money = *money;
                            richest_entity_name = name.clone();
                        }
                    }

                    let mut min_money = 0.;
                    let mut poorest_entity_name = "A".to_string();
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
                                &get_entity(state, &richest_entity_name),
                                &"money".to_string(),
                            ) + 1.,
                        },
                        Action {
                            name: "Give".to_string(),
                            resource: "money".to_string(),
                            entities: vec![richest_entity_name.clone()],
                            new_amount: get_resource(
                                &get_entity(state, &poorest_entity_name),
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
                    "If somebody has 4 or more dollars and less than 50, they double their wealth"
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
                                entities: vec![name.clone()],
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
    for t in 1..100 {
        main.next_step();
        println!("Time: {}", t);
        entropies.push(main.entropy);
    }
    println!("================================================");
    println!("Entropies: {:?}", entropies);
    println!("================================================");
    let probability_distribution = main
        .reachable_states
        .par_iter()
        .map(|(_, state)| state.probability)
        .collect::<Vec<f64>>();
    println!("Probability distribution: {:?}", probability_distribution);
    let most_probable_state = main
        .reachable_states
        .par_iter()
        .max_by(|(_, a), (_, b)| a.probability.partial_cmp(&b.probability).unwrap())
        .unwrap();
    println!("most probable state: {:#?}", most_probable_state);
    // println!("Size of simulation: {:?}", std::mem::size_of_val(&main));
}
