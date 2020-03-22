pub struct Matcher {
    next: Vec<i32>,
    p: Vec<u8>,
}

impl Matcher {
    pub fn new(p: &[u8]) -> Self {
        let n = p.len();
        let mut next = vec![-1; n];
        let mut i = 0;
        let mut j = -1;
        while i+1 < n {
            if p[(i+1) as usize] == p[(j+1) as usize] {
                i+=1;
                j+=1;
                next[i as usize] = j;
            } else if j == -1 {
                i+=1;
            } else {
                j = next[j as usize];
            }
        }
        Self{next, p: Vec::from(p),}
    }
    fn do_match(&self, s: &[u8]) -> Vec<usize> {
        if self.p.is_empty() {
            return vec![];
        }
        let mut ans = vec![];
        let mut i = 0;
        let mut j = 0;
        let pn = self.p.len();
        while i < s.len() {
            if s[i] == self.p[j] {
                i+=1;
                j+=1;
                if j == pn {
                    ans.push(i-pn);
                    j = (self.next[j-1]+1) as usize;
                }
            } else if j == 0 {
                i+=1;
            } else {
                j = (self.next[j-1]+1) as usize;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Matcher;

    #[test]
    fn test_kmp() {
        let test_cases = vec![
            ("ababa", "ababacabababa", vec![0, 6, 8]),
            ("aaa", "aaaaaabaaaa", vec![0,1,2,3,7,8]),
        ];
        for (p, s, expect) in test_cases {
            let kmp = Matcher::new(p.as_bytes());
            assert_eq!(kmp.do_match(s.as_bytes()), expect, "p:{}, s:{}",p,s);
        }
    }
}