impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut sorted = nums.clone();
        sorted.sort();
        let n = nums.len();
        let mut i = 0;
        while i < n && nums[i]== sorted[i] { i+=1; }
        if i == n {
            return 0;
        }
        let mut j = n-1;
        while j > 0 && nums[j] == sorted[j] { j-=1; }
        (j-i+1) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_unsorted_subarray() {
        let test_cases = vec![
            (vec![1,3,4,2,4], 3),
            (vec![2,3,4,5,1], 5),
            (vec![1,2,3,4,5], 0),
            (vec![2, 6, 4, 8, 10, 9, 15], 5),
        ];
        for (nums, expect) in test_cases {
            assert_eq!(Solution::find_unsorted_subarray(nums.clone()), expect, "nums: {:?}", nums);
        }
    }
}