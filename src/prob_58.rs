impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let cs = s.as_bytes();
        let n = s.len();
        let last_c_pos = {
            let mut i = n-1;
            while i > 0 && cs[i] == b' ' { i-= 1;}
            i
        };
        if last_c_pos == 0 && cs[last_c_pos] == b' ' {
            return 0;
        }
        let mut i = last_c_pos;
        while i > 0 && cs[i] != b' ' {
            i-=1;
        }
        if i == 0 && cs[0] != b' ' {
            (last_c_pos+1) as i32
        } else {
            (last_c_pos-i) as i32
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_length_of_last_word() {
        let test_cases = vec![
            ("Hello World", 5),
            ("Hello    World              ", 5),
            ("", 0),
            ("abcdef", 6),
            (" ab ", 2),
            (" abc", 3),
            ("abcd              ", 4),
            ("   a b cde   ", 3),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::length_of_last_word(s.to_string()), expect, "s: {}", s);
        }
    }
}