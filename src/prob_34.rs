impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return vec![-1, -1];
        }
        let (mut l, mut r) = (0, n-1);
        while l < r {
            let m = (l+r) / 2;
            let v = nums[m];
            if v < target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        if nums[l] != target {
            return vec![-1, -1];
        }
        let st = l as i32;
        while l < n && nums[l] == target {
            l+=1;
        }
        vec![st, (l-1) as i32]
    }
}

struct Solution;


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_search_range() {
        let test_cases = vec![
            (vec![5,7,7,8,8,10], 8, vec![3,4]),
            (vec![5,7,7,8,8,10], 6, vec![-1, -1]),
            (vec![1,2,3,4,5,8,10], 5, vec![4,4]),
            (vec![1,1,1,1,2,2,3,4,5], 1, vec![0,3]),
        ];
        for (nums, target, expect) in test_cases {
            assert_eq!(Solution::search_range(nums.clone(), target), expect, "nums: {:?}, target: {}", nums, target);
        }
    }
}