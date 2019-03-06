impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        let n = s.len();
        if n < 2 {
            return s;
        }
        let mut b = unsafe {s.as_mut_vec()};
        let (mut l,mut r) = (0,n-1);
        loop {
            while l < n-1 && !Self::is_vowel(b[l]) {l+=1;}
            while r > 0 && !Self::is_vowel(b[r]) {r-=1;}
            if l >= r {
                break;
            }
            b.swap(l,r);
            l+=1;
            r-=1;
        }
        unsafe{String::from_utf8_unchecked(b.to_vec())}
    }
    #[inline]
    fn is_vowel(c: u8) -> bool {
        match c {
            b'a'|b'e'|b'i'|b'o'|b'u'|b'A'|b'E'|b'I'|b'O'|b'U' => true,
            _ => false,
        }
    }
}

struct Solution;