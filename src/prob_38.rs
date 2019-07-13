impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let preset = vec!["1", "11", "21", "1211", "111221"];
        if n <= 5 {
            return preset[n as usize-1].to_string();
        }
        let mut pre = "111221".to_string();
        for i in 6..=n {
            pre = Self::count(pre.as_bytes());
        }
        pre
    }
    fn count(s: &[u8]) -> String {
        let mut res = String::new();
        let n = s.len();
        let mut i = 0;
        while i < n {
            let c = s[i];
            if i == n-1 {
                res.push_str(format!("1{}",c as char).as_str());
                return res;
            }
            let mut t = 1;
            let mut j = i+1;
            while j < n && s[j] == s[i]{
                t+=1;
                j+=1;
            }
            res.push_str(format!("{}{}", t, c as char).as_str());
            i = j;
        }
        res
    }
}

struct Solution;


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_and_say() {
        let test_cases = vec![
            (6, "312211"),
            (7, "13112221"),
            (8, "1113213211"),
            (9, "31131211131221"),
            (10, "13211311123113112211"),
            (11, "11131221133112132113212221"),
        ];
        for (n, expecpt) in test_cases {
            assert_eq!(Solution::count_and_say(n), expecpt, "n: {}", n);
        }
    }
}