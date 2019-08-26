use std::collections::HashMap;

impl Solution {
    pub fn generate_palindromes(s: String) -> Vec<String> {
        if s.is_empty() {
            return vec![];
        }
        if s.len() == 1 {
            return vec![s];
        }
        let mut count = HashMap::new();
        for c in s.chars() {
            let p = count.entry(c).or_insert(0);
            *p += 1;
        }
        let mut odd = 0;
        let mut mid = None;
        let mut total = 0;
        for (&k,v) in count.iter_mut() {
            if *v & 1 == 1 {
                odd+=1;
                *v /= 2;
                total += *v;
                if odd > 1 {
                    return vec![];
                }
                mid = Some(k);
            } else {
                *v /= 2;
                total += *v;
            }
        }
        let mut cur = vec![];
        let mut ans = vec![];
        let mut chars = vec![];
        count.iter().for_each(|(&k,&v)| {
            for _ in 0..v {
                chars.push(k);
            }
        });
        let mut used = vec![false; chars.len()];
        Self::gen(&chars, &mut used, &mut cur, &mid, &mut ans);
        ans
    }
    fn gen(chars: &Vec<char>, used: &mut Vec<bool>, cur: &mut Vec<char>, mid: &Option<char>, ans: &mut Vec<String>) {
        if cur.len() >= chars.len() {
            let mut v = cur.clone();
            if let Some(c) = mid {
                v.push(*c);
            }
            cur.iter().rev().for_each(|&c| v.push(c));
            ans.push(v.into_iter().collect());
            return;
        }
        for i in 0..chars.len() {
            if used[i] {
                continue;
            }
            if i > 0 && chars[i] == chars[i-1] && !used[i-1] {
                continue;
            }
            used[i] = true;
            cur.push(chars[i]);
            Self::gen(chars,used,cur,mid,ans);
            cur.pop();
            used[i] = false;
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_generate_palindromes() {
        let test_cases = vec![
            ("aaaabbbb", vec!["aabbbbaa","ababbaba","abbaabba","baabbaab","babaabab","bbaaaabb"]),
            ("aaaaaa", vec!["aaaaaa"]),
            ("aaaa", vec!["aaaa"]),
            ("abbba", vec!["abbba", "babab"]),
            ("aab", vec!["aba"]),
            ("aabb", vec!["abba", "baab"]),
            ("abc", vec![]),
        ];
        for (s, expect) in test_cases {
            let mut ans = Solution::generate_palindromes(s.to_string());
            ans.sort();
            assert_eq!(ans, expect, "s: {}", s);
        }
    }
}