use std::collections::HashMap;

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        if formula.is_empty() {
            return "".to_owned();
        }
        let m = Self::count(formula.as_bytes());
        if m.is_none() {
            return "".to_owned();
        }
        let m = m.unwrap();
        let mut arr: Vec<(&str, i32)> = m.into_iter().map(|v| v).collect();
        arr.sort_by(|(s1,_), (s2, _)| s1.cmp(s2));
        let mut ans = vec![];
        for (k,v) in arr {
            ans.push(k.to_string());
            if v > 1 {
                ans.push(v.to_string());
            }
        }
        ans.join("")
    }
    fn count(s: &[u8]) -> Option<HashMap<&str, i32>> {
        if s.is_empty() {
            return None;
        }
        let mut ans = HashMap::new();
        let mut i = 0;
        let n = s.len();
        while i < n {
            if s[i] == b'(' {
                let mut count = 1;
                let mut j = i+1;
                while j < n {
                    if s[j] == b'(' {
                        count+=1;
                    } else if s[j] == b')' {
                        count -= 1;
                    }
                    if count == 0 {
                        break;
                    }
                    j += 1;
                }
                let mut k = j+1;
                let mut times = 0;
                while k < n {
                    if s[k] >= b'0' && s[k] <= b'9' {
                        times = times*10 + ((s[k]-b'0') as i32);
                        k += 1;
                    } else {
                        break;
                    }
                }
                if times == 0 {
                    times = 1;
                }
                if let Some(inner) = Self::count(&s[i+1..j]) {
                    for (k,v) in inner {
                        *ans.entry(k).or_insert(0) += v*times;
                    }
                }
                i = k;
            } else {
                let mut j = i+1;
                while j < n {
                    if s[j] >= b'a' && s[j] <= b'z' {
                        j+=1;
                    } else {
                        break;
                    }
                }
                let mut times = 0;
                let mut k = j;
                while k < n {
                    if s[k] >= b'0' && s[k]<=b'9' {
                        times = times*10 + ((s[k]-b'0') as i32);
                        k+=1;
                    } else {
                        break;
                    }
                }
                if times == 0 {
                    times = 1;
                }
                *ans.entry(unsafe{std::str::from_utf8_unchecked(&s[i..j])}).or_insert(0) += times;
                i = k;
            }
        }
        Some(ans)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_of_atoms() {
        let test_cases = vec![
            ("H2O", "H2O"),
            ("Mg(OH)2", "H2MgO2"),
            ("K4(ON(SO3)2)2", "K4N2O14S4"),
        ];
        for (formula, expect) in test_cases {
            assert_eq!(Solution::count_of_atoms(formula.to_string()), expect, "formula: {}", formula);
        }
    }
}