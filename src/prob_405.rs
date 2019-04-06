use std::collections::HashMap;

impl Solution {
    pub fn to_hex(mut num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        if num == std::i32::MIN {
            return "80000000".to_string();
        }
        if num > 0 {
            let mut ans = String::new();
            while num > 0 {
                ans.push(Self::i32_to_char(num%16));
                num /= 16;
            }
            return ans.chars().rev().collect();
        }
        let pow_2_32= 0x80000000 as i64;
        let new_num = (pow_2_32+num as i64) as i32;
        let mut ans = Self::to_hex(new_num);
        let mut bytes = unsafe {ans.as_bytes_mut()};
        let last = bytes.first_mut().unwrap();
        *last = Self::i32_to_char(((*last - b'0') | 8u8) as i32) as u8;
        String::from_utf8_lossy(bytes).to_string()
    }
    fn i32_to_char(n: i32) -> char {
        match n {
            0...9 => (n as u8 + b'0') as char,
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            _ => unreachable!(),
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_to_hex() {
        assert_eq!(format!("{:x}", std::i32::MIN), Solution::to_hex(std::i32::MIN), "i:{}",std::i32::MIN);
        for i in -5000..5000 {
            assert_eq!(format!("{:x}", i), Solution::to_hex(i), "i:{}",i);
        }
        assert_eq!(format!("{:x}", std::i32::MAX), Solution::to_hex(std::i32::MAX), "i:{}",std::i32::MAX);
    }
}