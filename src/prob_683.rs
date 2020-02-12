struct TreeNode {
    val: i32,
    lch: Option<Box<TreeNode>>,
    rch: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(n: usize) -> Self {
        Self::build(1, n)
    }
    fn build(l: usize, r: usize) -> TreeNode {
        if l == r {
            return TreeNode{
                val: 0,
                lch: None,
                rch: None,
            };
        }
        let mid = (l+r)/2;
        let lch = Self::build(l, mid);
        let rch = Self::build(mid+1, r);
        Self{
            val: lch.val+rch.val,
            lch: Some(Box::new(lch)),
            rch: Some(Box::new(rch)),
        }
    }
    pub fn add(&mut self, t: usize, ll: usize, rr: usize) {
        if ll == rr {
            self.val+=1;
            return;
        }
        let mid = (ll+rr)/2;
        if t <= mid {
            self.lch.as_mut().unwrap().add(t, ll, mid);
        } else {
            self.rch.as_mut().unwrap().add(t, mid+1, rr);
        }
        self.val+=1;
    }
    pub fn search(&self, l: usize, r: usize, ll: usize, rr: usize) -> i32 {
        if l == ll && r == rr {
            return self.val;
        }
        let mid = (ll+rr)/2;
        if r <= mid {
            self.lch.as_ref().unwrap().search(l,r,ll,mid)
        } else if l > mid {
            self.rch.as_ref().unwrap().search(l,r,mid+1,rr)
        } else {
            self.lch.as_ref().unwrap().search(l,mid,ll,mid) + self.rch.as_ref().unwrap().search(mid+1,r,mid+1,rr)
        }
    }
}

impl Solution {
    pub fn k_empty_slots(bulbs: Vec<i32>, k: i32) -> i32 {
        let n = bulbs.len();
        let uk = k as usize+2;
        if n < uk {
            return -1;
        }
        if n == 2 {
            return 2;
        }
        let mut open = vec![false; n+1];
        let mut root = TreeNode::new(n);
        open[bulbs[0] as usize] = true;
        root.add(bulbs[0] as usize, 1, n);
        for i in 1..n {
            let idx = bulbs[i] as usize;
            if idx>=uk && open[idx-uk+1] && root.search(idx-uk+1, idx, 1, n) == 1 {
                return i as i32+1;
            }
            if idx+uk-1 <= n && open[idx+uk-1] && root.search(idx, idx+uk-1, 1, n) == 1 {
                return i as i32+1;
            }
            root.add(idx, 1, n);
            open[idx] = true;
        }
        -1
    }
    pub fn window(bulbs: Vec<i32>, k: i32) -> i32 {
        let n = bulbs.len();
        let uk = k as usize;
        if n < uk+2 {
            return -1;
        }
        if n == 2 {
            return 2;
        }
        let mut flowers = vec![0; n];
        for i in 0..n {
            flowers[bulbs[i] as usize-1] = i+1;
        }
        let (mut left, mut right) = (0, uk+1);
        const INF: usize = 999999;
        let mut ans = INF;
        while right < n {
            let mut break_flag = false;
            for i in left+1..right {
                let cur = flowers[i];
                if cur < flowers[left] || cur < flowers[right] {
                    left = i;
                    right = i + uk + 1;
                    break_flag = true;
                    break;
                }
            }
            if !break_flag {
                ans = ans.min(flowers[left].max(flowers[right]));
                left = right;
                right = left+uk+1;
            }
        }
        if ans == 999999 {
            -1
        } else {
            ans as i32
        }
    }
    pub fn check(bulbs: Vec<i32>, k: i32) -> i32 {
        let n = bulbs.len();
        let uk = k as usize+2;
        if n < uk {
            return -1;
        }
        if n == 2 {
            return 2;
        }
        let mut f = vec![false; n+1];
        f[bulbs[0] as usize] = true;
        for i in 1..n {
            let idx = bulbs[i] as usize;
            f[idx] = true;
            if idx >= uk && f[idx+1-uk] {
                let mut ok = true;
                for j in idx+2-uk..idx {
                    if f[j] {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    return i as i32+1;
                }
            }
            if idx+uk-1 <= n && f[idx+uk-1]{
                let mut ok = true;
                for j in idx+1..idx+uk-1 {
                    if f[j] {
                        ok =false;
                        break;
                    }
                }
                if ok {
                    return i as i32+1;
                }
            }
        }
        -1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_k_empty_slots() {
        let test_cases = vec![
            (vec![6,5,8,9,7,1,10,2,3,4], 2, 8),
            (vec![1,2,3,4,5,7,6], 1, 6),
            (vec![2,1,4,3], 1, 3),
            (vec![1,3,2], 1, 2),
            (vec![1,2,3], 1, -1),
        ];
        for (bulbs, k, expect) in test_cases {
            assert_eq!(Solution::k_empty_slots(bulbs.clone(), k), Solution::window(bulbs.clone(), k), "bulbs:{:?}, k:{}", bulbs, k);
        }
    }
}