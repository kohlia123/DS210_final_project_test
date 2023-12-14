#[cfg(test)]
mod tests {
    use super::*; 
    use crate::common::{common_friends};
    use crate::graph::SocialGraph;
    use crate::jaccard::{calc_sim, jaccard_similarity};
    use std::collections::HashSet;
    use petgraph::Graph;
    use petgraph::graph::NodeIndex;
    
    #[test]
    fn test_calc_sim() {
        let set1: HashSet<_> = vec![1, 2, 3].into_iter().collect();
        let set2: HashSet<_> = vec![2, 3, 4].into_iter().collect();

        let similarity = calc_sim(&set1, &set2);

        assert!(similarity <= 1.0);
    }

    #[test]
    fn test_jaccard_sim() {
        let social_graph = SocialGraph::new();
        let node1 = NodeIndex::new(0);
        let node2 = NodeIndex::new(1);

        let similarity = jaccard_similarity(&social_graph, node1, node2);

        assert_eq!(similarity, 0.0);
    }

    #[test]
    fn test_common_friends() {
        let mut graph = Graph::new();
        let node1 = graph.add_node(1);
        let node2 = graph.add_node(2);
        let node3 = graph.add_node(3);

        graph.add_edge(node1, node3, ());
        graph.add_edge(node2, node3, ());

        let result = common_friends(&graph, node1, node2);
        assert_eq!(result, 1);

        let result = common_friends(&graph, node2, node3);
        assert_eq!(result, 0);
    }

   
}