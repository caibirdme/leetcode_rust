impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        if n == 0 || target <= nums [0] {
            return 0;
        }
        let (mut l, mut r) = (0, n-1);
        while l<=r {
            let mid = (l+r) / 2;
            let v = nums[mid];
            if target == v {
                return mid as i32;
            }
            if target > v {
                l = mid + 1;
            } else {
                r = mid-1;
            }
        }
        l as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_search_insert() {
        let test_cases = vec![
            (vec![1,3,5,6], 5, 2),
            (vec![1,3,5,6], 2, 1),
            (vec![1,3,5,6], 7, 4),
            (vec![1,3,5,6], 0, 0),
        ];
        for (nums, target, expect) in test_cases {
            assert_eq!(Solution::search_insert(nums.clone(), target), expect, "nums: {:?}, target: {}", nums, target);
        }
    }
}