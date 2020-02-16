impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let idx = equation.find('=').unwrap();
        let bs = equation.as_bytes();
        let (a1,b1) = Self::calc(&bs[..idx]);
        let (a2, b2) = Self::calc(&bs[idx+1..]);
        if a1 == a2 {
            if b1 == b2 {
                "Infinite solutions".to_string()
            } else {
                "No solution".to_string()
            }
        } else {
            format!("x={}",(b2-b1)/(a1-a2))
        }
    }
    fn calc(s: &[u8]) -> (i32, i32) {
        let mut a = 0;
        let mut b = 0;
        let mut i = 0;
        while i < s.len() {
            let mut j = i;
            let mut positive = true;
            // read sign
            if s[j] == b'-' {
                positive = false;
                j+=1;
            } else if s[j] == b'+' {
                j+=1;
            }
            let mut v: Option<i32> = None;
            // read number
            while j < s.len() && s[j] >= b'0' && s[j] <= b'9' {
                v = v.or(Some(0)).map(|pre| pre*10 + (s[j]-b'0') as i32);
                j+=1;
            }
            if j == s.len() || s[j] == b'-' || s[j] == b'+' {
                if positive {
                    b += v.unwrap();
                } else {
                    b -= v.unwrap();
                }
            } else {
                v = v.or(Some(1));
                if positive {
                    a += v.unwrap();
                } else {
                    a -= v.unwrap();
                }
                j+=1;
            }
            i = j;
        }
        (a,b)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solve_equation() {
        let test_cases = vec![
            ("7+8-0x=0", "No solution"),
            ("0x=0", "Infinite solutions"),
            ("-5x+4+x=-8+7-7", "x=3"),
            ("x+5-3+x=6+x-2", "x=2"),
            ("x=x", "Infinite solutions"),
            ("2x=x", "x=0"),
            ("2x+3x-6x=x+2", "x=-1"),
            ("x=x+2", "No solution"),
        ];
        for (equation, expect) in test_cases {
            assert_eq!(Solution::solve_equation(equation.to_string()), expect.to_string(), "equation: {}", equation);
        }
    }
}