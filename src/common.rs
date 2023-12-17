//common.rs

use petgraph::graph::{Graph, NodeIndex}; 
use std::collections::HashSet;

//method to find the common friends between nodes
pub fn common_friends(graph: &Graph<u32, ()>, node1: NodeIndex, node2: NodeIndex) -> usize {
    //create a has set of neighbors for the second node to optimize checking whether a node is a neighbor of the second node 
    let neighbors_set: HashSet<_> = graph.neighbors(node2).collect();
    //count number of common friends by filtering neighbors of the first node
    graph
        .neighbors(node1)
        .filter(|&neighbor| neighbors_set.contains(&neighbor))
        .count()
}
//method to find the pair of nodes with the most common friends
pub fn pair_most_common_friends(graph: &Graph<u32, ()>) -> (NodeIndex, NodeIndex, usize) {
    let mut max_common_friends = 0;
    let mut best_pair = (NodeIndex::new(0), NodeIndex::new(1));

    //iterate over all pairs of nodes
    for node1 in graph.node_indices() {
        for node2 in graph.node_indices() {
            if node1 != node2 {
                //calculate the number of common friends
                let common_friends = common_friends(graph, node1, node2);

                //update best pair if the current pair has more common friends
                if common_friends > max_common_friends {
                    max_common_friends = common_friends;
                    best_pair = (node1, node2);
                }
            }
        }
    }
    //returns a tuple of node indices of the best pair and number of common friends they have
    (best_pair.0, best_pair.1, max_common_friends)
}
