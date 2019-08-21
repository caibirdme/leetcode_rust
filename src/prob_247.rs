impl Solution {
    pub fn find_strobogrammatic(n: i32) -> Vec<String> {
        if n == 1 {
            return vec!["0","1","8"].into_iter().map(|v| v.to_string()).collect();
        }
        let mut cur = vec![];
        let mut ans = vec![];
        Self::gen(0, n as usize/2-1, n&1==1, &mut cur, &mut ans);
        ans
    }
    fn gen(t: usize, n: usize, more: bool, cur: &mut Vec<char>, ans: &mut Vec<String>) {
        if t > n {
            let mut now = cur.clone();
            let mut rest = vec![];
            for &q in now.iter().rev() {
                let m = match q {
                    '0' => '0',
                    '1' => '1',
                    '6' => '9',
                    '8' => '8',
                    '9' => '6',
                    _ => unreachable!(),
                };
                rest.push(m);
            }
            if !more {
                now.append(&mut rest);
                ans.push(now.into_iter().collect());
                return;
            }
            for c in vec!['0','1','8'] {
                let mut _now = now.clone();
                let mut _rest = rest.clone();
                _now.push(c);
                _now.append(&mut _rest);
                ans.push(_now.into_iter().collect());
            }
            return;
        }
        for c in vec!['0','1','6','8','9'] {
            if t == 0 && c == '0' {
                continue;
            }
            cur.push(c);
            Self::gen(t+1, n, more, cur, ans);
            cur.pop();
        }
    }
}


struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_strobogrammatic() {
        let test_cases = vec![
            (2, vec!["11", "69", "88", "96"]),
            (1, vec!["0", "1", "8"]),
        ];
        for (n, expect) in test_cases {
            assert_eq!(Solution::find_strobogrammatic(n), expect, "n: {}", n);
        }
    }
}