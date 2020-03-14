impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let n = preorder.len();
        if n == 0 {
            return true;
        }
        let mut s: Vec<&str> = preorder.as_str().split(',').collect();
        if s[0] == "#" {
            return s.len() == 1;
        }

        // is_backtrace
        let mut q = vec![false];
        let mut i = 0;
        while let Some(is_back) = q.pop() {
            i+=1;
            if i >= s.len() {
                return false;
            }
            if is_back {
                if s[i] != "#" {
                    q.push(false);
                }
            } else {
                q.push(true);
                if s[i] != "#" {
                    q.push(false);
                }
            }
        }
        i+1 == s.len()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_is_valid_serialization() {
        let test_cases = vec![
            ("#", true),
            ("1,#", false),
            ("9,3,4,#,#,1,#,#,2,#,6,#,#", true),
            ("1", false),
            ("1123,#,#", true),
            ("1123,#,123123,#,#", true),
            ("9,#,#,1", false),
        ];
        for (s,ok) in test_cases {
            assert_eq!(
                Solution::is_valid_serialization(s.to_string()),
                ok,
                "s: {}",
                s
            );
        }
    }
}