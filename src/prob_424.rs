use std::cmp::max;

macro_rules! pos {
    ($e:expr) => {
        ($e-b'A') as usize
    };
}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let n = s.len();
        let k = k as usize;
        if k+1 >= n {
            return n as i32;
        }
        let mut count = [0; 26];
        let (mut left, mut right) = (0,0);
        let mut ans = 1;
        let mut cur_max = 0;
        let s = s.as_bytes();
        while right < n {
            count[pos!(s[right])] += 1;
            cur_max = max(cur_max, count[pos!(s[right])]);
            while right-left+1>cur_max+k {
                count[pos!(s[left])] -= 1;
                left+=1;
            }
            ans = max(ans, right-left+1);
            right+=1;
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_character_replacement() {
        let test_cases = vec![
            ("ABBBA", 2, 5),
            ("ABAB", 1, 3),
            ("ABAB", 2, 4),
            ("ABCDAA", 3, 6),
            ("ABCDAA", 2, 4),
            ("ABBCDAA", 2, 4),
            ("ABBADAA", 3, 7),
        ];
        for (s, k, expect) in test_cases {
            assert_eq!(expect, Solution::character_replacement(s.clone().to_string(), k), "s: {}, k:{}", s, k);
        }
    }
}