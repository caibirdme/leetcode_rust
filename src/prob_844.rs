impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut q1 = vec![];
        let mut q2 = vec![];
        s.as_bytes().iter().for_each(|&c| {
            if c != b'#' {
                q1.push(c);
            } else {
                q1.pop();
            }
        });
        t.as_bytes().iter().for_each(|&c| {
            if c != b'#' {
                q2.push(c);
            } else {
                q2.pop();
            }
        });
        if q1.len() != q2.len() {
            return false;
        }
        for (c1, c2) in q1.into_iter().zip(q2.into_iter()) {
            if c1 != c2 {
                return false;
            }
        }
        true
    }
}

struct Solution;