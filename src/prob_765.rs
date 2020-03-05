struct DST {
    parent: Vec<usize>,
    n: usize,
}

impl DST {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n {parent[i] = i;}
        Self{
            parent,n,
        }
    }
    fn get_parent(&mut self, n: usize) -> usize {
        if self.parent[n] == n {
            return n;
        }
        let p = self.get_parent(self.parent[n]);
        self.parent[n] = p;
        p
    }
    fn merge(&mut self, a: usize, b: usize) {
        let pa = self.get_parent(a);
        let pb = self.get_parent(b);
        self.parent[pa] = pb;
    }
}

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let n = row.len();
        if n < 3 {
            return 0;
        }
        let mut couches: Vec<Vec<usize>> = vec![vec![]; n/2];
        for i in 0..n {
            couches[(row[i]/2) as usize].push(i/2);
        }
        let mut dst = DST::new(n/2);
        for couch in couches {
            dst.merge(couch[0], couch[1]);
        }
        (n/2 - (0..n/2).into_iter().filter(|&v| dst.get_parent(v) == v).count()) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_swaps_couples() {
        let test_cases = vec![
            (vec![0,2,1,3], 1),
            (vec![3,2,0,1], 0),
            (vec![0,2,4,8,3,5,6,9,7,1], 4),
        ];
        for (row, expect) in test_cases {
            assert_eq!(Solution::min_swaps_couples(row.clone()), expect, "row: {:?}", row);
        }
    }
}