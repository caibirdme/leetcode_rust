impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let n = preorder.len();
        if n == 0 {
            return true;
        }
        let mut ok = true;
        let mut s: Vec<&str> = preorder.as_str().split(',').collect();
        Self::visit(&mut s, &mut ok);
        ok && s.len() == 0
    }
    fn visit(s: &mut Vec<&str>, ok: &mut bool) {
        if s.len() == 0 {
            *ok = false;
            return;
        }
        let cur = s[0];
        s.remove(0);
        if cur != "#" {
            Self::visit(s, ok);
            Self::visit(s, ok);
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_is_valid_serialization() {
        let test_cases = vec![
            ("9,3,4,#,#,1,#,#,2,#,6,#,#", true),
            ("1,#", false),
            ("1", false),
            ("1123,#,#", true),
            ("1123,#,123123,#,#", true),
            ("9,#,#,1", false),
        ];
        for (s,ok) in test_cases {
            assert_eq!(
                Solution::is_valid_serialization(s.to_string()),
                ok
            );
        }
    }
}