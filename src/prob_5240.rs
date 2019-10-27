use std::collections::HashSet;
use std::cmp::max;

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        if arr.is_empty() {
            return 0;
        }
        let all_s: Vec<String> = arr.into_iter().filter(|v| {
            let mut used = HashSet::new();
            for c in v.as_bytes() {
                if used.contains(c) {
                    return false;
                }
                used.insert(*c);
            }
            true
        }).collect();
        let mut set = HashSet::new();
        let mut ans = 0;
        Self::dfs(&all_s, &mut set, 0, &mut ans);
        ans as i32
    }
    fn dfs(arr: &Vec<String>, set: &mut HashSet<u8>, idx: usize, ans: &mut usize) {
        for i in idx..arr.len() {
            if set.len() + arr[i].len() > 26 {
                continue;
            }
            if Self::not_in_set(set, &arr[i]) {
                for c in arr[i].as_bytes() {
                    set.insert(*c);
                }
                *ans = max(*ans, set.len());
                Self::dfs(arr, set, i+1, ans);
                for c in arr[i].as_bytes() {
                    set.remove(c);
                }
            }
        }
    }
    fn not_in_set(set: &HashSet<u8>, s: &String) -> bool {
        for c in s.as_bytes() {
            if set.contains(c) {
                return false;
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
    fn test_max_length() {
        let test_cases = vec![
            (vec!["un","iq","ue"], 4),
            (vec!["cha","r","act","ers"], 6),
            (vec!["abcdefghijklmnopqrstuvwxyz"], 26),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::max_length(s.iter().map(|v| v.to_string()).collect()), expect, "s: {:?}", s);
        }
    }
}