impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }
        if nums.first().unwrap() == nums.last().unwrap() {
            return Self::find_o_n(&nums, target);
        }
        if nums.first().unwrap() < nums.last().unwrap() {
            match nums.binary_search(&target) {
                Ok(_) => return true,
                _ => return false,
            }
        }
        let n = nums.len()-1;
        let (mut l, mut r) = (0, n);
        let n0 = nums[0];
        while l < r {
            let mid = (l+r) / 2;
            let v = nums[mid];
            if v == target {
                return true;
            }
            if v >= n0 {
                l = mid+1;
            } else {
                r = mid
            }
        }
        let slice = {
            if target >= n0 {
                &nums[..l]
            } else {
                &nums[l..]
            }
        };
        match slice.binary_search(&target) {
            Ok(_) => true,
            _ => false,
        }
    }
    fn find_o_n(nums: &Vec<i32>, target: i32) -> bool {
        match nums.iter().find(|&&v| v==target) {
            Some(_) => true,
            None => false,
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_search() {
        let test_cases = vec![
            (vec![2,2,2,0,1], 0, true),
            (vec![5,7,9,11,1,2,3], 5, true),
            (vec![1,3], 3, true),
            (vec![2,5,6,0,0,1,2], 0, true),
            (vec![2,5,6,0,0,1,2], 3, false),
            (vec![5,7,9,11,1,2,3], 2, true),
            (vec![5,7,9,11,1,2,3], 9, true),
            (vec![5,7,9,11,1,2,3], 8, false),
            (vec![5,7,9,11,1,2,3], 4, false),
            (vec![5,7,9,11,1,2,3], 12, false),
        ];
        for (nums, target, ok) in test_cases {
            assert_eq!(Solution::search(nums.clone(), target), ok, "nums: {:?}, target: {}", nums, target);
        }
    }
}