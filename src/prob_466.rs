use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let cols = s1.len();
        let un2 = n2 as usize;
        let rows = n1 as usize;
        if cols * rows < s2.len() * (n2 as usize) {
            return 0;
        }
        let bs1 = s1.as_bytes();
        let bs2 = s2.as_bytes();
        let lens2 = bs2.len();
        let set: HashSet<_> = bs1.iter().map(|v| *v).collect();
        for c in bs2 {
            if !set.contains(c) {
                return 0;
            }
        }
        let mut ans = 0;
        let mut row = 0;
        let mut idx = 0;
        let mut hash = HashMap::new();
        while row < rows {
            let mut col = 0;
            while col < cols {
                while col < cols && bs1[col] != bs2[idx] {
                    col+=1;
                }
                if col == cols {
                    break;
                }
                idx += 1;
                if idx == lens2 {
                    ans += 1;
                    idx = 0;
                }
                col += 1;
            }
            if let Some((rr, t)) = hash.get(&(col, idx)) {
                let skip_r = row - *rr;
                let r_ans = ans - *t;
                let cycle = (rows-1-row) / skip_r;
                ans += cycle * r_ans;
                row += cycle*skip_r;
            } else {
                hash.insert((col, idx), (row, ans));
            }
            row += 1;
        }
        (ans / un2) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_max_repetitions() {
        let test_cases = vec![
            ("aaa", 3, "aa", 1, 4),
            ("abc", 6, "acb", 2, 1),
            ("abc", 8, "acb", 2, 2),
            ("acb", 4, "ab", 2, 2),
        ];
        for (s1,n1,s2,n2,expect) in test_cases {
            assert_eq!(Solution::get_max_repetitions(s1.to_string(),n1,s2.to_string(),n2),expect,"s1: {}, ni:{}, s2: {}, n2:{}",s1,n1,s2,n2);
        }
    }
}