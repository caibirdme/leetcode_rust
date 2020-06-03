use std::collections::HashMap;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut graph = HashMap::new();
        let mut come_from = HashMap::new();
        for edge in connections {
            let (from, to) = (edge[0], edge[1]);
            graph.entry(from).or_insert(vec![]).push(to);
            come_from.entry(to).or_insert(vec![]).push(from);
        }
        let mut head = 0;
        let mut q = vec![0];
        let mut used = vec![false; n as usize];
        used[0] = true;
        while head < q.len() {
            let cur = q[head];
            head += 1;
            if let Some(come) = come_from.get(&cur) {
                for &v in come {
                    if !used[v as usize] {
                        used[v as usize] = true;
                        q.push(v);
                    }
                }
            }
            if let Some(change) = graph.get(&cur) {
                for &v in change {
                    if !used[v as usize] {
                        used[v as usize] = true;
                        q.push(v);
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}

struct Solution;