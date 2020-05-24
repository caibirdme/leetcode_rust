
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut count = 0;
        for i in 0..k as usize {
            if Self::is_vowel(s[i]) {
                count += 1;
            }
        }
        let mut ans = count;
        let mut i = 0;
        let mut j = k as usize;
        while j < s.len() {
            if Self::is_vowel(s[i]) {
                count-=1;
            }
            if Self::is_vowel(s[j]) {
                count+=1;
                ans = ans.max(count);
            }
            i+=1;
            j+=1;
        }
        ans
    }
    fn is_vowel(c: u8) -> bool {
        match c {
            b'a'|b'e'|b'i'|b'o'|b'u' => true,
            _ => false,
        }
    }
}

struct Solution;