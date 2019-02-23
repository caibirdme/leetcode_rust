
impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        use std::collections::HashMap;
        if pattern.is_empty() {
            return str.is_empty();
        }
        let words: Vec<String> = str.split(' ').filter(|x| !x.is_empty()).map(|x| x.to_string()).collect();
        if words.len() != pattern.len() {
            return false;
        }
        let mut pattern_2_word: HashMap<char, &str> = HashMap::new();
        let mut word_2_pattern: HashMap<&str, char> = HashMap::new();
        for (c,s) in pattern.chars().zip(words.iter()) {
            match pattern_2_word.get(&c) {
                Some(&expect_s) => {
                    if expect_s != s {return false;}
                },
                None => {
                    match word_2_pattern.get(s.as_str()) {
                        Some(&r_c) => {
                            if r_c != c {return false;}
                        },
                        None => {
                            pattern_2_word.insert(c, s.as_str());
                            word_2_pattern.insert(s.as_str(), c);
                        }
                    }
                }
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
    fn test_word_pattern() {
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog   cat   cat   dog".to_string()),
            true
        );
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
            true
        );
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
            false
        );
        assert_eq!(
            Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()),
            false
        );
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog dog dog dog".to_string()),
            false
        );
    }
}