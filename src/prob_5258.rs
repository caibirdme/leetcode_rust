
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let n = words.len();
        if n == 0 {
            return 0;
        }
        if letters.is_empty() {
            return 0;
        }
        let mut letter_count = vec![0; 26];
        for c in letters {
            letter_count[(c as u8 - b'a') as usize] += 1;
        }
        let mut used = vec![false; n];
        let mut ans = 0;
        Self::dfs(0, &words, &mut used, &letter_count, &score, &mut ans);
        ans
    }
    fn dfs(cur: usize, words: &Vec<String>, used: &mut Vec<bool>, char_count: &Vec<i32>, score: &Vec<i32>, ans: &mut i32) {
        if cur >= words.len() {
            let mut count = vec![0; 26];
            let mut get_score = 0;
            for i in 0..words.len() {
                if used[i] {
                    for &c in words[i].as_bytes() {
                        let pos = (c - b'a') as usize;
                        count[pos] += 1;
                        get_score += score[pos];
                        if count[pos] > char_count[pos] {
                            return;
                        }
                    }
                }
            }
            if get_score > *ans {
                *ans = get_score;
            }
            return;
        }
        for i in cur..words.len() {
            Self::dfs(i+1, words, used, char_count, score, ans);
            used[i] = true;
            Self::dfs(i+1, words, used, char_count, score, ans);
            used[i] = false;
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_score_words() {
        let test_cases = vec![
            (vec!["dog","cat","dad","good"], vec!['a','a','c','d','d','d','g','o','o'], vec![1,0,9,5,0,0,3,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0], 23),
        ];
        for (words, letters, scores, expect) in test_cases {
            assert_eq!(Solution::max_score_words(words.iter().map(|s| s.to_string()).collect(), letters.clone(), scores.clone()), expect, "words: {:?}, letters: {:?}", words, letters);
        }
    }
}