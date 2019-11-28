use std::cmp::min;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let l1 = nums1.len();
        let l2 = nums2.len();
        let t = l1 + l2;
        if t == 0 {
            return 0f64;
        }
        if t%2 == 0 {
            let v1 = Self::find_kth(&nums1, &nums2, t/2) as f64;
            let v2 = Self::find_kth(&nums1, &nums2, t/2+1) as f64;
            (v1+v2)/2.0
        } else {
            Self::find_kth(&nums1, &nums2, t/2+1) as f64
        }
    }
    fn find_kth(nums1: &[i32], nums2: &[i32], k: usize) -> i32 {
        if nums1.is_empty() {
            return nums2[k-1];
        } else if nums2.is_empty() {
            return nums1[k-1];
        }
        if k == 1 {
            return min(nums1[0], nums2[0]);
        }
        let m = k/2;
        if nums1.len() < m {
            return Self::find_kth(nums1, &nums2[k-nums1.len()-1..], nums1.len()+1);
        } else if nums2.len() < m {
            return Self::find_kth(&nums1[k-nums2.len()-1..], nums2, nums2.len()+1);
        }
        let v1 = nums1[m-1];
        let v2 = nums2[m-1];
        if v1 <= v2 {
            Self::find_kth(&nums1[m..], nums2, k-m)
        } else {
            Self::find_kth(nums1, &nums2[m..], k-m)
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_median_sorted_arrays() {
        let test_cases = vec![
            (vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22],vec![0,6], 10.5),
            (vec![1], vec![], 1.0),
            (vec![1,3], vec![2], 2.0),
            (vec![1,2],vec![3,4], 2.5),
            (vec![1,3,5], vec![2,4,6],3.5),
            (vec![1,2,3,6,7,8], vec![5], 5.0),
        ];
        for (nums1, nums2, expect) in test_cases {
            assert_eq!(Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone()), expect, "nums1: {:?}, nums2: {:?}", nums1, nums2);
        }
    }
}