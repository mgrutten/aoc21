use petgraph::graph::DefaultIx;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;


fn find_paths(graph: &Graph<Cave, (), Undirected>,
              start: NodeIndex,
              visited: &HashSet<NodeIndex>) -> u64 {
    if *graph.node_weight(start).unwrap() == Cave::End {
        return 1;
    }

    let mut path_count = 0;
    for node in graph.neighbors(start) {
        let cave_size = graph.node_weight(node).unwrap();
        if *cave_size == Cave::Big {
            path_count += find_paths(graph, node, visited);
        } else if *cave_size != Cave::Start && !visited.contains(&node) {
            let mut new_visited = visited.clone();
            new_visited.insert(node);
            path_count += find_paths(graph, node, &new_visited);
        }
    }

    path_count
}

fn part1(graph: &Graph<Cave, (), Undirected>) {
    let start_node = NodeIndex::<DefaultIx>::new(0);
    let path_count = find_paths(graph, start_node, &HashSet::new());
    println!("Part 1: {}", path_count);
}


fn find_paths2(graph: &Graph<Cave, (), Undirected>,
               start: NodeIndex,
               visited: &HashMap<NodeIndex, u8>) -> u64 {
    if start == NodeIndex::<DefaultIx>::new(1) {
        return 1;
    }

    let mut path_count = 0;
    for node in graph.neighbors(start) {
        let cave_size = graph.node_weight(node).unwrap();
        if *cave_size == Cave::Big {
            path_count += find_paths2(graph, node, visited);
        } else if *cave_size != Cave::Start {
            if !visited.contains_key(&node) {
                let mut new_visited = visited.clone();
                new_visited.insert(node, 0);
                path_count += find_paths2(graph, node, &new_visited);
            } else {
                let visit_sum = visited.values().sum::<u8>();
                if visit_sum == 0 {
                    let mut new_visited = visited.clone();
                    new_visited.insert(node, 1);
                    path_count += find_paths2(graph, node, &new_visited);
                }
            }
        }
    }

    path_count
}


fn part2(graph: &Graph<Cave, (), Undirected>) {
    let start_node = NodeIndex::<DefaultIx>::new(0);
    let path_count = find_paths2(graph, start_node, &HashMap::new());
    println!("Part 2: {}", path_count);
}


#[derive(Debug, PartialEq)]
enum Cave {
    Small,
    Big,
    Start,
    End,
}

fn add_node<'a>(name: &'a str,
                graph: &mut Graph<Cave, (), Undirected>,
                node_labels: &mut HashMap<&'a str, NodeIndex<DefaultIx>>) -> NodeIndex<DefaultIx> {
    *node_labels.entry(name).or_insert_with(|| {
        let node_type = if name.to_lowercase() == name {
            Cave::Small
        } else {
            Cave::Big
        };
        graph.add_node(node_type)
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day12/day12-example1.txt")?;

    // Undirected graph
    let mut graph: Graph<Cave, (), _> = Graph::new_undirected();
    let mut node_labels = HashMap::from([
        ("start", graph.add_node(Cave::Start)),
        ("end", graph.add_node(Cave::End)),
    ]);

    for line in file_str.lines() {
        let lr = line.split('-').collect::<Vec<&str>>();

        let l_node = add_node(lr[0], &mut graph, &mut node_labels);
        let r_node = add_node(lr[1], &mut graph, &mut node_labels);

        graph.add_edge(l_node, r_node, ());
    }

    part1(&graph);
    part2(&graph);

    Ok(())
}