use std::cmp::max;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        if word1.is_empty() {
            return word2.len() as i32;
        }
        if word2.is_empty() {
            return word1.len() as i32;
        }
        if word1.len() < word2.len() {
            return Self::min_distance(word2, word1);
        }
        let m = word2.len();
        let w2: Vec<&u8> = word2.as_bytes().iter().collect();
        let mut f = [vec![0; m+1], vec![0; m+1]];
        let mut idx = 0;
        for &c1 in word1.as_bytes() {
            let np = idx ^ 1;
            for j in 0..m {
                let lj = j+1;
                let &c2 = w2[j];
                if c1 == c2 {
                    f[idx][lj] = f[np][j] + 1;
                } else {
                    f[idx][lj] = max(f[np][lj], f[idx][j]);
                }
            }
            idx = np;
        }
        (word1.len() + m - f[idx^1][m]*2) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_distance() {
        let test_cases = vec![
            ("sea", "eat", 2),
            ("hello", "helper", 5),
            ("hello", "help", 3),
            ("I'm Lily", "I'm Lil", 1),
        ];
        for (s1, s2, expect) in test_cases {
            assert_eq!(Solution::min_distance(s1.to_string(), s2.to_string()), expect);
        }
    }
}