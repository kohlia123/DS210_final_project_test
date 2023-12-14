//graph.rs

use petgraph::graph::{Graph, NodeIndex}; //using petgraph crate for building the graph
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

//struct representing a social graph.
pub struct SocialGraph {
    pub graph: Graph<u32, ()>,
    node_indices: HashMap<u32, NodeIndex>,
}

impl SocialGraph {
    //method to create new instance of social graph
    pub fn new() -> Self {
        SocialGraph {
            graph: Graph::new(), //initializes empty graph
            node_indices: HashMap::new(), //initializes empty hash map for node indices
        }
    }
    //method to load edges from a file into the graph.
    pub fn load_edges(&mut self, edges_file_path: &str) -> io::Result<()> {
        //open the file specified by the path.
        let file = File::open(edges_file_path)?;
        //create a buffered reader for efficient file reading.
        let reader = io::BufReader::new(file);

        //iterate over lines in the file, skipping any errors.
        for line in reader.lines().filter_map(Result::ok) {
            //split each line into a vector of whitespace-separated u32 values.
            let edge_info: Vec<u32> = line.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();

            //check if the line contains exactly two elements.
            if edge_info.len() == 2 {
                let source = edge_info[0];
                let target = edge_info[1];
        
                //get or insert the node indices for the source and target nodes.
                let source_node = *self.node_indices
                    .entry(source)
                    .or_insert_with(|| self.graph.add_node(source));
        
                let target_node = *self.node_indices
                    .entry(target)
                    .or_insert_with(|| self.graph.add_node(target));
        
                //sdd an edge between the source and target nodes with an empty weight ().
                self.graph.add_edge(source_node, target_node, ());
            } else {
                //print an error message for improperly formatted lines.
                eprintln!("Skipping improperly formatted line: {}", line);
            }
        }

        Ok(()) //return Ok(()) indicating successful execution.
    }
}
