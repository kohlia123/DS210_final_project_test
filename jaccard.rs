//jaccard.rs

use petgraph::graph::NodeIndex;
use std::collections::HashSet;
use crate::graph::SocialGraph;

//method to calculate the jaccard similarity between two sets
pub fn calc_sim<T>(set1: &HashSet<T>, set2: &HashSet<T>) -> f64
where
    T: Eq + std::hash::Hash + std::fmt::Debug,
{
    let intersection_size = set1.intersection(set2).count() as f64;
    let union_size = set1.union(set2).count() as f64;

    if union_size.abs() < std::f64::EPSILON {
        0.0 // avoid dividing by 0
    } else {
        intersection_size / union_size
    }
}
//method uses calc_sim to find the jaccard similarity between the neighbors of the two nodes.
pub fn jaccard_similarity(graph: &SocialGraph, node1: NodeIndex, node2: NodeIndex) -> f64 {
    let neighbors1: HashSet<_> = graph.graph.neighbors(node1).collect();
    let neighbors2: HashSet<_> = graph.graph.neighbors(node2).collect();
    if neighbors1.is_empty() || neighbors2.is_empty() {
        //return a value indicating no similarity if either set of neighbors is empty
        return 0.0;
    }

    calc_sim(&neighbors1, &neighbors2)
}
//method finds highest jaccard similairty between a pair of nodes from a sample of the graph
pub fn find_highest_sim(graph: &SocialGraph, sampled_nodes: &[NodeIndex]) -> Option<(NodeIndex, NodeIndex, f64)> {
    if sampled_nodes.is_empty() {
        return None;
    }
    //initialize highest_sim variable with the first pair of nodes and a similarity of 0.0
    let mut highest_sim = (sampled_nodes[0], sampled_nodes[1], 0.0);
    
    for (i, &node1) in sampled_nodes.iter().enumerate() {
        let neighbors1: HashSet<_> = graph.graph.neighbors(node1).collect();
        for &node2 in &sampled_nodes[i + 1..] {
            
            let neighbors2: HashSet<_> = graph.graph.neighbors(node2).collect();

            //skip the pair if either set of neighbors is empty
            if neighbors1.is_empty() || neighbors2.is_empty() {
                continue;
            }
            //calculate similarity of the pair of nodes
            let similarity = jaccard_similarity(graph, node1, node2);
            //compare to the current hgihest similarity and update accordingly 
            if similarity > highest_sim.2 {
                highest_sim = (node1, node2, similarity);
            }
            
        }
    }
    //return an option with the node pair indices and the similarity
    Some(highest_sim)
    
}
