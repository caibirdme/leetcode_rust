impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut count = [0; 26];
        for &c in s.as_bytes() {
            count[(c-b'a') as usize] += 1;
        }
        let mut number = [0; 10];
        // the order is very critical
        let unique = vec![
            (b'w', "two", 2),
            (b'u', "four", 4),
            (b'x', "six", 6),
            (b'g', "eight", 8),
            (b'z', "zero", 0),
            (b's', "seven", 7),
            (b'h', "three", 3),
            (b'o', "one", 1),
            (b'f', "five", 5),
            (b'i', "nine", 9),
        ];
        for (c, s, v) in unique {
            let t = count[(c- b'a') as usize];
            for &p in s.as_bytes() {
                count[(p-b'a')as usize] -= t;
            }
            number[v] = t;
        }
        let mut ans = String::new();
        for i in 0..=9usize {
            if number[i] > 0 {
                ans += i.to_string().repeat(number[i]).as_str();
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
    fn test_original_digits() {
        let test_cases = vec![
            ("owoztneoeroruf", "0124"),
            ("fviefuro", "45"),
        ];
        for (s, expect) in test_cases {
            assert_eq!(expect, Solution::original_digits(s.clone().to_string()), "s: {}", s);
        }
    }
}

