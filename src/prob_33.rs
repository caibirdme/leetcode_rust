impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let n = nums.len()-1;
        if n == 0 {
            if nums[0] == target {
                return 0;
            }
            return -1;
        }
        // no rotate
        if nums[0] < nums[n] {
            match nums.binary_search(&target) {
                Ok(p) => return p as i32,
                _ => return -1,
            }
        }
        let pos = Self::find_pos(&nums);
        if target < nums[pos] {
            return -1;
        }
        if target <= nums[n] {
            match nums[pos..].binary_search(&target) {
                Ok(p) => return (pos+p) as i32,
                _ => return -1,
            }
        } else {
            match nums[..pos].binary_search(&target) {
                Ok(p) => return p as i32,
                _ => return -1,
            }
        }
        unreachable!("nums: {:?}", nums);
    }

    fn find_pos(nums: &Vec<i32>) -> usize {
        let n = nums.len() - 1;
        if n == 1 {
            return 1;
        }
        let (mut l, mut r) = (0, n);
        while l<=r {
            let mid = (l+r) / 2;
            let v = nums[mid];
            if mid > 0 && v < nums[mid-1] {
                return mid;
            }
            if nums[0] > v {
                r = mid-1;
            } else {
                l = mid+1;
            }
        }
        l
    }
}

struct Solution;


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_search() {
        let test_cases = vec![
            (vec![5,1,2,3,4], 4, 4),
            (vec![3,5,1], 0, -1),
            (vec![1,2], 1, 0),
            (vec![1,2], 2, 1),
            (vec![1,2], 0, -1),
            (vec![1,2], 3, -1),
            (vec![4,5,6,7,0,1,2], 0, 4),
            (vec![4,5,6,7,0,1,2], 7, 3),
            (vec![4,5,6,7,0,1,2], 4, 0),
            (vec![6,7,1,2,3,4,5], 4, 5),
            (vec![2,1], 1, 1),
            (vec![4,5,6,7,8,1,2,3], 8, 4),
            (vec![4,5,6,7,8,1,2,3], 9, -1),
            (vec![5,1,3], 5, 0),
            (vec![2,1], 2, 0),
            (vec![2,1], 0, -1),
            (vec![2,1], 3, -1),
            (vec![4,5,6,7,0,1,2], 3, -1),
        ];
        for (nums, target, expect) in test_cases {
            assert_eq!(Solution::search(nums.clone(), target), expect, "nums: {:?}, target: {}", nums, target);
        }
    }
}