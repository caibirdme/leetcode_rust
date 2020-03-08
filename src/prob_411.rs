impl Solution {
    pub fn min_abbreviation(target: String, dictionary: Vec<String>) -> String {
        let n = target.len();
        if n == 0 {
            return "".to_string();
        }
        if dictionary.is_empty() {
            return n.to_string();
        }
        let target = target.as_bytes();
        let mut min_len = std::i32::MAX;
        let mut ans = String::new();
        let mut v = vec![];
        for i in 0..1<<n {
            let mut ok = true;
            for s in &dictionary {
                let s = s.as_bytes();
                if s.len() != n {
                    continue;
                }
                let mut not_same = false;
                for j in 0..n {
                    if i&(1<<j) == 0 && target[j] != s[j] {
                        not_same = true;
                        break;
                    }
                }
                if !not_same {
                    ok = false;
                    break;
                }
            }
            if !ok {
                continue;
            }
            let mut len = 0;
            let mut j = 0;
            v.clear();
            while j < n {
                let mut c = 0;
                while j < n && i&(1<<j) > 0 {
                    j+=1;
                    c+=1;
                }
                len += 1;
                if c == 0 {
                    v.push(target[j]);
                    j += 1;
                } else {
                    let before_len = v.len();
                    while c > 0 {
                        v.push((c%10) as u8+b'0');
                        c /= 10;
                    }
                    let mut l = before_len;
                    let mut r = v.len()-1;
                    while l < r {
                        v.swap(l,r);
                        l+=1;
                        r-=1;
                    }
                }
            }
            if len < min_len {
                min_len = len;
                ans = unsafe {std::str::from_utf8_unchecked(&v).to_string()};
            }
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_abbreviation() {
        let test_cases = vec![
            ("apple", vec!["blade"], "a4"),
            //("apple", vec!["plain", "amber", "blade"], "1p3"),
        ];
        for (target, dict, expect) in test_cases {
            assert_eq!(Solution::min_abbreviation(target.to_string(), dict.iter().map(|v| v.to_string()).collect()), expect, "target: {}, dict: {:?}", target, dict);
        }
    }
}