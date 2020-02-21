impl Solution {
    pub fn find_permutation(s: String) -> Vec<i32> {
        let n = s.len();
        let bs = s.as_bytes();
        let mut ans = vec![1];
        let mut i = 1;
        let mut cur = 2;
        while i < n && bs[i] == bs[i-1] {
            ans.push(cur);
            cur += 1;
            i += 1;
        }
        ans.push(cur);
        cur+=1;
        if bs[i-1] == b'D' {
            ans.reverse();
        }
        while i < n {
            let mut j = i;
            ans.push(cur);
            cur+=1;
            while j+1<n && bs[j] == bs[j+1] {
                j+=1;
                ans.push(cur);
                cur += 1;
            }
            if bs[i] == b'D' {
                let mut l = i;
                let mut r = ans.len()-1;
                while l < r {
                    ans.swap(l,r);
                    l+=1;
                    r-=1;
                }
            }
            i = j+1;
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_permutation() {
        let test_cases = vec![
            "DDIIDI",
            "I",
            "D",
            "DI",
            "ID",
            "DDI",
            "IIIDDD",
            "IDIDIDDDI",
            "DIDIDIDIIIDDDIIIDIDIDIIIIIIIDDDDIDIDIDIDIDIIIIDIDDDDDII",
        ];
        for s in test_cases {
            let actual = Solution::find_permutation(s.to_string());
            assert_eq!(check(s.as_bytes(), &actual), true, "s: {}, actual: {:?}", s, actual);
        }
    }
    fn check(s: &[u8], v: &Vec<i32>) -> bool {
        let n = s.len();
        for i in 0..n {
            if s[i] == b'D' {
                if v[i] < v[i+1] {
                    return false;
                }
            } else {
                if v[i] > v[i+1] {
                    return false;
                }
            }
        }
        true
    }
}