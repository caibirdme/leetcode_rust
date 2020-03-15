use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        const M: i64 = 1000000007;
        let mut tuple: Vec<(i32, i32)> = efficiency
            .into_iter().zip(speed).map(|t| t).collect();
        tuple.sort_by(|a,b| {
            if a.0 == b.0 {
                return a.1.cmp(&b.1);
            }
            a.0.cmp(&b.0)
        });
        let uk = k as usize;
        let mut heap = BinaryHeap::new();
        let mut total = 0;
        let n = n as usize;
        let mut ans = 0;
        for i in (n-uk..n).rev() {
            heap.push(Reverse(tuple[i].1 as i64));
            total += tuple[i].1 as i64;
            ans = ans.max(tuple[i].0 as i64 * total);
        }
        for i in (0..n-uk).rev() {
            let peek = heap.peek().unwrap_or(&Reverse(0)).0;
            let maybe_new_total = total+tuple[i].1 as i64-peek;
            let cur = tuple[i].0 as i64 * maybe_new_total;
            ans = ans.max(cur);
            if tuple[i].1 as i64 > peek {
                heap.pop();
                heap.push(Reverse(tuple[i].1 as i64));
                total = maybe_new_total;
            }
        }
        (ans % M) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_performance() {
        let test_cases = vec![
            (3, vec![2,8,2], vec![2,7,1], 2, 56),
            (6, vec![2,10,3,1,5,8], vec![5,4,3,9,7,2], 2, 60),
            (6, vec![2,10,3,1,5,8], vec![5,4,3,9,7,2], 3, 68),
            (6, vec![2,10,3,1,5,8], vec![5,4,3,9,7,2], 4, 72),
        ];
        for (n, speed, eff, k, expect) in test_cases {
            assert_eq!(Solution::max_performance(n, speed.clone(), eff.clone(), k), expect, "n: {}, speed: {:?}, eff: {:?}, k:{}",n,speed,eff,k);
        }
    }
}