use std::cmp::{min, max};

impl Solution {
    pub fn minimum_moves(arr: Vec<i32>) -> i32 {
        if arr.is_empty() {
            return 0;
        }
        let n = arr.len();
        if n == 1 {
            return 1;
        }
        let mut f = vec![vec![0; n]; n];
        Self::dfs(0, n-1, &arr, &mut f)
    }
    fn dfs(l: usize, r: usize, arr: &Vec<i32>, f: &mut Vec<Vec<i32>>) -> i32 {
        if l > r {
            return 0;
        }
        if l == r {
            return 1;
        }
        if f[l][r] > 0 {
            return f[l][r];
        }
        let mut ans = (r-l+1) as i32;
        for i in l..=r {
            if l == i {
                ans = min(ans, 1+Self::dfs(i+1, r, arr, f));
            } else {
                if arr[l] == arr[i] {
                    ans = min(ans, max(Self::dfs(l+1, i-1, arr, f), 1)+Self::dfs(i+1, r, arr, f));
                } else {
                    ans = min(ans, Self::dfs(l+1, i-1, arr, f)+2+Self::dfs(i+1, r, arr, f));
                }
            }
        }
        f[l][r] = ans;
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_minimum_moves() {
        let test_cases = vec![
            (vec![1,2,3], 3),
            (vec![1,5,2,2,5], 2),
            (vec![1,5,2,5,1], 1),
            (vec![1,5,2,1], 2),
            (vec![1,5,1], 1),
            (vec![1,3,4,1,5], 3),
            (vec![1,2], 2),
        ];
        for (nums, expect) in test_cases {
            assert_eq!(Solution::minimum_moves(nums.clone()), expect, "arr: {:?}", nums);
        }
    }
}