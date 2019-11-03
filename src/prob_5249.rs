impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let mut l = 0;
        let bs: Vec<u8> = s.as_bytes().into_iter().filter(|&&c| {
            match c {
                b'(' => {
                    l += 1;
                    true
                },
                b')' => {
                    if l > 0 {
                        l-=1;
                        true
                    } else {
                        false
                    }
                },
                _ => true,
            }
        }).map(|c| *c).collect();
        let n = bs.len();
        let t = bs.into_iter().rev().filter(|&c| {
            match c {
                b'(' if l > 0 => {
                    l-=1;
                    false
                },
                _ => true,
            }
        }).collect::<Vec<u8>>();
        let ans = t.into_iter().rev().collect::<Vec<u8>>();
        unsafe {std::str::from_utf8_unchecked(&ans)}.to_string()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    fn check_valid(st: &str) -> bool {
        let mut l = 0;
        for &c in st.as_bytes() {
            match c {
                b'(' => l+=1,
                b')' => {
                    if l > 0 {
                        l-=1;
                    } else {
                        return false;
                    }
                },
                _ => {},
            }
        }
        l==0
    }

    #[test]
    fn test_min_remove_to_make_valid() {
        let test_cases = vec![
            ("))(()))()))))))())())(((((()()()(()())i))))s(i)))((((a)s(((()()(k)))))l)()))()(bm(()(()))))r))))())("),
            (")(a)((b)"),
            ("lee(t(c)o)de)"),
            ("a)b(c)d"),
            ("))(("),
            ("(a(b(c)d)"),
        ];
        for s in test_cases {
            let actual = Solution::min_remove_to_make_valid(s.to_string());
            assert!(check_valid(&actual), "s: {}", s);
        }
    }
}