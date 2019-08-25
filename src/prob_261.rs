use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        if edges.is_empty() {
            if n == 1 {
                return true;
            }
            return false;
        }
        if (edges.len() as i32) < n-1 {
            return false;
        }
        let mut count = vec![0; n as usize];
        let mut graph = HashMap::new();
        for edge in &edges {
            let (s, e) = (edge[0] as usize, edge[1] as usize);
            if s == e {
                return false;
            }
            let p = graph.entry(s).or_insert(HashSet::new());
            p.insert(e);
            let q = graph.entry(e).or_insert(HashSet::new());
            q.insert(s);
            count[s] += 1;
            count[e] += 1;
        }
        loop {
            let mut ok = false;
            for i in 0..n as usize {
                if count[i] == 1 {
                    ok = true;
                    count[i] = 0;
                    let g = graph.remove(&i).unwrap();
                    for node in g {
                        count[node] -= 1;
                        let ng = graph.get_mut(&node).unwrap();
                        ng.remove(&i);
                    }
                }
            }
            if !ok {
                for &v in &count {
                    if v > 0 {
                        return false;
                    }
                }
                return true;
            }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_valid_tree() {
        let test_cases = vec![
            (2, vec![], false),
            (1, vec![], true),
            (4, vec![vec![0,1], vec![2,3]], false),
            (2, vec![vec![0, 1]], true),
            (3, vec![vec![0, 1], vec![0,2]], true),
            (3, vec![vec![0, 1], vec![1,2]], true),
            (3, vec![vec![0, 1], vec![1,2], vec![0,2]], false),
            (5, vec![vec![0,1], vec![1,2], vec![2,3], vec![1,3], vec![1,4]], false),
            (5, vec![vec![0,1], vec![0,2], vec![0,3], vec![1,4]], true),
        ];
        for (n, graph, ok) in test_cases {
            assert_eq!(Solution::valid_tree(n,graph.clone()), ok, "n: {}, graph: {:?}", n, graph);
        }
    }
}