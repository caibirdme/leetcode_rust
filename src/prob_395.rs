impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        if k == 1 {
            return s.len() as i32;
        }
        Self::search(s.as_bytes(), k as usize)
    }
    fn search(s: &[u8], k: usize) -> i32 {
        if s.len() < k {
            return 0;
        }
        let mut count = [0;26];
        for i in 0..s.len() {
            count[(s[i]-b'a')as usize] += 1;
        }
        let mut offset = 0;
        let mut min = std::i32::MAX as usize;
        for i in 0..26 {
            if count[i] > 0 && count[i] < min {
                min = count[i];
                offset = i;
            }
        }
        if min >= k {
            return s.len() as i32;
        }
        let mut idx_list = Vec::with_capacity(min);
        let target = offset as u8+b'a';
        for i in 0..s.len() {
            if s[i] == target {idx_list.push(i);}
        }
        idx_list.push(s.len());
        let mut ans = 0;
        let mut pre = 0;
        for idx in idx_list {
            ans = std::cmp::max(ans, Self::search(&s[pre..idx], k));
            pre = idx+1;
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_longest_substring() {
        let test_cases = vec![
            ("bbaaacbd", 3, 3),
            ("ababbcc", 2, 7),
            ("aaabb", 3, 3),
            ("ababbc", 2, 5),
            ("ababbc", 4, 0),
            ("ababbccac", 2, 9),
            ("abcdefghi", 1, 9),
        ];
        for (s,k,expect) in test_cases {
            assert_eq!(expect, Solution::longest_substring(s.to_string(), k), "s: {}, k: {}", s, k);
        }
    }
}