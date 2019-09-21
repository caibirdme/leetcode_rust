use std::collections::HashMap;

struct UnionTree {
    tree: HashMap<(i32,i32), (i32,i32)>,
}

impl UnionTree {
    fn new() -> Self {
        Self{
            tree: HashMap::new(),
        }
    }
    fn get_parent(&mut self, x: i32, y: i32) -> Option<(i32,i32)> {
        if let Some((a,b)) = self.tree.get(&(x,y)) {
            if *a == x && *b == y {
                return Some((*a,*b));
            }
            let (px,py) = self.get_parent(*a,*b).unwrap();
            let p = self.tree.get_mut(&(x,y)).unwrap();
            *p = (px,py);
            return Some((px,py));
        }
        None
    }
    fn set_parent(&mut self, child: (i32,i32), parent: (i32,i32)) {
        if let Some((px,py)) = self.get_parent(parent.0, parent.1) {
            self.tree.insert(child, (px,py));
        } else {
            self.tree.insert(child, parent);
        }
    }
    fn combine_parent(&mut self, a: (i32,i32), b: (i32,i32)) {
        let (ax,ay) = self.get_parent(a.0,a.1).unwrap();
        let (bx,by) = self.get_parent(b.0,b.1).unwrap();
        self.set_parent((ax,ay), (bx,by));
    }
}

impl Solution {
    pub fn num_islands2(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ut = UnionTree::new();
        let mut total = 0;
        let mut ans = vec![];
        let mut tmp: Vec<(i32,i32)> = vec![];
        let steps = [[0,1],[1,0],[0,-1],[-1,0]];
        for tuple in positions {
            let (x,y) = (tuple[0], tuple[1]);
            if let Some(_) = ut.get_parent(x,y) {
                ans.push(total);
                continue;
            }
            for step in steps.iter() {
                let (a,b) = (x+step[0], y+step[1]);
                if let Some((px,py)) = ut.get_parent(a,b) {
                    let mut ok = true;
                    for (vx,vy) in &tmp {
                        if *vx == px && *vy == py {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        tmp.push((px,py));
                    }
                }
            }
            if tmp.is_empty() {
                ut.set_parent((x,y),(x,y));
                total += 1;
            } else {
                for i in 1..tmp.len() {
                    ut.combine_parent((tmp[i-1].0, tmp[i-1].1), (tmp[i].0, tmp[i].1));
                    total -= 1;
                }
                ut.set_parent((x,y),(tmp[0].0,tmp[0].1));
            }
            ans.push(total);
            tmp.clear();
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_islands2() {
        let test_cases = vec![
            (3,3,vec![vec![0,0], vec![0,1], vec![1,2], vec![1,2]], vec![1,1,2,2]),
            (3,3,vec![vec![0,0], vec![0,1], vec![1,2], vec![2,1]], vec![1,1,2,3]),
            (3,3,vec![vec![0,0], vec![0,1], vec![1,2], vec![2,1], vec![0,2], vec![2,2]], vec![1,1,2,3,2,1]),
        ];
        for (m,n,positions,expect) in test_cases {
            assert_eq!(Solution::num_islands2(m,n,positions.clone()), expect, "m:{}, n:{}, positions: {:?}", m,n,positions);
        }
    }
}