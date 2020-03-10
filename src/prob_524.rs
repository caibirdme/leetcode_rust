use std::collections::HashMap;

impl Solution {
    pub fn find_longest_word(s: String, mut d: Vec<String>) -> String {
        if s.is_empty() || d.is_empty() {
            return "".to_string();
        }
        d.sort_by(|a,b| {
            if a.len() == b.len() {
                return a.cmp(b);
            }
            b.len().cmp(&(a.len()))
        });
        let mut hash = HashMap::new();
        for (idx, &c) in s.as_bytes().iter().enumerate() {
            hash.entry(c).or_insert(vec![]).push(idx);
        }
        for ds in d {
            let mut pos = None;
            let mut ok = true;
            for dc in ds.as_bytes() {
                if let Some(arr) = hash.get(dc) {
                    let next = pos.map_or(0, |v| v+1);
                    match arr.binary_search(&next) {
                        Ok(idx) => {
                            pos = Some(next);
                        },
                        Err(idx) => {
                            if idx == arr.len() {
                                ok = false;
                                break;
                            }
                            pos = Some(arr[idx]);
                        },
                    }
                } else {
                    ok = false;
                    break;
                }
            }
            if ok {
                return ds;
            }
        }
        "".to_string()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_longest_word() {
        let test_cases = vec![
            ("abpcplea", vec!["ale","apple","monkey","plea"], "apple"),
            ("abpcplea", vec!["a", "b", "c"], "a"),
        ];
        for (s, d, expect) in test_cases {
            assert_eq!(Solution::find_longest_word(s.to_string(), d.iter().map(|v| v.to_string()).collect()), expect, "s: {}, d: {:?}",s,d);
        }
    }
}