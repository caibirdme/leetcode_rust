
struct DST {
    parent: Vec<usize>,
}

impl DST {
    fn new(n: usize) -> Self {
        let mut f = vec![0; n+1];
        for i in 1..=n {
            f[i] = i;
        }
        Self{
            parent: f,
        }
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
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut ind = vec![0; n+1];
        for edge in edges.iter() {
            let to = edge[1];
            ind[to as usize] += 1;
        }
        let mut target = 0;
        let mut big_circle = true;
        for i in 1..=n {
            if ind[i] == 2 {
                target = i;
                big_circle = false;
                break;
            }
        }
        if big_circle {
            let mut dst = DST::new(n);
            let mut ans = vec![];
            for edge in &edges {
                let (from, to) = (edge[0], edge[1]);
                if dst.get_parent(from as usize) == dst.get_parent(to as usize) {
                    ans = vec![from, to];
                } else {
                    dst.union(from as usize, to as usize);
                }
            }
            return ans;
        }
        for edge in edges.iter().rev() {
            let (from, to) = (edge[0], edge[1]);
            if to as usize == target {
                if !Self::is_circle_exist(&edges, from, to) {
                    return vec![from, to];
                } else {
                    for i in 0..n {
                        if edges[i][1] == to {
                            return vec![edges[i][0], edges[i][1]];
                        }
                    }
                }
            }
        }
        unreachable!()
    }
    fn is_circle_exist(edges: &Vec<Vec<i32>>, from_: i32, to_: i32) -> bool {
        let mut dst = DST::new(edges.len());
        for edge in edges {
            let (from, to) = (edge[0], edge[1]);
            if from == from_ && to == to_ {
                continue;
            }
            if dst.get_parent(from as usize) == dst.get_parent(to as usize) {
                return true;
            } else {
                dst.union(from as usize, to as usize);
            }
        }
        false
    }
}

struct Solution;