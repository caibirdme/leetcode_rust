impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count = vec![0; 256];
        for &c in magazine.as_bytes() {
            count[c as usize] += 1;
        }
        for &c in ransom_note.as_bytes() {
            count[c as usize] -= 1;
            if count[c as usize] < 0 {
                return false;
            }
        }
        true
    }
}

struct Solution;