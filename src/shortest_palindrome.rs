struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let length = s.len();
        if length <= 1 {
            return s;
        }
        let mut s2 = s.clone();
        s2 += "#";
        s2 += s.chars().rev().collect::<String>().as_str();
        let ss = s2.as_bytes();
        let length2 = ss.len();
        let mut next: Vec<isize>= vec![-1; length2];
        let mut k = -1;
        for i in 1..length2 {
            while k > -1 && ss[i] != ss[(k+1) as usize] {
                k = next[k as usize];
            }
            if ss[i] == ss[(k+1) as usize] {
                k += 1;
            }
            next[i] = k;
        }
        let &t = next.last().unwrap();
        s[(t+1) as usize..].chars().rev().collect::<String>() + s.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_shortest_palindrome() {
        assert_eq!(
            Solution::shortest_palindrome("aacecaaa".to_string()),
            "aaacecaaa".to_string()
        );
        assert_eq!(
            Solution::shortest_palindrome("abcd".to_string()),
            "dcbabcd".to_string()
        );
        assert_eq!(
            Solution::shortest_palindrome("aaa".to_string()),
            "aaa".to_string()
        );
        assert_eq!(
            Solution::shortest_palindrome("ab".to_string()),
            "bab".to_string()
        );
        assert_eq!(
            Solution::shortest_palindrome("a".to_string()),
            "a".to_string()
        );
    }
}