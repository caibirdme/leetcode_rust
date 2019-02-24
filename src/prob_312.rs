impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }
        let mut new_nums = vec![1 as i64];
        for i in nums {
            new_nums.push(i as i64);
        }
        new_nums.push(1);
        let n = new_nums.len();
        let mut f = vec![vec![None; n]; n];
        Self::get_max(&new_nums, 0, n-1, &mut f) as i32
    }
    fn get_max(nums: &Vec<i64>, i:usize,j:usize, f: &mut Vec<Vec<Option<i64>>>) -> i64 {
        use std::cmp::max;
        if i+1 == j {
            return 0;
        }
        match f[i][j] {
            Some(v) => {return v;},
            None => {},
        }
        let t = nums[i] * nums[j];
        let mut ans = 0;
        for k in i+1..j {
            let l = Self::get_max(nums, i,k, f);
            let r = Self::get_max(nums,k,j,f);
            ans = max(ans, t*nums[k] + l + r);
        }
        f[i][j] = Some(ans);
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_max_coins() {
        assert_eq!(
            Solution::max_coins(vec![2,3]),
            9
        );
        assert_eq!(
            Solution::max_coins(vec![2,2,2]),
            14
        );
        assert_eq!(
            Solution::max_coins(vec![3,1,5,8]),
            167
        );
    }
}