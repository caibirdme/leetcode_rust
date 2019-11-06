use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn tree_diameter(edges: Vec<Vec<i32>>) -> i32 {
        if edges.is_empty() {
            return 0;
        }
        if edges.len() == 1 {
            return 1;
        }
        let mut graph = HashMap::new();
        for edge in edges {
            let (a,b) = (edge[0], edge[1]);
            graph.entry(a).or_insert(HashSet::new()).insert(b);
            graph.entry(b).or_insert(HashSet::new()).insert(a);
        }
        let (t, _) = Self::bfs(&graph, 0);
        let (_, ans) = Self::bfs(&graph, t);
        ans
    }
    fn bfs(graph: &HashMap<i32, HashSet<i32>>, start: i32) -> (i32, i32) {
        let mut visited = HashSet::new();
        let mut q = vec![];
        let mut s = 0;
        let mut ans = 0;
        q.push((start, 0));
        visited.insert(start);
        while !q.is_empty() {
            let (cur, dist) = q.pop().unwrap();
            if ans < dist {
                ans = dist;
                s = cur;
            }
            if let Some(nodes) = graph.get(&cur) {
                for &node in nodes.iter() {
                    if !visited.contains(&node) {
                        visited.insert(node);
                        q.push((node, dist+1));
                    }
                }
            }
        }
        (s, ans)
    }
}

struct Solution;