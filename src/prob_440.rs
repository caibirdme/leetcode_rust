impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let n = n as i64;
        let k = k as i64;
        let mut rest = k - 1;
        let mut cur = 1;
        while rest > 0 {
            let steps = Self::calc(n, cur, cur+1);
            if steps <= rest {
                rest -= steps;
                cur+=1;
            } else {
                rest -= 1;
                cur *= 10;
            }
        }
        cur as i32
    }
    fn calc(n: i64, mut cur: i64, mut next: i64) -> i64 {
        use std::cmp::min;
        let mut steps = 0;
        while cur <= n {
            steps += min(n+1, next) - cur;
            cur *= 10;
            next *= 10;
        }
        steps
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_kth_number() {
        let test_cases = vec![
            (681692778, 351251360, 416126219),
            (13, 2, 10),
            (13,3, 11),
            (13,5,13),
            (13,6,2),
        ];
        for (n,k,e) in test_cases {
            assert_eq!(e, Solution::find_kth_number(n,k), "n:{}, k:{}", n,k);
        }
    }
}