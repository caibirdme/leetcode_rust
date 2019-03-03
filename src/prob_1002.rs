use std::collections::HashMap;
use std::cmp::min;

impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let n = a.len();
        if n == 1 {
            return a[0].chars().map(|c| c.to_string()).collect();
        }
        let mut all = HashMap::new();
        a[0].chars().for_each(|c| {
            let count = all.entry(c).or_insert(0);
            *count += 1;
        });
        let mut cur = HashMap::new();
        let mut list: Vec<char> = vec![];
        for i in 1..n {
            cur.clear();
            a[i].chars().for_each(|c| {
                let count = cur.entry(c).or_insert(0);
                *count += 1;
            });
            list.clear();
            all.iter_mut().for_each(|(k,v)| {
                match cur.get(k) {
                    Some(&v2) => {*v = min(*v, v2);},
                    None => {list.push(*k);},
                }
            });
            for c in list.iter() {
                all.remove(c);
            }
        }
        let mut ans = vec![];
        for (k,v) in all.into_iter() {
            for i in 0..v {
                ans.push(k.to_string());
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
    fn test_common_chars() {
        let test_cases = vec![
            (vec!["bella","label","roller"], vec!["e","l","l"]),
            (vec!["cool","lock","cook"], vec!["c", "o"])
        ];
        for (a, expect) in test_cases {
            assert_eq!(
                Solution::common_chars(a.into_iter().map(|s| s.to_string()).collect()),
                expect.into_iter().map(|s| s.to_string()).collect::<Vec<String>>()
            );
        }
    }
}