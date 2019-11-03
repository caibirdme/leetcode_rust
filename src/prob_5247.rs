impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let mut x = 0;
        let mut y = 0;
        for (&sx, &sy) in s1.as_bytes().into_iter().zip(s2.as_bytes()) {
            if sx != sy {
                if sx == b'x' {
                    x += 1;
                } else {
                    y += 1;
                }
            }
        }
        let mut ans = 0;
        ans += x / 2;
        x %= 2;
        ans += y / 2;
        y %= 2;
        if x + y == 1 {
            return -1
        } else if x == 1 && y == 1 {
            ans += 2;
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_minimum_swap() {
        let test_cases = vec![
            ("xy", "yx", 2),
            ("xxyyxyxyxx", "xyyxyxxxyx", 4),
            ("xx", "yy", 1),
            ("xx", "xy", -1),
        ];
        for (s1,s2,expect) in test_cases {
            assert_eq!(Solution::minimum_swap(s1.to_string(), s2.to_string()), expect, "s1: {}, s2: {}",s1,s2);
        }
    }
}