impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut data = vec![];
        let mut acc = 0;
        let bs = s.as_bytes();
        for i in (0..s.len()).rev() {
            if bs[i] == b'-' {
                continue;
            }
            data.push(Self::convert(bs[i]));
            acc += 1;
            if acc == k {
                data.push(b'-');
                acc = 0;
            }
        }
        if let Some(&last) = data.last() {
            if last == b'-' {
                data.pop();
            }
        }
        if data.is_empty() {
            return "".to_string();
        }
        let mut i = 0;
        let mut j = data.len()-1;
        while i < j {
            data.swap(i,j);
            i+=1;
            j-=1;
        }
        unsafe {std::str::from_utf8_unchecked(&data).to_string()}
    }
    fn convert(b: u8) -> u8 {
        if b <= b'z' && b>=b'a' {
            b-b'a'+b'A'
        } else {
            b
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_license_key_formatting() {
        let test_cases = vec![
            ("5F3Z-2e-9-w", 4, "5F3Z-2E9W"),
            ("2-5g-3-J", 2, "2-5G-3J"),
        ];
        for (original, k, expect) in test_cases {
            assert_eq!(Solution::license_key_formatting(original.to_string(), k), expect, "s: {}", original);
        }
    }
}