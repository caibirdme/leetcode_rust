use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn check_if_prerequisite(n: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut graph = HashMap::new();
        for v in prerequisites.iter() {
            let (to, from) = (v[0], v[1]);
            graph.entry(from as usize).or_insert(HashSet::new()).insert(to as usize);
        }
        let mut father = HashMap::new();
        for i in 0..n as usize {
            let f = Self::get_father(n as usize, i, &graph);
            father.insert(i, f);
        }
        let mut ans = vec![];
        for q in queries {
            let (to, from) = (q[0] as usize, q[1] as usize);
            if let Some(set) = father.get(&from) {
                ans.push(set.contains(&to));
            }
        }
        ans
    }
    fn get_father(n: usize, s: usize, graph: &HashMap<usize, HashSet<usize>>) -> HashSet<usize> {
        let mut went = vec![false; n];
        went[s] = true;
        let mut ans = HashSet::new();
        let mut q = vec![s];
        let mut head = 0;
        while head < q.len() {
            let cur = q[head];
            head += 1;
            if let Some(set) = graph.get(&cur) {
                for &next in set.iter() {
                    if !went[next] {
                        went[next] = true;
                        ans.insert(next);
                        q.push(next);
                    }
                }
            }
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_check_if_prerequisite() {
        let test_cases = vec![
            (3, vec![[1,2],[1,0],[2,0]], vec![[1,0],[1,2]], vec![true, true]),
            (5, vec![[0,1],[1,2],[2,3],[3,4]], vec![[0,4],[4,0],[1,3],[3,0]], vec![true,false,true,false]),
        ];
        for (n, pre, q, expect) in test_cases {
            let pres = pre.iter().map(|v| vec![v[0], v[1]]).collect();
            let qs = q.iter().map(|v| vec![v[0], v[1]]).collect();
            assert_eq!(Solution::check_if_prerequisite(n, pres, qs), expect, "n:{}, pres: {:?}, qs: {:?}", n, pre, q);
        }
    }
}