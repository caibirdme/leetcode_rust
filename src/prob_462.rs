impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = 0;
        nums.sort();
        let mut f = vec![0 as i64; n];
        for (i,&v) in nums.iter().enumerate() {
            sum += v as i64;
            f[i] = sum;
        }
        let n64 = n as i64;
        let mut ans: i64 = sum-f[0]-(n64-1)*(nums[0] as i64);
        for i in 1..n {
            let si = i as i64;
            let t = sum-f[i]+(2*si-n64+1)*(nums[i] as i64)-f[i-1];
            ans = std::cmp::min(ans, t);
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_min_moves2() {
        let test_cases = vec![
            (vec![203125577,-349566234,230332704,48321315,66379082,386516853,50986744,-250908656,-425653504,-212123143], 2127271182),
            (vec![1,0,0,8,6], 14),
            (vec![1,2,3], 2),
            (vec![1,2,3,4], 4),
        ];
        for (nums, expect) in test_cases {
            assert_eq!(Solution::min_moves2(nums.clone()), expect, "nums: {:?}", nums);
        }
    }
}