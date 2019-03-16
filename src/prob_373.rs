use std::collections::BinaryHeap;
use std::cmp::Ordering;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
        let (n,m) = (nums1.len(), nums2.len());
        if n == 0 || m == 0 {
            return vec![];
        }
        let mut max_heap = BinaryHeap::new();
        for i in 0..n {
            for j in 0..m {
                let p = Pair{a:nums1[i],b:nums2[j],};
                if max_heap.len() < k as usize{
                    max_heap.push(p);
                } else {
                    let top = max_heap.peek().unwrap();
                    if p.a+p.b < top.a+top.b {
                        max_heap.pop();
                        max_heap.push(p);
                    }
                }
            }
        }
        max_heap.into_sorted_vec().into_iter().map(|p| vec![p.a,p.b]).collect()
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Pair {
    a:i32,
    b:i32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.a+self.b).cmp(&(other.a+other.b))
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_k_smallest_pairs() {
        let test_cases = vec![
            (vec![1,2], vec![3], 3, vec![vec![1,3], vec![2,3]]),
            (vec![1,1,2], vec![1,2,3], 2, vec![vec![1,1], vec![1,1]]),
            (vec![1,7,11], vec![2,4,6], 3, vec![vec![1,2],vec![1,4], vec![1,6]]),
        ];
        for (nums1,nums2,k,expect) in test_cases {
            let actual = Solution::k_smallest_pairs(nums1.clone(),nums2.clone(),k);
            assert_eq!(expect, actual, "nums1:{:?}, nums2:{:?}, k:{}",nums1,nums2,k);
        }
    }
}