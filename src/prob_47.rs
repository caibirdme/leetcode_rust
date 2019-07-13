impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let n = nums.len();
        let mut used = vec![false; n];
        let mut current = vec![0usize; n];
        let mut result = vec![];
        Self::dfs(0, n, &nums, &mut used, &mut current, &mut result);
        result
    }
    fn dfs(step: usize, n: usize, nums: &Vec<i32>, used: &mut Vec<bool>, current: &mut Vec<usize>, result: &mut Vec<Vec<i32>>) {
        if step >= n {
            let mut v = vec![0; n];
            for i in 0..n {
                v[i] = nums[current[i]];
            }
            result.push(v);
            return;
        }
        for i in 0..n {
            if i>0 && nums[i] == nums[i-1] && !used[i-1] {
                continue;
            }
            if used[i] {
                continue;
            }
            used[i] = true;
            current[step] = i;
            Self::dfs(step+1, n, nums, used, current, result);
            used[i] = false;
        }
    }
}

struct Solution;


#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_permute_unique() {
        assert_eq!(Solution::permute_unique(vec![2,1,1]), vec![
        vec![1,1,2],vec![1,2,1], vec![2,1,1],
        ]);
        assert_eq!(Solution::permute_unique(vec![2,1]), vec![
            vec![1,2],vec![2,1],
        ]);
    }
}