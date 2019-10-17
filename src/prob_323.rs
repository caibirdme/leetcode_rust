use std::collections::HashMap;

struct UnionTree {
    parent: HashMap<i32, i32>,
}

impl UnionTree {
    fn new(n: i32) -> Self {
        let mut t = HashMap::new();
        for i in 0..n {
            t.insert(i, i);
        }
        Self{
            parent: t,
        }
    }

    fn get_father(&mut self, node: i32) -> i32 {
        if let Some(&p) = self.parent.get(&node) {
            if p == node {
                return node;
            }
            let parent = self.get_father(p);
            let pointer = self.parent.get_mut(&node).unwrap();
            *pointer = parent;
            return parent;
        }
        unreachable!()
    }

    fn merge(&mut self, a: i32, b: i32) {
        let pa = self.get_father(a);
        let pb = self.get_father(b);
        let pointer = self.parent.get_mut(&pa).unwrap();
        *pointer = pb;
    }
}


impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 || edges.is_empty() {
            return n;
        }
        let mut t = UnionTree::new(n);
        for edge in edges {
            let (x,y) = (edge[0], edge[1]);
            t.merge(x,y);
        }
        let mut count = 0;
        for i in 0..n {
            if t.get_father(i) == i {
                count += 1;
            }
        }
        count
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_components() {
        let test_cases = vec![
            (
                5,
                vec![
                    vec![0,1],
                    vec![1,2],
                    vec![2,3],
                    vec![3,4],
                ],
                1,
            ),
            (
                5,
                vec![
                    vec![0,1],
                    vec![1,2],
                    vec![3,4],
                ],
                2,
            ),
        ];
        for (n, edges, expect) in test_cases {
            assert_eq!(Solution::count_components(n,edges.clone()), expect, "n: {}, edges: {:?}",n ,edges);
        }
    }
}