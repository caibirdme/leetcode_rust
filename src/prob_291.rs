use std::collections::HashMap;

impl Solution {
    pub fn word_pattern_match(pattern: String, str: String) -> bool {
        if pattern.is_empty() {
            return str.is_empty();
        }
        if str.is_empty() {
            return false;
        }
        if pattern.len() == 1 {
            return true;
        }
        if pattern.len() > str.len() {
            return false;
        }
        let mut dict = HashMap::new();
        let mut index = HashMap::new();
        Self::is_match(pattern.as_bytes(), str.as_bytes(), &mut dict, &mut index)
    }
    fn is_match<'a,'b: 'a>(pattern: &'b [u8], st: &'b [u8], dict: &mut HashMap<u8, &'a [u8]>, index: &mut HashMap<&'a [u8], u8>) -> bool {
        if pattern.is_empty() {
            return st.is_empty();
        }
        if st.is_empty() {
            return false;
        }
        if pattern.len() > st.len() {
            return false;
        }
        let c = pattern[0];
        if let Some(&p) = dict.get(&c) {
            if !Self::expect(st, p) {
                return false;
            }
            return Self::is_match(&pattern[1..], &st[p.len()..], dict, index);
        } else {
            for i in 0..=st.len()-pattern.len() {
                let part = &st[..i+1];
                if index.contains_key(&part) {
                    continue;
                }
                dict.insert(c, part);
                index.insert(part, c);
                if Self::is_match(&pattern[1..], &st[i+1..], dict, index) {
                    return true;
                }
                index.remove(part);
                dict.remove(&c);
            }
        }
        false
    }
    fn expect(st: &[u8], target: &[u8]) -> bool {
        if target.len() > st.len() {
            return false;
        }
        target == &st[..target.len()]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_word_pattern_match() {
        let test_cases = vec![
            ("itwasthebestoftimes", "ittwaastthhebesttoofttimesss", false),
            ("abba", "dogcatcatdog", true),
            ("ab", "aa", false),
            ("abab", "redblueredblue", true),
            ("aaaa", "asdasdasdasd", true),
            ("aabb", "xyzabcxzyabc", false),
        ];
        for (pattern, st, ok) in test_cases {
            assert_eq!(Solution::word_pattern_match(pattern.to_string(), st.to_string()), ok, "pattern: {}, str: {}", pattern, st);
        }
    }
}