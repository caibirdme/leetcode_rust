use std::cmp::{Ordering, min, max};

impl Solution {
    pub fn max_number_dfs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let (nx,ny) = (nums1.len(), nums2.len());
        if nx == 0 {
            let mut t = nums2.clone();
            t.truncate(k as usize);
            return t;
        }
        if ny == 0 {
            let mut t = nums1.clone();
            t.truncate(k as usize);
            return t;
        }
        let x_tree = LineTree::new(0, nx-1, &nums1);
        let y_tree = LineTree::new(0, ny-1, &nums2);
        Self::dfs(&x_tree, 0, nx-1, &y_tree, 0, ny-1, k).data
    }
    fn dfs(x_tree: &LineTree, x:usize, x_n: usize, y_tree: &LineTree, y:usize, y_n:usize, k: i32) -> List {
        let options = Self::choose_idx(x_tree, x, x_n, x_n, y_tree, y,y_n,y_n, k);
        if k == 1 {
            return List::new(options[0].0);
        }
        let mut cur = List::new(options[0].0);
        let mut res = List::empty();
        for (num, idx, is_x) in options {
            let t = {
                if is_x {
                    Self::dfs(x_tree, idx+1, x_n, y_tree, y,y_n,k-1)
                } else {
                    Self::dfs(x_tree,x,x_n, y_tree, idx+1, y_n, k-1)
                }
            };
            if res < t {
                res = t;
            }
        }
        cur.push(&mut res);
        cur
    }
    fn choose_idx(x_tree: &LineTree, x_a:usize, x_b:usize,x_n:usize, y_tree: &LineTree, y_a:usize,y_b:usize, y_n:usize, k: i32) -> Vec<(i32, usize, bool)> {
        if x_a > x_b {
            let (max_y, y_idx) = Self::choose_idx_from(y_tree, y_a, y_b, y_n, k);
            return vec![(max_y, y_idx, false)];
        } else if y_a>y_b {
            let (max_x, x_idx) = Self::choose_idx_from(x_tree, x_a, x_b, x_n, k);
            return vec![(max_x, x_idx, true)];
        }
        let (max_x, x_idx) = x_tree.get_max(x_a,x_b);
        let (max_y, y_idx) = y_tree.get_max(y_a, y_b);
        let rest_length_x = x_n-x_idx+y_n-y_a+2;
        let rest_length_y = y_n-y_idx+x_n-x_a+2;
        let uk = k as usize;
        if max_x > max_y && rest_length_x >= uk {
            vec![(max_x, x_idx, true)]
        } else if max_y>max_x && rest_length_y >= uk {
            vec![(max_y, y_idx, false)]
        } else {
            if max_x == max_y && rest_length_x >= uk && rest_length_y < uk {
                vec![(max_x, x_idx, true)]
            } else if max_x == max_y && rest_length_y >= uk && rest_length_x < uk{
                vec![(max_y, y_idx, false)]
            } else if  max_x == max_y && rest_length_x >= uk && rest_length_y >= uk {
                vec![(max_x, x_idx, true), (max_y, y_idx, false)]
            } else {
                let mut reduce_x = {
                    if x_idx == x_a {
                        vec![(max_x, x_idx, true)]
                    } else {
                        Self::choose_idx(x_tree, x_a, x_idx-1, x_n, y_tree,y_a,y_b,y_n,k)
                    }
                };
                let mut reduce_y = {
                    if y_idx == y_a {
                        vec![(max_y, y_idx, false)]
                    } else {
                        Self::choose_idx(x_tree, x_a,x_b,x_n,y_tree, y_a,y_idx-1,y_n,k)
                    }
                };
                if reduce_x[0].0 > reduce_y[0].0 {
                    reduce_x
                } else if reduce_x[0].0 < reduce_y[0].0 {
                    reduce_y
                } else {
                    for (y_v, q_idx, upper) in reduce_y {
                        let mut not_found = true;
                        for (_, p_idx, p_upper) in reduce_x.iter() {
                            if *p_idx == q_idx && *p_upper == upper {
                                not_found = false;
                            }
                        }
                        if not_found {
                            reduce_x.push((y_v, q_idx, upper));
                        }
                    }
                    reduce_x
                }
            }
        }
    }
    fn choose_idx_from(tree: &LineTree, a:usize, b: usize, n: usize, k: i32) -> (i32, usize) {
        let (val,idx) = tree.get_max(a,b);
        if n-idx+1 >= k as usize {
            (val,idx)
        } else {
            Self::choose_idx_from(tree, a, idx-1, n, k)
        }
    }
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let (nx,ny) = (nums1.len(), nums2.len());
        if nx == 0 {
            let mut t = nums2.clone();
            t.truncate(k as usize);
            return t;
        }
        if ny == 0 {
            let mut t = nums1.clone();
            t.truncate(k as usize);
            return t;
        }
        let uk = k as usize;
        let mut ans: Vec<i32> = Vec::with_capacity(uk);
        let mut cur = Vec::with_capacity(uk);
        for i in 0..=nx {
            if i > uk {
                break;
            }
            let j = uk-i;
            if j > ny {
                continue;
            }
            let upper = Self::get_max_stack(&nums1, i);
            let lower = Self::get_max_stack(&nums2, j);
            let (mut x, mut y) = (0,0);
            while x<i || y<j {
                if Self::larger_than(&upper, &lower, x,y) {
                    cur.push(upper[x]);
                    x+=1;
                } else {
                    cur.push(lower[y]);
                    y+=1;
                }
            }
            if Self::should_update(&ans, &cur) {
                ans = cur.clone();
            }
            cur.clear();
        }
        ans
    }
    fn larger_than(num1: &Vec<i32>, num2: &Vec<i32>, mut x:usize, mut y:usize) -> bool {
        let (n1,n2) = (num1.len(), num2.len());
        while x<n1 && y < n2 && num1[x]==num2[y] {x+=1; y+=1;}
        y==n2 || (x<n1 && num1[x]>num2[y])
    }

    fn should_update(num1: &Vec<i32>, num2: &Vec<i32>) -> bool {
        if num1.len() == 0 {
            return true;
        }
        for (&a,&b) in num1.iter().zip(num2.iter()) {
            if a < b {
                return true;
            } else if a>b {
                return false;
            }
        }
        false
    }
    fn get_max_stack(nums: &Vec<i32>, k: usize) -> Vec<i32> {
        if k == 0 {
            return vec![];
        }
        let n = nums.len();
        let mut ans: Vec<i32> = vec![];
        for (i,&val) in nums.iter().enumerate() {
            loop {
                if ans.is_empty() {
                    break;
                }
                let top = *ans.last().unwrap();
                if val > top && ans.len()-1+n-i >= k {
                    ans.pop();
                } else {
                    break;
                }
            }
            if ans.len() < k {
                ans.push(val);
            }
        }
        ans
    }

    pub fn max_number_dp(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums1.len();
        let m = nums2.len();
        let uk = k as usize;
        let up = Self::pick_from(&nums1, uk);
        let down = Self::pick_from(&nums2, uk);
        let mut ans = vec![0; uk];
        let mut cur = Vec::with_capacity(uk);
        let mut p1 = vec![];
        let mut p2 = vec![];
        let h1 = Self::get_count_map(&nums1);
        let h2 = Self::get_count_map(&nums2);
        for i in 0..=min(uk, nums1.len()) {
            let j = uk-i;
            if j > nums2.len() {
                continue;
            }
            Self::get_from(&up, &h1,i, &mut p1);
            Self::get_from(&down, &h2, j, &mut p2);
            Self::merge_two_slice(&p1, &p2, &mut cur);
            if Self::cmp(&cur, &ans) {
                ans = cur.clone();
            }
        }
        ans
    }
    fn get_count_map(a: &[i32]) -> Vec<Vec<usize>> {
        let mut h = vec![vec![]; 10];
        for (i,&v) in a.iter().enumerate() {
            h[v as usize].push(i);
        }
        h
    }
    fn cmp(a: &[i32], b: &[i32]) -> bool {
        for (&x,&y) in a.iter().zip(b) {
            if x == y {
                continue;
            }
            return x > y;
        }
        a.len() > b.len()
    }
    fn get_from(arr: &Vec<Vec<i32>>, mp: &Vec<Vec<usize>>, k: usize, q: &mut Vec<i32>) {
        q.clear();
        if k == 0 {
            return;
        }
        let mut i = 0;
        let mut rk = k;
        let mut pos = [0; 10];
        while q.len() < k {
            let c = arr[rk][i];
            q.push(c);
            let uc = c as usize;
            let record = &mp[uc];
            while record[pos[uc]] < i {
                pos[uc]+=1;
            }
            i = record[pos[uc]]+1;
            pos[uc]+=1;
            rk -= 1;
        }
    }
    fn merge_two_slice(up: &Vec<i32>, down: &Vec<i32>, q: &mut Vec<i32>) {
        let n = up.len();
        let m = down.len();
        q.clear();
        let mut i = 0;
        let mut j = 0;
        while i < n && j < m {
            if up[i] > down[j] {
                q.push(up[i]);
                i+=1;
            } else if up[i] < down[j] {
                q.push(down[j]);
                j+=1;
            } else if Self::cmp(&up[i..], &down[j..]) {
                q.push(up[i]);
                i+=1;
            } else {
                q.push(down[j]);
                j+=1;
            }
        }
        while i < n {
            q.push(up[i]);
            i+=1;
        }
        while j<m {
            q.push(down[j]);
            j+=1;
        }
    }
    fn pick_from(nums: &[i32], k: usize) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n == 0 {
            return vec![];
        }
        let mut f = vec![vec![0; n+1]; min(n,k)+1];
        for i in 1..=min(k, n) {
            f[i][n-i] = nums[n-i];
            for j in (0..n-i).rev() {
                f[i][j] = max(f[i][j+1], nums[j]);
            }
        }
        f
    }
}



