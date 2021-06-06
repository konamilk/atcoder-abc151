use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use itertools::Itertools;

fn main() {
    let source = AutoSource::from("3 5
    ...#.
    .#.#.
    .#...");
    input!{
        // from source,
        h: usize,
        w: usize,
        s: [Chars; h]
    };

    let mut indices = vec![];

    let n = w * h;
    let mut edges = vec![];

    for i in 0usize..h {
        for j in 0usize..w{
            let idx = i * w + j;
            if s[i][j] == '.'{
                indices.push(idx)
            }
            if j + 1 < w && s[i][j] == '.' && s[i][j+1] == '.'{
                edges.push((idx, idx+1))
            }
            if i + 1 < h && s[i][j] == '.' && s[i+1][j] == '.'{
                edges.push((idx, idx + w))
            }
        }
    }

    let mut g = Graph::<(), (), Undirected>::new_undirected();
    let nodes: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
    g.extend_with_edges(edges.iter().map(|&(i, j)| (nodes[i], nodes[j])));

    // println!("{:?}", g);

    let mut ans = 0usize;


    for xs in (0..indices.len()).combinations(2) {
        // println!("{:?}",xs);

        let start = indices[xs[0]];
        let end = indices[xs[1]];

        let mut bfs = MyBfs::new(&g, node_index(start));
        while let Some(nx) = bfs.next(&g) {
            // println!("[{}] {}", nx.index(), bfs.depth);
            if nx.index() == end {
                ans = max(ans,  bfs.depth);
                // println!("{} {} {}", start, end, bfs.depth);
                break;
            }
        }
    }

    println!("{}", ans);
}

/// MyBfs
///
/// # Examples
///
/// ```
/// let n = 9;
/// let edges = [(0usize, 1usize), (0, 2), (0, 4), (1, 3), (1, 4), (1, 8), (2, 5),
///     (3, 7), (3, 8), (4, 8), (5, 6), (5, 8), (6, 7)];
///
/// let mut g = Graph::<(), (), Undirected>::new_undirected();
/// let nodes: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
/// g.extend_with_edges(edges.iter().map(|&(i, j)| (nodes[i], nodes[j])));
///
/// let mut bfs = MyBfs::new(&g, node_index(0));
/// while let Some(nx) = bfs.next(&g) {
///     println!("[{}] {}", nx.index(), bfs.depth)
/// }
///
/// ```
use petgraph::graph::{Graph, UnGraph, node_index};
use petgraph::{Undirected, Directed};
use petgraph::visit::{Dfs, Bfs, depth_first_search, Control, DfsEvent, NodeCount};
use petgraph::visit::{IntoNeighbors, GraphRef, VisitMap, Visitable};
use std::collections::VecDeque;

#[derive(Clone)]
pub struct MyBfs<N, VM> {
    /// The queue of nodes to visit
    pub stack: VecDeque<N>,
    /// The map of discovered nodes
    pub discovered: VM,
    stack_depth: VecDeque<usize>,
    pub depth: usize,
}

impl<N, VM> MyBfs<N, VM>
    where
        N: Copy + PartialEq,
        VM: VisitMap<N>,
{
    /// Create a new **Bfs**, using the graph's visitor map, and put **start**
    /// in the stack of nodes to visit.
    pub fn new<G>(graph: G, start: N) -> Self
        where
            G: GraphRef + Visitable<NodeId=N, Map=VM> + NodeCount,
    {
        let mut discovered = graph.visit_map();
        discovered.visit(start);
        let mut stack = VecDeque::new();
        stack.push_front(start);
        let mut stack_depth = VecDeque::new();
        stack_depth.push_front(0);
        let mut depth = 0;
        MyBfs { stack, discovered, stack_depth, depth }
    }

    /// Return the next node in the bfs, or **None** if the traversal is done.
    pub fn next<G>(&mut self, graph: G) -> Option<N>
        where
            G: IntoNeighbors<NodeId=N>,
    {
        if let Some(node) = self.stack.pop_front() {
            self.depth = self.stack_depth.pop_front().unwrap();
            for succ in graph.neighbors(node) {
                if self.discovered.visit(succ) {
                    self.stack.push_back(succ);
                    self.stack_depth.push_back(self.depth + 1);
                }
            }

            return Some(node);
        }
        None
    }
}
