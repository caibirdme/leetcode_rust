impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let ts = target.to_string();
        match Self::calc(num.as_bytes(), 0,0,b'+',target as i64, true) {
            None => vec![],
            Some(val) => {
                val.iter().map(|s| s[1..].to_string()).collect()
            },
        }
    }

    fn calc(num: &[u8], cur: i64, pre: i64, pre_op: u8, t: i64, flag: bool) -> Option<Vec<String>> {
        use std::str::from_utf8;
        let n = num.len();
        if n == 0 {
            if cur == t {
                return Some(vec!["".to_string()]);
            }
            return None;
        }
        let mut acc = 0;
        let mut output = vec![];
        for i in 0..n {
            let c = num[i];
            acc = acc*10 + (c-b'0') as i64;
            if let Some(res) = Self::calc(&num[i+1..], cur+acc, acc, b'+',t, false) {
                for s in res {
                    output.push( "+".to_string() + from_utf8(&num[..i+1]).unwrap() + s.as_str());
                }
            }
            if !flag {
                if let Some(res) = Self::calc(&num[i+1..],cur-acc, acc, b'-',t, false) {
                    for s in res {
                        output.push("-".to_string() + from_utf8(&num[..i+1]).unwrap() + s.as_str());
                    }
                }
                match pre_op {
                    b'+' => {
                        if let Some(q) = pre.checked_mul(acc) {
                            if let Some(nc_1) = cur.checked_sub(pre) {
                                if let Some(nc) = nc_1.checked_add(q) {
                                    if let Some(res) = Self::calc(&num[i + 1..], cur - pre + q, q, pre_op, t, false) {
                                        for s in res {
                                            output.push("*".to_string() + from_utf8(&num[..i + 1]).unwrap() + s.as_str());
                                        }
                                    }
                                }
                            }
                        }
                    },
                    b'-' => {
                        if let Some(q) = pre.checked_mul(acc) {
                            if let Some(nc_1) = cur.checked_add(pre) {
                                if let Some(nc) = nc_1.checked_sub(q) {
                                    if let Some(res) = Self::calc(&num[i + 1..], nc, q, pre_op, t, false) {
                                        for s in res {
                                            output.push("*".to_string() + from_utf8(&num[..i + 1]).unwrap() + s.as_str());
                                        }
                                    }
                                }
                            }
                        }
                    },
                    _ => unreachable!(),
                }
            }
            if acc == 0 {
                break;
            }
        }
        if output.is_empty() {
            None
        } else {
            Some(output)
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_add_operators() {
        let mut test_cases = vec![
            ("105", 5, vec!["1*0+5".to_string(), "10-5".to_string()]),
            ("232", 8, vec!["2*3+2".to_string(), "2+3*2".to_string()]),
            ("00", 0, vec!["0+0".to_string(), "0-0".to_string(), "0*0".to_string()]),
            ("3456237490", 9191, vec![]),
            ("12", 12, vec!["12".to_string()]),
            ("112", 12, vec!["1*12".to_string()]),
            ("2147483647", 2147483647, vec!["2147483647".to_string()]),
        ];
        for (s,t,mut expect) in test_cases {
            let mut actual = Solution::add_operators(s.to_string(), t,);
            actual.sort();
            expect.sort();
            assert_eq!(actual,expect);
        }
    }
}