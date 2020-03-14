use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let n = nums2.len();
        let mut q = vec![];
        let mut hash = HashMap::new();
        hash.insert(nums2[n-1], -1);
        q.push(nums2[n-1]);
        for i in (0..n-1).rev() {
            let v = nums2[i];
            while let Some(&top) = q.last() {
                if v < top { break; }
                q.pop();
            }
            if let Some(&top) = q.last() {
                hash.insert(v, top);
            } else {
                hash.insert(v, -1);
            }
            q.push(v);
        }
        nums1.into_iter().map(|v| *hash.get(&v).unwrap()).collect()
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