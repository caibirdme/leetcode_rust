use std::collections::BTreeMap;
use std::ops::Bound::Included;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if k <= 0 || t < 0 {
            return false;
        }
        let n = nums.len();
        if n <= 1 {
            return false;
        }
        let t = t as i64;
        let mut root: BTreeMap<i64, ()> = BTreeMap::new();
        root.insert(nums[0] as i64, ());
        for i in 1..n {
            let (lb,rb) = Self::get_range(nums[i] as i64 -t, nums[i] as i64+t);
            let mut r = root.range((Included(&lb), Included(&rb)));
            if r.next().is_some() {
                return true;
            }
            if i >= k as usize {
                let q = nums[i-k as usize] as i64;
                root.remove(&q);
            }
            root.insert(nums[i] as i64, ());
        }
        false
    }
    fn get_range(a: i64, b: i64) -> (i64, i64) {
        if a < b {
            (a,b)
        } else {
            (b,a)
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_contains_nearby_almost_duplicate() {
        let test_cases = vec![
            (vec![-1, 2147483647], 1, 2147483647, false),
            (vec![1,2], 0, 1, false),
            (vec![2,2], 3, 0, true),
            (vec![-1,-1,], 1, -1, false),
            (vec![1,2,3,1], 3, 0, true),
            (vec![1,0,1,1], 1, 2, true),
            (vec![1,5,9,1,5,9], 2, 3, false),
        ];
        for (nums, k, t, ok) in test_cases {
            assert_eq!(Solution::contains_nearby_almost_duplicate(nums.clone(), k, t), ok, "nums: {:?}, k: {}, t: {}", nums, k, t);
        }
    }
}