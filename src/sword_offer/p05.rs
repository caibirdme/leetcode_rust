impl Solution {
    pub fn replace_space(s: String) -> String {
        let bs = s.as_bytes();
        let mut new_s = vec![];
        let target = vec![b'%',b'2',b'0'];
        for &c in bs {
            if c == b' ' {
                new_s.append(&mut target.clone());
            } else {
                new_s.push(c);
            }
        }
        unsafe {std::str::from_utf8_unchecked(&new_s).to_string()}
    }
}

struct Solution;