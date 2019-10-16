impl Solution {
    pub fn generate_abbreviations(word: String) -> Vec<String> {
        if word.is_empty() {
            return vec!["".to_string()];
        }
        if word.len() == 1 {
            return vec![word, "1".to_string()];
        }
        let mut cur: Vec<char> = vec![];
        let mut ans = vec![];
        Self::dfs(&word, &mut cur, &mut ans, 0, 0);
        ans
    }
    fn dfs(word: &str, cur: &mut Vec<char>, ans: &mut Vec<String>, pos: usize, num: i32) {
        if pos == word.len() {
            let s = cur.iter().collect::<String>();
            if num > 0 {
                ans.push(s + num.to_string().as_str());
            } else {
                ans.push(s);
            }
            return;
        }
        let origin_len = cur.len();
        // reserve
        if num > 0 {
            num.to_string().as_bytes().into_iter().for_each(|c| cur.push(*c as char));
        }
        cur.push(word.as_bytes()[pos] as char);
        Self::dfs(word, cur, ans, pos+1, 0);

        // abbr
        cur.truncate(origin_len);
        Self::dfs(word, cur, ans, pos+1, num+1);
        cur.truncate(origin_len);
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_generate_abbreviations() {
        let test_cases = vec![
            ("ab", vec!["ab", "a1", "1b", "2"]),
            ("word", vec!["word", "1ord", "w1rd", "wo1d", "wor1", "2rd", "w2d", "wo2", "1o1d", "1or1", "w1r1", "1o2", "2r1", "3d", "w3", "4"]),
            ("a", vec!["a", "1"]),
            ("", vec![""]),
        ];
        for (word, mut expect) in test_cases {
            let mut actual = Solution::generate_abbreviations(word.to_string());
            actual.sort();
            expect.sort();
            assert_eq!(actual, expect, "word: {}", word);
        }
    }
}