use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let h1: HashSet<i32> = HashSet::from_iter(nums1);
        let h2: HashSet<i32> = HashSet::from_iter(nums2);
        h1.intersection(&h2).map(|x| *x).collect()
    }
}

struct Solution;