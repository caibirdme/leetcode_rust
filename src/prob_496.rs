use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let n = nums2.len();
        let mut f = vec![0; n];
        let mut hash = HashMap::new();
        hash.insert(nums2[n-1], n-1);
        for i in (0..n-1).rev() {
            hash.insert(nums2[i], i);
            if nums2[i] < nums2[i+1] {
                f[i] = 1;
            } else {
                let mut j = i+1;
                while j < n && nums2[j] < nums2[i] {
                    if f[j] == 0 {
                        break;
                    }
                    j += f[j];
                }
                if nums2[i] < nums2[j] {
                    f[i] = j-i;
                }
            }
        }
        let mut ans = vec![];
        for v in &nums1 {
            let pos = *hash.get(v).unwrap();
            if f[pos] == 0 {
                ans.push(-1);
            } else {
                ans.push(nums2[f[pos]+pos]);
            }
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_next_greater_element() {
        let test_cases = vec![
            (vec![4,1,2], vec![1,3,4,2], vec![-1,3,-1]),
            (vec![2,4], vec![1,2,3,4], vec![3,-1]),
        ];
        for (nums1, nums2, expect) in test_cases {
            assert_eq!(Solution::next_greater_element(nums1.clone(), nums2.clone()), expect, "nums1: {:?}, nums2: {:?}", nums1,nums2);
        }
    }
}