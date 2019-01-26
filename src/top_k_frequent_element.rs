use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

struct Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.capacity() == 0 {
            return vec![];
        }
        let mut count = HashMap::new();
        for i in nums {
            let mut c = count.entry(i).or_insert(0);
            *c += 1;
        }
        let mut min_heap = BinaryHeap::with_capacity(k as usize);
        let mut idx: usize = 0;
        let uk = k as usize;
        for (&key,&v) in count.iter() {
            if idx < uk {
                min_heap.push(ordNum(v,key));
                idx+=1;
            } else if v > min_heap.peek().unwrap().0 {
                min_heap.push(ordNum(v,key));
                min_heap.pop();
            }
        }
        let mut ans = Vec::with_capacity(k as usize);
        while let Some(num) = min_heap.pop() {
            ans.push(num.1);
        }
        ans.reverse();
        ans
    }
}

#[derive(Eq, PartialEq)]
struct ordNum(i32,i32);
impl Ord for ordNum {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 > other.0 {
            Ordering::Less
        } else if self.0 < other.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
impl PartialOrd for ordNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_top_k_frequent() {
        assert_eq!(
            Solution::top_k_frequent(vec![1,1,1,2,2,3], 2),
            vec![1,2]
        );
        assert_eq!(
            Solution::top_k_frequent(vec![1], 1),
            vec![1]
        );
        assert_eq!(
            Solution::top_k_frequent(vec![1,1,1,1,2,2,2,2,2,2,2,2], 1),
            vec![2]
        );
        assert_eq!(
            Solution::top_k_frequent(vec![3,3,1,1,1,1,2,2,2,2,2,2,2,2], 2),
            vec![2,1]
        );
        assert_eq!(
            Solution::top_k_frequent(vec![3,3,1,1,1,1,2,2,2,2,2,2,2,2], 3),
            vec![2,1,3]
        );
    }
}