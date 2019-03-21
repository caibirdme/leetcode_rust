impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        if input.len() == 0 {
            return 0;
        }

        let mut ans = 0;
        let mut pos = 0;
        let ns = "\n".to_string() + input.as_str();
        Self::dfs(ns.as_bytes(), &mut pos, 0, 0, &mut ans);
        if ans == 0 {
            ans as i32
        } else {
            ans as i32 -1
        }
    }
    fn dfs(s: &[u8], pos: &mut usize, depth: usize, length: usize, ans: &mut usize) {
        while Self::peek(s, *pos, depth) {
            *pos += depth+1;
            let (l,is_file) = Self::eat(s, *pos);
            *pos+=l;
            let t = length+1+l;
            if !is_file {
                Self::dfs(s, pos, depth+1, t, ans);
            } else if t > *ans {
                *ans = t;
            }
        }
    }
    fn eat(s: &[u8], pos: usize) -> (usize,bool) {
        let mut idx = pos;
        let mut is_file = false;
        while idx < s.len() && s[idx]!=b'\n' {
            if s[idx] == b'.' {
                is_file = true;
            }
            idx+=1;
        }
        (idx-pos,is_file)
    }
    fn peek(s: &[u8], mut pos: usize, n: usize) -> bool {
        if pos+n >= s.len() {
            return false;
        }
        if s[pos] != b'\n' {
            return false;
        }
        for _ in 0..n {
            pos+=1;
            if s[pos] != b'\t' {
                return false;
            }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_length_longest_path() {
        let test_cases = vec![
            ("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext", 20),
            ("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext", 32),
            ("dir", 0),
            ("", 0),
            ("a\n\taa\n\t\taaa\n\t\t\taaaaaaaaaaaaaa\n\taaaaaaa.q", 11),
            ("a\n\taa\n\t\taaa\n\t\t\ta\n\taaaaaaaaa.q", 13),
        ];
        for (s, expect) in test_cases {
            assert_eq!(expect, Solution::length_longest_path(s.to_string()), "{}", s);
        }
    }
}