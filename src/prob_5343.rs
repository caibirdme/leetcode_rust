use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut heap: BinaryHeap<i64> = target.iter().map(|&v| v as i64).collect();
        let mut sum = target.iter().fold(0, |pre,&v| pre+v as i64);
        let n = target.len() as i64;
        loop {
            if sum == n {
                return true;
            }
            let largest = heap.pop().unwrap();
            let before = 2*largest-sum;
            if before <= 0 {
                return false;
            }
            heap.push(before);
            sum = largest;
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_possible() {
        let test_cases = vec![
            (vec![9,9,9], false),
            (vec![9,3,5], true),
            (vec![1,1,1,2], false),
        ];
        for (target, ok) in test_cases {
            assert_eq!(Solution::is_possible(target.clone()), ok, "target: {:?}", target);
        }
    }
}