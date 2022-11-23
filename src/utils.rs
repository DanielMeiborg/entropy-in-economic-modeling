use ordered_float::NotNan;
use rayon::prelude::*;

use crate::simulation::*;

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

pub fn get_resource(
    entity: &Entity,
    resource_name: &String,
) -> f64 {
    entity
        .resources
        .get(resource_name)
        .unwrap()
        .clone()
}

pub fn get_entity(state: &State, entity_name: &String) -> Entity {
    state
        .data
        .entities
        .get(entity_name)
        .unwrap()
        .clone()
}