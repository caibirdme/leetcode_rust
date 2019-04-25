use std::collections::HashMap;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut p_map = HashMap::new();
        for c in p.as_bytes() {
            let t = p_map.entry(c).or_insert(0);
            *t += 1;
        }
        let m = p.len();
        let mut cur = HashMap::new();
        let mut count = 0;
        let mut ans = vec![];
        let sbs = s.as_bytes();
        for (i, c) in sbs.iter().enumerate() {
            if let Some(&v) = p_map.get(c) {
                let t = cur.entry(c).or_insert(0);
                *t += 1;
                count += 1;
                if *t > v {
                    let mut j = {
                        if i<count {
                            0
                        } else {
                            i+1-count
                        }
                    };
                    while j<i {
                        count -= 1;
                        let q = cur.get_mut(&sbs[j]).unwrap();
                        *q -= 1;
                        if sbs[j] == *c {
                            break;
                        }
                        j+=1;
                    }
                }
                if count == m {
                    ans.push((i+1-m) as i32);
                }
            } else {
                cur.clear();
                count = 0;
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
    fn test_find_anagrams() {
        let test_cases = vec![
            ("cbaebabacd", "abc", vec![0,6]),
            ("abab", "ab", vec![0,1,2]),
            ("aaaaa", "a", vec![0,1,2,3,4]),
        ];
        for (s,p,expect) in test_cases {
            assert_eq!(expect, Solution::find_anagrams(s.clone().to_string(), p.clone().to_string()), "s: {}, p: {}", s,p);
        }
    }
}