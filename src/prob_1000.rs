impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let n = stones.len();
        if n <= 1 {
            return 0;
        }
        if k == 1 {
            return -1;
        }
        if k as usize == n {
            return stones.iter().sum();
        }
        if !Self::can_merge(n as i32, k) {
            return -1;
        }
        let mut f: Vec<Vec<Vec<Option<i32>>>> = vec![vec![vec![None; k as usize+1]; n]; n];
        let mut sum = vec![0; n+1];
        for (i, &v) in stones.iter().enumerate() {
            sum[i+1] = sum[i]+v;
        }
        Self::dfs(&sum, &mut f, 0, n-1, 1, k as usize)
    }
    fn dfs(sum: &Vec<i32>, f: &mut Vec<Vec<Vec<Option<i32>>>>, l: usize, r: usize, tar: usize, k: usize) -> i32 {
        if l == r {
            if tar == 1 {
                return 0;
            }
            return std::i32::MAX;
        }
        if r-l+1 == tar {
            return 0;
        }
        if let Some(v) = f[l][r][tar] {
            return v;
        }
        if tar == 1 {
            if !Self::can_merge((r-l+1) as i32, k as i32) {
                return std::i32::MAX;
            }
            let ans = Self::dfs(sum, f, l, r, k, k)+sum[r+1]-sum[l];
            f[l][r][1] = Some(ans);
            return ans;
        }
        let mut result: i32 = std::i32::MAX;
        let mut i = l+tar-1-1;
        while i < r {
            let p1 = Self::dfs(sum ,f, l, i, tar-1, k);
            if p1 == std::i32::MAX {
                i+=1;
                continue;
            }
            let p2 = Self::dfs(sum, f, i+1, r, 1, k);
            if p2 == std::i32::MAX {
                i+=1;
                continue;
            }
            result = result.min(p1+p2);
            i+=1;
        }
        f[l][r][tar] = Some(result);
        result
    }
    // make sure k > 1
    #[inline]
    fn can_merge(n: i32, k: i32) -> bool {
        (n-1)%(k-1) == 0
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_merge_stones() {
        let test_cases = vec![
            (vec![1,1,3,4], 2, 16),
            (vec![3,2,4,1], 2, 20),
            (vec![3,2,4,1], 3, -1),
        ];
        for (stones, k, expect) in test_cases {
            assert_eq!(Solution::merge_stones(stones.clone(), k), expect, "stones: {:?}, k:{}", stones, k);
        }
    }
}