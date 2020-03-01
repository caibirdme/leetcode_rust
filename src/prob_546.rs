use std::collections::HashMap;
use std::ops::Add;

impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let mut f = vec![vec![vec![0; 101]; 101]; 101];
        Self::search(&boxes, 0, boxes.len()-1, 0, &mut f)
    }
    fn search(boxes: &Vec<i32>, l: usize, r: usize, k: usize, f: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if l > r {
            return 0;
        }
        if l == r {
            return ((k+1)*(k+1)) as i32;
        }
        if f[l][r][k] != 0 {
            return f[l][r][k];
        }
        let mut ans = ((1+k)*(1+k)) as i32 + Self::search(boxes, l+1, r, 0, f);
        for i in l+1..=r {
            if boxes[l] == boxes[i] {
                ans = ans.max(
                    Self::search(boxes, l+1, i-1, 0, f).add(
                        Self::search(boxes, i, r, k+1, f)
                    )
                );
            }
        }
        f[l][r][k] = ans;
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_remove_boxes() {
        let test_cases = vec![
            (vec![1, 3, 2, 2, 2, 3, 4, 3, 1], 23),
        ];
        for (boxes, expect) in test_cases {
            assert_eq!(Solution::remove_boxes(boxes.clone()), expect, "boxes: {:?}", boxes);
        }
    }
}