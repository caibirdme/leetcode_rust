impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let m = nums.len();
        let mut count = 0;
        let mut idx = 0;
        let mut max = 1i64;
        while max <= n as i64 {
            if idx<m && nums[idx] as i64 <= max {
                max += nums[idx] as i64;
                idx+=1;
            } else {
                max *=2 ;
                count+=1;
            }
        }
        count
    }
}

struct Solution;