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
    pub fn fast_dp(mut nums: Vec<i32>) -> i32 {
        let mut arr = vec![1];
        arr.append(&mut nums);
        arr.push(1);
        let n = arr.len();
        let mut f = vec![vec![0; n]; n];
        for len in 2..n {
            for i in 0..n-len {
                let j = i+len;
                let t = arr[i]*arr[j];
                for k in i+1..j {
                    f[i][j] = f[i][j].max(f[i][k]+f[k][j]+t*arr[k]);
                }
            }
        }
        f[0][n-1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_max_coins() {
        let test_cases = vec![
            vec![2,3],
            vec![2,2,2],
            vec![3,1,5,8],
            vec![5,6,7,8,3,2,5,4,7,9,1,2,3,54,6,8,4,6,5,2,1,3,4,5,6,9,8,7,5,3,6],
        ];
        for nums in test_cases {
            assert_eq!(Solution::fast_dp(nums.clone()), Solution::max_coins(nums.clone()), "nums: {:?}", nums);
        }
    }
}