impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut count = [0; 4];
        let mut ans = 0;
        let mut rest = 0;
        for &c in croak_of_frogs.as_bytes() {
            let c = Self::conv(c);
            if c == 0 {
                if rest == 0 {
                    ans += 1;
                } else {
                    rest -= 1;
                }
                count[c] += 1;
            } else {
                count[c-1] -= 1;
                if count[c-1] < 0 {
                    return -1;
                }
                if c < 4 {
                    count[c] += 1;
                } else {
                    rest += 1;
                }
            }
        }
        for i in 0..4 {
            if count[i] != 0 {
                return -1;
            }
        }
        ans
    }
    fn conv(x: u8) -> usize {
        match x {
            b'c' => 0,
            b'r' => 1,
            b'o' => 2,
            b'a' => 3,
            b'k' => 4,
            _ => unreachable!(),
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_number_of_frogs() {
        let test_cases = vec![
            ("crcoakroak", 2),
            ("croakcroak", 1),
            ("croakcrook", -1),
            ("croakcroa", -1),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::min_number_of_frogs(s.to_string()), expect, "s: {}",s );
        }
    }
}