use std::collections::HashSet;

impl Solution {
    pub fn leads_to_destination(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut graph = vec![HashSet::new(); n as usize];
        for edge in edges {
            graph[edge[0] as usize].insert(edge[1] as usize);
        }
        if graph[destination as usize].len() > 0 {
            return false;
        }
        let mut visited = vec![false; n as usize];
        visited[source as usize] = true;
        Self::dfs(source as usize, destination as usize, &graph, &mut visited)
    }
    fn dfs(cur: usize, dest: usize, graph: &Vec<HashSet<usize>>, visited: &mut Vec<bool>) -> bool {
        if graph[cur].len() == 0 {
            return cur == dest;
        }
        for &next in graph[cur].iter() {
            if visited[next] {
                return false;
            }
            visited[next] = true;
            if !Self::dfs(next, dest, graph, visited) {return false;}
            visited[next] = false;
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_leads_to_destination() {
        let test_cases = vec![
            (4, vec![
                vec![0,1],vec![0,3],vec![1,2],vec![2,1]
            ],0,3,false),
            (4, vec![
                vec![0,1],vec![0,3],vec![1,3],vec![2,3]
            ],0,3,true),
            (2,vec![
                vec![0,1],vec![1,1],
            ],0,1,false),
            (3, vec![
                vec![0,1], vec![1,1], vec![1,2],
            ],0,2,false),
        ];
        for (n,edges,source,destination,ok) in test_cases {
            assert_eq!(Solution::leads_to_destination(n, edges.clone(), source, destination), ok, "n:{}, edges: {:?}, source: {}, dest: {}", n,edges,source,destination);
        }
    }
}