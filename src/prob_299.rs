impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        if secret.is_empty() || guess.is_empty() {
            return "".to_string();
        }
        use std::cmp::min;
        let mut count_a = [0; 10];
        let mut count_b = [0; 10];
        let mut ans_a = 0;
        let mut ans_b = 0;
        for (&a,&b) in secret.as_bytes().iter().zip(guess.as_bytes()) {
            if a == b {
                ans_a+=1
            } else {
                count_a[(a-b'0') as usize] += 1;
                count_b[(b-b'0') as usize] += 1;
            }
        }
        for i in 0..10 as usize {
            ans_b += min(count_a[i], count_b[i]);
        }
        format!("{}A{}B",ans_a,ans_b)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_get_hint() {
        assert_eq!(
            Solution::get_hint("1807".to_string(), "7810".to_string()),
            "1A3B".to_string()
        );
        assert_eq!(
            Solution::get_hint("1123".to_string(), "0111".to_string()),
            "1A1B".to_string()
        );
    }
}