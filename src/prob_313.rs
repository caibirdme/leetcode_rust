
impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        use std::collections::{BinaryHeap,HashSet};
        use std::cmp::Reverse;
        let mut heap = BinaryHeap::new();
        let mut used = HashSet::new();
        heap.push(Reverse(1));
        let mut count = 0;
        while !heap.is_empty() {
            let num = heap.pop().unwrap().0;
            count+=1;
            if count == n {
                return num;
            }
            used.remove(&num);
            for &p in primes.iter() {
                if let Some(n_v) = num.checked_mul(p) {
                    if !used.contains(&n_v) {
                        heap.push(Reverse(n_v));
                        used.insert(n_v);
                    }
                }
            }
        }
        unreachable!()
    }
    pub fn nth_super_ugly_number_2(n: i32, primes: Vec<i32>) -> i32 {
        use std::cmp::min;
        //use std::i32;
        let mut ans = vec![1];
        let m = primes.len();
        let mut little = vec![0; m];
        for _ in 2..=n {
            let mut next = i32::max_value();
            for (&idx,&b) in little.iter().zip(primes.iter()) {
                next = min(next, ans[idx]*b);
            }
            ans.push(next);
            for (idx, &b) in little.iter_mut().zip(primes.iter()) {
                while ans[*idx]*b <= next {*idx += 1;}
            }
        }
        *ans.last().unwrap()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_nth_super_ugly_number() {
        let test_cases = vec![
            (10, vec![2,3,5]),
        ];
        for (n, primes) in test_cases {
            assert_eq!(
                Solution::nth_super_ugly_number(n, primes.clone()),
                Solution::nth_super_ugly_number_2(n, primes.clone())
            );
        }
    }
}