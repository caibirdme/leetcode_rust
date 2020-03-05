impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        if n == 0 {
            return vec![];
        }
        let mut visited = vec![false; n];
        let mut ans = vec![None; n];
        for i in 0..n {
            if ans[i].is_some() {
                continue;
            }
            visited[i] = true;
            Self::dfs(i, &graph, &mut visited, &mut ans);
            visited[i] = false;
        }
        (0..n as i32).into_iter().filter(|&v| ans[v as usize].unwrap_or(true)).collect()
    }
    fn dfs(x: usize, graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>, ans: &mut Vec<Option<bool>>) -> bool {
        if graph[x].is_empty() {
            ans[x] = Some(true);
            return true;
        }
        if let Some(ok) = ans[x] {
            return ok;
        }
        for &next in graph[x].iter() {
            let u = next as usize;
            if let Some(ok) = ans[u] {
                if !ok {
                    ans[x] = Some(false);
                    return false;
                } else {
                    continue;
                }
            }
            if visited[u] {
                ans[x] = Some(false);
                return false;
            }
            visited[u] = true;
            if !Self::dfs(u, graph, visited, ans) {
                ans[x] = Some(false);
                return false;
            }
            visited[u] = false;
        }
        ans[x] = Some(true);
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_eventual_safe_nodes() {
        let test_cases = vec![
            (vec![
                vec![],
            ],vec![0]),
            (vec![
                vec![1],vec![2,3],vec![1],vec![4],vec![5],vec![],
            ], vec![3,4,5]),
            (vec![
                vec![1,2],vec![2,3],vec![5],vec![0],vec![5],vec![],vec![],
            ], vec![2,4,5,6]),
        ];
        for (graph, expect) in test_cases {
            assert_eq!(Solution::eventual_safe_nodes(graph.clone()), expect, "graph: {:?}", graph);
        }
    }
}