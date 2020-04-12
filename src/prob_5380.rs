impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let n = words.len();
        let mut ans = vec![];
        for i in 0..n {
            for j in 0..n {
                if j != i && words[i].len() < words[j].len() {
                    if words[j].contains(&words[i]) {
                        ans.push(words[i].clone());
                        break;
                    }
                }
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
    fn test_string_matching() {
        let test_cases = vec![
            (vec!["mass","as","hero","superhero"], vec!["as", "hero"]),
        ];
        for (arr, expect) in test_cases {
            assert_eq!(Solution::string_matching(arr.iter().map(|v| v.to_string()).collect()), expect);
        }
    }
}