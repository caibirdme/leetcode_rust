struct DST {
    parent: Vec<usize>,
}

impl DST {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).into_iter().collect(),
        }
    }
    fn get_parent(&mut self, u: usize) -> usize {
        if self.parent[u] == u {
            return u;
        }
        let p = self.get_parent(self.parent[u]);
        self.parent[u] = p;
        p
    }
    fn union(&mut self, a: usize, b: usize) {
        let pa = self.get_parent(a);
        let pb = self.get_parent(b);
        self.parent[pa] = pb;
    }
}

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        if n == 0 {
            return false;
        }
        let mut dst = DST::new(n);
        for (i,keys) in rooms.into_iter().enumerate() {
            for j in keys {
                dst.union(i, j as usize);
            }
        }
        (0..n as usize).into_iter().fold(0, |pre, i| if dst.get_parent(i) == i { pre+1 } else {pre}) == 1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_can_visit_all_rooms() {
        let test_cases = vec![
            (vec![
                vec![1],vec![2],vec![3],vec![]
            ], true),
            (vec![
                vec![1,3],vec![3,0,1],vec![2],vec![0],
            ], false),
        ];
        for (rooms, ok) in test_cases {
            assert_eq!(Solution::can_visit_all_rooms(rooms.clone()), ok, "rooms: {:?}", rooms);
        }
    }
}