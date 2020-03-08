impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut trans = vec![vec![0; 1<<7]; 1<<7];
        for allow in allowed {
            let s = allow.as_bytes();
            let (x,y,z) = (1<<(s[0]-b'A'), 1<<(s[1]-b'A'), 1<<(s[2]-b'A'));
            for i in 0..(1<<7) {
                if i&(x as usize) > 0 {
                    for j in 0..(1<<7) {
                        if j&(y as usize) > 0 {
                            trans[i][j] |= z;
                        }
                    }
                }
            }
        }
        let mut state = vec![];
        for &c in bottom.as_bytes() {
            state.push(1<<(c-b'A') as usize);
        }
        for i in (0..bottom.len()).rev() {
            for j in 0..i {
                state[j] = trans[state[j]][state[j+1]];
            }
        }
        state[0] > 0
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_pyramid_transition() {
        let test_cases = vec![
            ("BCD", vec!["BCG", "CDE", "GEA", "FFF"], true),
            ("AABA", vec!["AAA", "AAB", "ABA", "ABB", "BAC"], false),
        ];
        for (bottom, allowed, ok) in test_cases {
            assert_eq!(Solution::pyramid_transition(bottom.to_string(), allowed.iter().map(|v| v.to_string()).collect()), ok, "bottom: {}, allowed: {:?}", bottom, allowed);
        }
    }
}