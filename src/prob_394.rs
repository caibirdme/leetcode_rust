impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut ans = vec![];
        let mut repeat = vec![];
        let mut q = vec![];
        let mut i = 0;
        let bs = s.as_bytes();
        while i < bs.len() {
            match bs[i] {
                b'0'...b'9'=> {
                    let mut acc = 0;
                    while i<bs.len() && bs[i]>=b'0' && bs[i]<=b'9' {
                        acc = acc*10 + (bs[i]-b'0') as i32;
                        i+=1;
                    }
                    repeat.push(acc);
                },
                b'[' => {
                    q.push(b'[');
                    i+=1;
                },
                b']' => {
                    i+=1;
                    let mut tmp = vec![];
                    while let Some(c) = q.pop() {
                        if c == b'[' {break;}
                        tmp.push(c);
                    }
                    let mut l = 0;
                    let mut r = tmp.len()-1;
                    while l < r {
                        tmp.swap(l,r);
                        l+=1;
                        r-=1;
                    }
                    let repeat_times = repeat.pop().unwrap();
                    for _ in 0..repeat_times {
                        for &c in &tmp {
                            if repeat.is_empty() {
                                ans.push(c);
                            } else {
                                q.push(c);
                            }
                        }
                    }
                },
                _ => {
                    if repeat.is_empty() {
                       ans.push(bs[i]);
                    } else {
                        q.push(bs[i]);
                    }
                    i+=1;
                },
            }
        }
        unsafe {std::str::from_utf8_unchecked(&ans).to_string()}
    }

    pub fn decode_string_dfs(s: String) -> String {
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