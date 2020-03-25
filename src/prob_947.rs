use std::collections::HashSet;

struct DSU {
    parent: Vec<usize>
}

impl DSU {
    fn new(n: usize, step: usize) -> Self {
        let mut parent = vec![0; n+step];
        for i in 0..n+step {
            parent[i] = i;
        }
        Self{
            parent,
        }
    }
    fn get_parent(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        let p = self.get_parent(self.parent[x]);
        self.parent[x] = p;
        p
    }
    fn union(&mut self, x: usize, y: usize) {
        let px = self.get_parent(x);
        let py = self.get_parent(y);
        self.parent[py] = px;
    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        if n == 1 {
            return 0;
        }
        let mut t = DSU::new(10000, 10000);
        for stone in &stones {
            let (x,y) = (stone[0], stone[1]+10000);
            t.union(x as usize,y as usize);
        }
        let mut set = HashSet::new();
        for stone in &stones {
            let (x,y) = (stone[0] as usize, stone[1] as usize+10000);
            if t.get_parent(x) == x {
                set.insert(x);
            } else if t.get_parent(y) == y {
                set.insert(y);
            }
        }
        (n-set.len()) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_remove_stones() {
        let test_cases = vec![
            (vec![[0,1],[1,0]], 0),
            (vec![[0,0]], 0),
            (vec![[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]], 5),
            (vec![[0,0],[0,2],[1,1],[2,0],[2,2]], 3),
        ];
        for (stones, expect) in test_cases {
            assert_eq!(Solution::remove_stones(stones.iter().map(|v| vec![v[0],v[1]]).collect()), expect, "stones: {:?}", stones);
        }
    }
}