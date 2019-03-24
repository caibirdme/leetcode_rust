impl Solution {
    pub fn decode_string(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let mut pos = 0;
        Self::dfs(s.as_bytes(), &mut pos)
    }
    fn dfs(s: &[u8], pos: &mut usize) -> String {
        let mut ans = String::new();
        while *pos < s.len() && s[*pos] != b']'{
            let len = Self::read_num(s, pos);
            if len == 0 {
                ans += Self::read_chars(s, pos).as_str();
            } else {
                // eat [
                *pos += 1;

                ans += Self::dfs(s, pos).repeat(len).as_str();

                // eat ]
                *pos +=1;
            }
        }
        ans
    }
    fn read_num(s: &[u8], pos: &mut usize) -> usize {
        let mut t = 0;
        loop {
            let c = s[*pos];
            if !Self::is_num(c) {
                break;
            }
            t = t*10 + (c-b'0') as usize;
            *pos+=1;
        }
        t
    }
    fn read_chars(s: &[u8], pos: &mut usize) -> String {
        let start = *pos;
        while *pos < s.len() && Self::is_char(s[*pos]) {
            *pos += 1;
        }
        unsafe {std::str::from_utf8_unchecked(&s[start..*pos]).to_string()}
    }
    fn is_num(c: u8) -> bool {
        match c {
            b'0'...b'9' => true,
            _ => false,
        }
    }
    fn is_char(c: u8) -> bool {
        if Self::is_num(c) || c == b'[' || c == b']' {
            false
        } else {
            true
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_decode_string() {
        let test_cases = vec![
            ("3[a]2[b4[F]c]", "aaabFFFFcbFFFFc"),
            ("", ""),
            ("1[ab]2[cd]", "abcdcd"),
            ("assacx", "assacx"),
            ("3[a]2[bc]", "aaabcbc"),
            ("3[a2[c]d]", "accdaccdaccd"),
            ("2[abc]3[cd]ef", "abcabccdcdcdef"),
        ];
        for (s, expect) in test_cases {
            assert_eq!(expect, Solution::decode_string(s.to_string()), "{}", s);
        }
    }
}