struct LineTree {
    val: i32,
    idx: usize,
    l_bound: usize,
    r_bound: usize,
    lch: Option<Box<LineTree>>,
    rch: Option<Box<LineTree>>,
}

impl LineTree {
    pub fn new(l:usize, r:usize, nums: &Vec<i32>) -> Self {
        if l == r {
            return Self{
                val: nums[l],
                idx: l,
                l_bound: l,
                r_bound: l,
                lch: None,
                rch: None,
            };
        }
        let mid = (l+r) >> 1;
        let lch = Self::new(l,mid, nums);
        let rch = Self::new(mid+1, r, nums);
        let mut val = lch.val;
        let mut idx = lch.idx;
        if rch.val > val {
            val = rch.val;
            idx = rch.idx;
        }
        Self {
            val,
            idx,
            l_bound: l,
            r_bound: r,
            lch: Some(Box::new(lch)),
            rch: Some(Box::new(rch)),
        }
    }

    pub fn get_max(&self, l:usize, r:usize) -> (i32, usize) {
        if l == self.l_bound && r == self.r_bound {
            return (self.val,self.idx);
        }
        let mid = (self.l_bound+self.r_bound)>>1;
        if r<=mid {
            self.lch.as_ref().unwrap().get_max(l, r)
        } else if l > mid {
            self.rch.as_ref().unwrap().get_max(l, r)
        } else {
            let (l_max, l_idx) = self.lch.as_ref().unwrap().get_max(l, mid);
            let (r_max, r_idx) = self.rch.as_ref().unwrap().get_max(mid+1, r);
            if l_max>=r_max {
                (l_max,l_idx)
            } else {
                (r_max,r_idx)
            }
        }
    }
}

