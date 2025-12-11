use crate::{Solution, SolutionPair};
use petgraph::prelude::*;
use std::collections::HashMap;

/// Read the input as a successors relation.
fn prepare(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|line| line.trim())
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                let (pred, succs) = line.split_once(':').unwrap();
                Some((
                    pred.to_string(),
                    succs
                        .split(' ')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                ))
            }
        })
        .collect()
}

fn build_graph<N, E>(
    successors: &HashMap<String, Vec<String>>,
) -> (DiGraph<N, E>, HashMap<String, NodeIndex>)
where
    N: Default,
    E: Default,
{
    let mut graph = DiGraph::<N, E>::new();
    let mut node_to_index = HashMap::new();
    successors.iter().for_each(|(pred, succs)| {
        if !node_to_index.contains_key(pred) {
            node_to_index.insert(pred.to_string(), graph.add_node(Default::default()));
        }
        succs.iter().for_each(|succ| {
            if !node_to_index.contains_key(succ) {
                node_to_index.insert(succ.to_string(), graph.add_node(Default::default()));
            }
        });
    });
    let edges = successors.iter().flat_map(|(pred, succs)| {
        succs
            .iter()
            .map(|succ| (node_to_index[pred], node_to_index[succ], Default::default()))
    });
    graph.extend_with_edges(edges);
    (graph, node_to_index)
}

fn count_simple_paths<E>(mut graph: Graph<u64, E>, from: NodeIndex, to: NodeIndex) -> u64 {
    assert!(!petgraph::algo::is_cyclic_directed(&graph));
    graph.node_weights_mut().for_each(|w| *w = 0u64);

    // We count paths with a DFS in post-order on the reversed graph.
    //
    // The number of paths reaching node `nx` is:
    // - the sum of the number of paths reaching its predecessor,
    // - or 1 if `nx` is the starting point.
    graph.reverse();
    let mut dfs = DfsPostOrder::new(&graph, to);
    while let Some(nx) = dfs.next(&graph) {
        if nx == from {
            graph[nx] = 1;
        } else {
            graph[nx] = graph
                .neighbors_directed(nx, Direction::Outgoing)
                .map(|neigh| graph[neigh])
                .sum();
        }
    }
    graph[to]
}

fn solve_part1(input: &str) -> u64 {
    let successors = prepare(input);
    let (graph, node_to_index) = build_graph::<u64, ()>(&successors);
    let you = node_to_index["you"];
    let out = node_to_index["out"];
    count_simple_paths(graph, you, out)
}

fn solve_part2(input: &str) -> u64 {
    let successors = prepare(input);
    let (graph, node_to_index) = build_graph::<u64, ()>(&successors);

    // since the graph is acyclic, all paths that go through 'fft' and 'dac' reach them in the same
    // order:
    // Either svr -> dac -> fft -> out
    //     or svr -> fft -> dac -> out
    assert!(!petgraph::algo::is_cyclic_directed(&graph));

    let svr = node_to_index["svr"];
    let dac = node_to_index["dac"];
    let fft = node_to_index["fft"];
    let out = node_to_index["out"];

    // test reachability from dac to fft
    let node_order = if count_simple_paths(graph.clone(), dac, fft) > 0 {
        [svr, dac, fft, out]
    } else {
        [svr, fft, dac, out]
    };

    // simply count the paths between each hop and multiply them.
    count_simple_paths(graph.clone(), node_order[0], node_order[1])
        * count_simple_paths(graph.clone(), node_order[1], node_order[2])
        * count_simple_paths(graph.clone(), node_order[2], node_order[3])
}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
    aaa: you hhh
    you: bbb ccc
    bbb: ddd eee
    ccc: ddd eee fff
    ddd: ggg
    eee: out
    fff: out
    ggg: out
    hhh: ccc fff iii
    iii: out";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 5);
    }

    const EXAMPLE_INPUT_2: &str = "svr: aaa bbb
    aaa: fft
    fft: ccc
    bbb: tty
    tty: ccc
    ccc: ddd eee
    ddd: hub
    hub: fff
    eee: dac
    dac: fff
    fff: ggg hhh
    ggg: out
    hhh: out";

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT_2), 2);
    }
}
