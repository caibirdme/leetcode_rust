use std::collections::HashMap;

impl Solution {
    pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut single = vec![];
        let mut hash = HashMap::new();
        let mut tmp = vec![];
        for s in strings {
            if s.len() == 1 {
                single.push(s);
                continue;
            }
            let st = s.as_bytes();
            tmp.clear();
            for i in 1..st.len() {
                if st[i] >= st[i-1] {
                    tmp.push(st[i]-st[i-1]);
                } else {
                    tmp.push(st[i]+26-st[i-1]);
                }
            }
            let list = hash.entry(tmp.clone()).or_insert(vec![]);
            list.push(s);
        }
        let mut ans: Vec<Vec<String>> = hash.values().map(|v| v.clone()).collect();
        if !single.is_empty() {
            ans.push(single);
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_group_strings() {
        let test_cases = vec![
            (vec!["abc", "bcd", "acef", "xyz", "az", "ba", "a", "z"], 4),
        ];
        for (strings, num) in test_cases {
            let q = dbg!(Solution::group_strings(strings.into_iter().map(|v| v.to_string()).collect()));
            assert_eq!(q.len(), num);

        }
    }
}