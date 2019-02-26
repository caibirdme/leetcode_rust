impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let n = words.len();
        if n < 2 {
            return 0;
        }
        let mut num = vec![0; n];
        for i in 0..n {
            let mut m = 0;
            for c in words[i].bytes() {
                m |= 1<<(c - b'a') as usize;
            }
            num[i] = m;
        }
        let mut ans = 0;
        for i in 0..n-1 {
            for j in i+1..n {
                if num[i]&num[j] == 0 {
                    ans = ans.max(words[i].len()*words[j].len());
                }
            }
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_max_product() {
        let test_cases = vec![
            (vec!["abcw","baz","foo","bar","xtfn","abcdef"], 16),
            (vec!["a","ab","abc","d","cd","bcd","abcd"], 4),
            (vec!["a","aa","aaa","aaaa"], 0),
            (vec!["eae","ea","aaf","bda","fcf","dc","ac","ce","cefde","dabae"], 15),
        ];
        for (words, expect) in test_cases {
            let mut input: Vec<String> = words.into_iter().map(|x| x.to_string()).collect();
            assert_eq!(Solution::max_product(input), expect);
        }
    }
}