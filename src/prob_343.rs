impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        use std::cmp::max;
        let mut ans = 1;
        if n == 3 {
            return 2;
        }
        if n == 2 {
            return 1;
        }
        let t = n/3;
        for i in 0..=t {
            let rest = n-i*3;
            if rest&1==0 {
                ans = max(ans,3i32.pow(i as u32) * 2i32.pow((rest/2) as u32));
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
    fn test_integer_break() {
        let test_cases = vec![
            (5,6),
            (6,9),
            (4,4),
            (3, 2),
            (25, 8748),
            (10, 36),
            (2,1),
            (1,1),
        ];
        for (n, expect) in test_cases {
            assert_eq!(
                Solution::integer_break(n),
                expect,
                "{}", n
            );
        }
    }
}