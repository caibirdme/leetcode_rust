use std::cmp::min;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n <= 2 {
            return true;
        }
        if n % 2 == 0 {
            return true;
        }
        let mut sum = vec![0; n+1];
        for i in 0..n {
            sum[i+1] = sum[i]+nums[i];
        }
        let mut f = vec![vec![0; n+1]; n+1];
        let ans = Self::dfs(&sum, &mut f, 1, n);
        if sum[n] % 2 == 0 {
            ans >= sum[n] / 2
        } else {
            ans > sum[n] / 2
        }
    }
    fn dfs(sum: &Vec<i32>, f: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
        if x == y {
            return sum[y]-sum[y-1];
        }
        if f[x][y] != 0 {
            return f[x][y];
        }
        f[x][y] = sum[y]-sum[x-1] - min(Self::dfs(sum, f, x+1, y), Self::dfs(sum, f, x, y-1));
        f[x][y]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_predict_the_winner() {
        let test_cases = vec![
            (vec![1], true),
            (vec![1,3], true),
            (vec![1,1,2], true),
            (vec![1,5,1], false),
            (vec![1,5,2], false),
            (vec![1,5,3,1], true),
            (vec![1,5,233,7], true),
        ];
        for (nums, ok) in test_cases {
            assert_eq!(Solution::predict_the_winner(nums.clone()), ok, "nums: {:?}", nums);
        }
    }
}