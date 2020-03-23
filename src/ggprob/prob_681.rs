impl Solution {
    pub fn next_closest_time(time: String) -> String {
        use std::str::from_utf8_unchecked;
        let s = time.as_bytes();
        let mut d = vec![0; 4];
        d[0] = s[0];
        d[1] = s[1];
        d[2] = s[3];
        d[3] = s[4];
        d.sort();
        if d[0] == d[3] {
            return time;
        }
        let mut ans = Vec::from(s);
        for j in (0..=4).rev() {
            if j != 2 {
                for i in 0..4 {
                    if d[i] > ans[j] {
                        if j == 3 && d[i] > b'5' ||
                            j == 0 && d[i] > b'2' {
                            break;
                        }
                        if j == 1 && d[i] > b'3' && ans[0] == b'2'{
                            break;
                        }
                        ans[j] = d[i];
                        for k in j+1..=4 {
                            if k != 2 {
                                ans[k] = d[0];
                            }
                        }
                        return unsafe {from_utf8_unchecked(&ans).to_string()};
                    }
                }
            }
        }
        for i in 0..=4 {
            if i != 2 {
                ans[i] = d[0];
            }
        }
        return unsafe {from_utf8_unchecked(&ans).to_string()};
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_next_closest_time() {
        let test_cases = vec![
            ("23:45", "23:52"),
            ("23:44", "22:22"),
            ("07:08", "08:00"),
            ("19:34", "19:39"),
            ("23:59", "22:22"),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::next_closest_time(s.to_string()), expect, "s: {}", s);
        }
    }
}