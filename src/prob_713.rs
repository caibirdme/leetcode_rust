impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if k <= 1 || n == 0 {
            return 0;
        }
        let mut acc = 1;
        let mut ans = 0;
        let mut i = 0;
        for (j, &v) in nums.iter().enumerate() {
            acc *= v;
            while acc >= k {
                acc /= nums[i];
                i+=1;
            }
            ans += j as i32-i as i32+1;
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_subarray_product_less_than_k() {
        let test_cases = vec![
            (vec![1,1,1],1,0),
            (vec![1,1,1,1,1,1], 2, 21),
            (vec![1,1,1,100,100,2,5,10,1], 100, 14),
            (vec![1,1,1,100,100,2,5,8,1], 100, 16),
            (vec![10, 5, 2, 6], 100, 8),
        ];
        for (nums, k, expect) in test_cases {
            assert_eq!(Solution::num_subarray_product_less_than_k(nums.clone(), k), expect, "nums: {:?}, k: {}", nums, k);
        }
    }
}