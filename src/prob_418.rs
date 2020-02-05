use std::collections::HashMap;

impl Solution {
    pub fn words_typing(sentence: Vec<String>, rows: i32, cols: i32) -> i32 {
        let mut idx = 0;
        let mut f: HashMap<(i32, usize), (i32, i32)> = HashMap::new();
        let mut row = 0;
        let mut ans = 0;
        while row < rows {
            let mut col = 0;
            while col+(sentence[idx].len() as i32) <= cols {
                col += (sentence[idx].len() as i32) + 1;
                idx += 1;
                if idx == sentence.len() {
                    idx = 0;
                    ans += 1;
                }
            }
            if let Some((step_rows, repeat_times)) = f.get(&(col, idx)) {
                let this_repeat_times = ans-*repeat_times;
                let skip_rows = row - *step_rows;
                let cycle = (rows-1-row)/ skip_rows;
                ans += cycle * this_repeat_times;
                row += cycle * skip_rows;
            } else {
                f.insert((col, idx), (row, ans));
            }
            row += 1;
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_words_typing() {
        let test_cases = vec![
            (2, 8, vec!["hello", "world"], 1),
            (3, 6, vec!["a", "bcd", "e"], 2),
            (4, 5, vec!["I", "had", "apple", "pie"], 1),
        ];
        for (rows, cols, words, expect) in test_cases {
            assert_eq!(Solution::words_typing(words.iter().map(|v| v.to_string()).collect(), rows, cols), expect, "sentence: {:?}, rows:{}, cols:{}",words,rows,cols);
        }
    }
}