impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        if n < 3 {
            return true;
        }
        let mut colors = vec![-1; n];
        for i in 0..n {
            if colors[i] == -1 {
                colors[i] = 0;
                if !Self::dfs(i, &graph, &mut colors) {
                    return false;
                }
            }
        }
        true
    }
    fn dfs(node: usize, graph: &Vec<Vec<i32>>, colors: &mut Vec<i8>) -> bool {
        let color = colors[node];
        let next_color = color ^ 1;
        for &next in graph[node].iter() {
            let u = next as usize;
            if colors[u] == color {
                return false;
            }
            if colors[u] == next_color {
                continue;
            }
            colors[u] = next_color;
            if !Self::dfs(u, graph, colors) {return false;}
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_bipartite() {
        let test_cases = vec![
            (vec![vec![1,3],vec![0,2],vec![1,3],vec![0,2]], true),
            (vec![vec![],vec![2,4,6],vec![1,4,8,9],vec![7,8],vec![1,2,8,9],vec![6,9],
                  vec![1,5,7,8,9],vec![3,6,9],vec![2,3,4,6,9],vec![2,4,5,6,7,8]], false),
        ];
        for (graph, ok) in test_cases {
            assert_eq!(Solution::is_bipartite(graph.clone()), ok, "graph: {:?}", graph);
        }
    }
}