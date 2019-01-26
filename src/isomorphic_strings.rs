use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut reverse = HashMap::new();
        let mut convert = HashMap::new();
        for (&a,&b) in s.as_bytes().iter().zip(t.as_bytes()) {
            let from = reverse.entry(b).or_insert(a);
            if *from != a {
                return false;
            }
            let to = convert.entry(a).or_insert(b);
            if *to!=b {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_is_isomorphic() {
        assert_eq!(
            Solution::is_isomorphic("egg".to_string(), "add".to_string()),
            true
        );
        assert_eq!(
            Solution::is_isomorphic("foo".to_string(), "bar".to_string()),
            false
        );
        assert_eq!(
            Solution::is_isomorphic("paper".to_string(), "title".to_string()),
            true
        );
        assert_eq!(
            Solution::is_isomorphic("aa".to_string(), "ab".to_string()),
            false
        );
        assert_eq!(
            Solution::is_isomorphic("ab".to_string(), "aa".to_string()),
            false
        );
    }
}