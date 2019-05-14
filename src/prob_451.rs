use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut count = HashMap::new();
        for c in s.chars() {
            *count.entry(c).or_insert(0) += 1;
        }
        let mut arr:Vec<(char, i32)> = count.into_iter().map(|(k,v)| (k,v)).collect();
        arr.sort_by(|a,b| b.1.cmp(&a.1));
        let mut res = String::new();
        for (k,v) in arr {
            res += k.to_string().repeat(v as usize).as_str();
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_frequency_sort() {
        let test_data = vec![
            ("cccaaaa", "aaaaccc"),
            ("trreee", "eeerrt"),
            ("", ""),
            ("a", "a"),

        ];
        for (input, expect) in test_data {
            assert_eq!(expect, Solution::frequency_sort(input.to_string()));
        }
    }
}