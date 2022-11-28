use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use ordered_float::NotNan;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Graph;
use rayon::prelude::*;

use crate::simulation::*;

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

pub fn get_resource(entity: &Entity, resource_name: &String) -> f64 {
    *entity.resources.get(resource_name).unwrap()
}

pub fn get_entity(state: &State, entity_name: &String) -> Entity {
    state.data.entities.get(entity_name).unwrap().clone()
}
