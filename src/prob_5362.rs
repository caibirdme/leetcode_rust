impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let n = s.len();
        let k = k as usize;
        if n == 0 || n < k {
            return false;
        }
        if n == k {
            return true;
        }
        let mut count = [0; 26];
        for &c in s.as_bytes() {
            count[(c-b'a') as usize] += 1;
        }
        let mut odd_num = 0;
        for i in 0..26 {
            if count[i] & 1 == 1 {
                odd_num += 1;
            }
        }
        odd_num <= k
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_can_construct() {
        let test_cases = vec![
            ("leetcode", 3, false),
            ("yzyzyzyzyzyzyzy", 2, true),
            ("true", 4, true),
            ("cr", 7, false),
            ("annabelle", 2, true),
        ];
        for (s, k, ok) in test_cases {
            assert_eq!(Solution::can_construct(s.to_string(), k), ok, "s: {}, k:{}",s,k);
        }
    }
}