struct List {
    data: Vec<i32>,
}

impl List {
    pub fn empty() -> Self {
        Self{
            data: vec![],
        }
    }
    pub fn new(val: i32) -> Self {
        Self{
            data: vec![val],
        }
    }
    pub fn push(&mut self, other: &mut Self) {
        self.data.append(&mut other.data);
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }
        for (&a,&b) in self.data.iter().zip(other.data.iter()) {
            if a != b {
                return false;
            }
        }
        true
    }
}

impl Eq for List {}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        use std::cmp::min;
        let (n1,n2) = (self.data.len(), other.data.len());
        if n1 == 0 {
            if n2 == 0 {
                return Ordering::Equal;
            }
            return Ordering::Less;
        }
        if n2 == 0 {
            return Ordering::Greater;
        }
        let min_length = min(n1,n2);
        for i in 0..min_length {
            match self.data[i].cmp(&other.data[i]) {
                Ordering::Less => {return Ordering::Less;},
                Ordering::Greater => {return Ordering::Greater;},
                _ => {},
            }
        }
        n1.cmp(&n2)
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_max_number() {
        let test_cases = vec![
            (vec![6,7], vec![6,0,4], 5, vec![6,7,6,0,4]),
            (vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5, vec![9, 8, 6, 5, 3]),
            (vec![3,9],vec![8,9],3, vec![9,8,9]),
        ];
        for (nums1, nums2, k, expect) in test_cases {
            let actual = Solution::max_number(nums1, nums2, k);
            assert_eq!(actual, expect);
        }
    }
    #[test]
    fn test_max_number_dp() {
        let test_cases = vec![
            (
                vec![3,9,1,0,0,5,6,3,7,3,1,6,1,9,5,4,3,6,4,0,8,8,2,8],
                vec![1,8,7,9,7,8,0,1,4,7,5,6,9,9,8,7,1,1,4,2,5,5,0,0,8,0,6,5,4,1,2,3,3],
                57,
                vec![3,9,1,8,7,9,7,8,1,0,1,4,7,5,6,9,9,8,7,1,1,4,2,5,5,0,0,8,0,6,5,4,1,2,3,3,0,0,5,6,3,7,3,1,6,1,9,5,4,3,6,4,0,8,8,2,8],
            ),
            (
                vec![5,0,2,1,0,1,0,3,9,1,2,8,0,9,8,1,4,7,3],
                vec![7,6,7,1,0,1,0,5,6,0,5,0],
                31,
                vec![7,6,7,5,1,0,2,1,0,1,0,5,6,0,5,0,1,0,3,9,1,2,8,0,9,8,1,4,7,3,0],
            ),
            (vec![6,7], vec![6,0,4], 5, vec![6,7,6,0,4]),
            (vec![3,1],vec![1,4,2], 4, vec![4,3,2,1]),
            (vec![3,6,4], vec![9,1,5], 4, vec![9,6,5,4]),
            (vec![3,1],vec![4,2], 4, vec![4,3,2,1]),
            (vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5, vec![9, 8, 6, 5, 3]),
            (vec![3,4,5], vec![9,1], 3, vec![9,5,1]),
            (vec![3,9],vec![8,9],3, vec![9,8,9]),
        ];
        for (nums1, nums2, k, expect) in test_cases {
            let actual = Solution::max_number_dp(nums1.clone(), nums2.clone(), k);
            assert_eq!(actual, expect, "s1: {:?}, s2: {:?}, k:{}", nums1,nums2,k);
        }
    }
}