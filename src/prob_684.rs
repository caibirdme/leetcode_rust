struct DST {
    parent: Vec<usize>
}

impl DST {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n+1];
        for i in 1..=n {
            parent[i] = i;
        }
        Self{ parent }
    }
    fn get_parent(&mut self, node: usize) -> usize {
        if self.parent[node] == node {
            return node;
        }
        let p = self.get_parent(self.parent[node]);
        self.parent[node] = p;
        p
    }
    fn union(&mut self, a: usize, b: usize) {
        let pa = self.get_parent(a);
        let pb = self.get_parent(b);
        self.parent[pa] = pb;
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut dst = DST::new(n);
        let mut ans = vec![];
        for i in 0..n {
            let (a,b) = (edges[i][0], edges[i][1]);
            if dst.get_parent(a as usize) == dst.get_parent(b as usize) {
                ans = vec![a,b];
            } else {
                dst.union(a as usize,b as usize);
            }
        }
        ans
    }
}

struct Solution;