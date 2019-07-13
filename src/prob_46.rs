impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![];
        }
        let mut result = vec![];
        let n = nums.len();
        let mut used = vec![false; n];
        let mut current = vec![0usize; n];
        Self::back_track(0, n, &nums, &mut used, &mut current, &mut result);
        result
    }
    fn back_track(now: usize, n: usize, nums: &Vec<i32>, used: &mut Vec<bool>, current: &mut Vec<usize>, res: &mut Vec<Vec<i32>>) {
        if now >= n {
            let mut v = vec![];
            for (i,&val) in current.iter().enumerate() {
                v.push(nums[val]);
            }
            res.push(v);
            return;
        }
        for i in 0..n {
            if !used[i] {
                current[now] = i;
                used[i] = true;
                Self::back_track(now+1, n, nums, used, current, res);
                used[i] = false;
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_permute() {
        let res = Solution::permute(vec![1,3,5]);
        assert_eq!(res.len(), 6);
        assert_eq!(res, vec![
        vec![1,3,5],vec![1,5,3],vec![3,1,5], vec![3,5,1], vec![5,1,3], vec![5,3,1],
        ]);
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }
}