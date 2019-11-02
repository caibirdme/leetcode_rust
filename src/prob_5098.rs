use std::collections::HashMap;
use std::mem;
use std::cmp::max;

struct TreeNode {
    ch_len: Vec<i32>,
}

impl TreeNode {
    fn new(cur: i32, graph: &HashMap<i32, Vec<i32>>, ans: &mut i32) -> Self {
        if !graph.contains_key(&cur) {
            return Self{
                ch_len: vec![],
            };
        }
        let arr = graph.get(&cur).unwrap();
        let mut ch_len = vec![];
        let mut max1 = None;
        let mut max2 = None;
        for v in arr.iter() {
            let ch = Self::new(*v, graph, ans);
            let max_len = match ch.ch_len.iter().max() {
                Some(l) => *l+1,
                None => 1,
            };
            if max1.is_none() {
                max1 = Some(max_len);
            } else if max_len > *max1.as_ref().unwrap() {
                max2 = max1.take();
                max1 = Some(max_len);
            } else if max2.is_none() {
                max2 = Some(max_len);
            } else if max_len > *max2.as_ref().unwrap() {
                max2 = Some(max_len);
            }
            ch_len.push(max_len);
        }
        *ans = max(*ans, max1.map_or(0, |v| v)+max2.map_or(0, |v| v)+1);
        Self{
            ch_len,
        }
    }
}

impl Solution {
    pub fn tree_diameter(edges: Vec<Vec<i32>>) -> i32 {
        if edges.is_empty() {
            return 0;
        }
        let mut graph = HashMap::new();
        let n = edges.len();
        for pairs in edges {
            let (mut a,mut b) = (pairs[0], pairs[1]);
            if a > b {
                mem::swap(&mut a, &mut b);
            }
            graph.entry(a).or_insert(vec![]).push(b);
        }
        let mut ans = 2;
        TreeNode::new(0, &graph, &mut ans);
        ans-1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_tree_diameter() {
        let test_cases = vec![
            (vec![vec![0,2], vec![2,1], vec![4,2], vec![2,3], vec![5,2], vec![6,3]], 3),
            (vec![vec![0,1],vec![1,2],vec![2,3],vec![1,4],vec![4,5]], 4),
            (vec![vec![0,1], vec![1,2]], 2),
        ];
        for (edges, expect) in test_cases {
            assert_eq!(Solution::tree_diameter(edges.clone()), expect, "edges: {:?}", edges);
        }
    }

    #[test]
    fn test_tree_diameter_1() {
        let test_cases = [[0,1],[0,2],[0,3],[2,4],[2,5],[2,6],[1,7],[0,8],[5,9],[9,10],[6,11],[8,12],[3,13],[3,14],[12,15],[0,16],[16,17],[3,18],[4,19],[9,20],[6,21],[8,22],[8,23],[0,24],[2,25],[4,26],[6,27],[13,28],[11,29],[27,30],[7,31],[27,32],[10,33],[5,34],[32,35],[23,36],[7,37],[30,38],[7,39],[24,40],[38,41],[29,42],[29,43],[16,44],[12,45],[0,46],[34,47],[34,48],[45,49],[42,50],[40,51],[34,52],[21,53],[35,54],[37,55],[1,56],[56,57],[7,58],[4,59],[6,60],[40,61],[61,62],[51,63],[30,64],[64,65],[44,66],[28,67],[57,68],[64,69],[42,70],[51,71],[27,72],[21,73],[32,74],[5,75],[47,76],[29,77],[8,78],[73,79],[7,80],[60,81],[12,82],[28,83],[42,84],[26,85],[40,86],[25,87],[84,88],[47,89],[82,90],[83,91],[86,92],[69,93],[34,94],[40,95],[52,96],[91,97]];
        let expect = 13;
        assert_eq!(Solution::tree_diameter(test_cases.iter().map(|v| vec![v[0], v[1]]).collect()), expect);
    }
}