struct DST {
    parent: Vec<usize>,
}

impl DST {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n+1];
        for i in 0..=n {parent[i] = i;}
        Self{ parent }
    }
    fn get_parent(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        let p = self.get_parent(self.parent[x]);
        self.parent[x] = p;
        p
    }
    fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.get_parent(x);
        let py = self.get_parent(y);
        if px == py {
            false
        } else {
            self.parent[py] = px;
            true
        }
    }
}

impl Solution {
    pub fn minimum_cost(n: i32, mut connections: Vec<Vec<i32>>) -> i32 {
        let mut dst = DST::new(n as usize);
        connections.sort_by(|a,b| a[2].cmp(&b[2]));
        let mut count = 0;
        for connect in connections {
            let (from, to, val) = (connect[0], connect[1], connect[2]);
            if dst.union(from as usize, to as usize) {
                count += val;
            }
        }
        if (1..=n as usize).into_iter().filter(|&v| dst.get_parent(v) == v).count() == 1 {
            count
        } else {
            -1
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_minimum_cost() {
        let test_cases = vec![
            (3, vec![
                vec![1,2,5], vec![1,3,6],vec![2,3,1],
            ], 6),
        ];
        for (n, connections, expect) in test_cases {
            assert_eq!(Solution::minimum_cost(n, connections.clone()), expect, "n: {}, connections: {:?}", n, connections);
        }
    }
}