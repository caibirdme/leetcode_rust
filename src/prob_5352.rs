impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n == 0 {
            return "a".to_string();
        }
        if n&1 == 1 {
            "a".repeat(n as usize)
        } else {
            let x = n-1;
            let y = 1;
            "a".repeat(x as usize) + "b"
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_generate_the_string() {
        let test_cases = vec![
            (4, "aaab"),
            (2, "ab"),
            (7, "aaaaaaa"),
        ];
        for (n, expect) in test_cases {
            assert_eq!(Solution::generate_the_string(n), expect, "n:{}",n);
        }
    }
}