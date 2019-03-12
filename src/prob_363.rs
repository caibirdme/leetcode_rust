impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::cmp::max;
        use std::i32;
        let mut ans = i32::MIN;
        let n = matrix.len();
        if n == 0 {
            return 0;
        }
        let m = matrix.first().unwrap().len();
        if m == 0 {
            return 0;
        }
        let mut sum = vec![0; n+1];
        for i in 0..m {
            sum.iter_mut().for_each(|v| *v = 0);
            for j in i..m {
                let mut acc = 0;
                for t in 0..n {
                    acc += matrix[t][j];
                    sum[t+1] += acc;
                }
                let mut tree = TreeNode::new(0);
                for t in 0..n {
                    let cur = sum[t+1];
                    let target = cur-k;
                    match tree.upper_bound(target) {
                        Some(v) => {ans = max(ans, cur-v);},
                        None => {if cur <= k && cur>ans {ans = cur;}},
                    }
                    tree.insert(cur);
                }
            }
        }
        ans
    }
}

struct TreeNode {
    val: i32,
    lch: Option<Box<TreeNode>>,
    rch: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            lch: None,
            rch: None,
        }
    }
    pub fn insert(&mut self, val: i32) {
        if val < self.val {
            match self.lch.take() {
                Some(mut lch) => {
                    lch.insert(val);
                    self.lch = Some(lch);
                },
                None => {self.lch = Some(Box::new(Self::new(val)));},
            }
        } else {
            match self.rch.take() {
                Some(mut rch) => {
                    rch.insert(val);
                    self.rch = Some(rch);
                },
                None => {self.rch = Some(Box::new(Self::new(val)));},
            }
        }
    }
    pub fn upper_bound(&self, val: i32) -> Option<i32> {
        if val == self.val {
            Some(val)
        } else if val < self.val {
            match self.lch.as_ref() {
                Some(lch) => {
                    let res = lch.upper_bound(val);
                    if res.is_some() {
                        res
                    } else {
                        Some(self.val)
                    }
                },
                None => Some(self.val),
            }
        } else {
            match self.rch.as_ref() {
                Some(rch) => rch.upper_bound(val),
                None => None,
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_max_sum_submatrix() {
        let test_cases = vec![
            (vec![
                vec![2,2,-1],
            ], 0, -1),
            (vec![
                vec![3,2,-1],
            ], 1, 1),
            (vec![
                vec![1,0,1],
                vec![0,-2,3]
            ], 5, 4),
            (vec![
                vec![1,0,1],
                vec![0,-2,3]
            ], 2, 2),
            (vec![
                vec![8],
            ], 9, 8),
            (vec![
                vec![3,2],
            ], 4, 3),
        ];
        for (matrix,k,expect) in test_cases {
            assert_eq!(
                expect,
                Solution::max_sum_submatrix(matrix.clone(), k),
                "matrix: {:?}, k: {}", matrix, k
            );
        }
    }
}