//main.rs 

mod graph;
mod jaccard;
mod common;

#[cfg(test)]
mod tests;

use graph::SocialGraph;
use jaccard::find_highest_sim;
use petgraph::graph::NodeIndex;
use rand::seq::SliceRandom;
use common::{pair_most_common_friends};

//method to get a random sample of nodes from the graph
fn sample_nodes(graph: &SocialGraph, sample_size: usize) -> Vec<NodeIndex> {
    let all_nodes: Vec<NodeIndex> = graph.graph.node_indices().collect();
    let mut rng = rand::thread_rng();

    let sampled_nodes: Vec<NodeIndex> = all_nodes
        .choose_multiple(&mut rng, sample_size)
        .cloned()
        .collect();

    sampled_nodes
}

fn main() {
    // creating the social_graph 
    let mut social_graph = SocialGraph::new();
    //using the facebook_combined.txt file to create the social graph
    social_graph.load_edges("facebook_combined.txt").expect("Failed to load edges");

    //finding the pair of nodes from the entire grah with the highest number of common friends
    let (best_node1, best_node2, max_common_friends) =
        pair_most_common_friends(&social_graph.graph);

    println!("From Entire Graph:");
    println!(
        "Nodes {} and {} have the most common friends: {}",
        best_node1.index(),
        best_node2.index(),
        max_common_friends
    );
    
    //taking a random sample from the graph to make runtime faster
    let sampled_nodes = sample_nodes(&social_graph, 1000);

    //finding the pair of nodes from this random sample with the hgihest jaccard similarity
    if let Some((node1_sim, node2_sim, highest_sim)) =
        find_highest_sim(&social_graph, sampled_nodes.clone().as_slice())
    {
        println!("From Random Sample:");
        println!(
            "Nodes with highest similarity: {:?} and {:?}, Similarity: {:.3}",
            social_graph.graph[node1_sim],
            social_graph.graph[node2_sim],
            highest_sim
        );
    } else {
        println!("Unable to find highest similarity and dissimilarity");
    }

}
