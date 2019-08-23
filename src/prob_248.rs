impl Solution {
    pub fn strobogrammatic_in_range(low: String, high: String) -> i32 {
        let mut ans = 0;
        let mut cur = vec![];
        let (ll, hl) = (low.len(), high.len());
        for i in ll+1..hl {
            let mut t: usize = 4 * 5usize.pow((i/2-1) as u32);
            if i & 1 == 1 {
                t *= 3
            }
            ans += t as i32;
        }
        let vec_low:Vec<char> = low.as_bytes().iter().map(|&v| v as char).collect();
        let vec_high:Vec<char> = high.as_bytes().iter().map(|&v| v as char).collect();
        if Self::is_a_less_b(&vec_high, &vec_low) {
            return 0;
        }
        if ll == hl {
            Self::less(0, ll/2, ll&1==1, &vec_low, &mut cur, &mut ans);
            let left = ans;
            ans = 0;
            Self::less(0, hl/2, hl&1==1, &vec_high, &mut cur, &mut ans);
            ans -= left;
            if Self::is_strobogrammatic(&vec_low) {
                ans += 1;
            }
            return ans;
        }
        Self::larger(0, ll/2, ll&1==1, &vec_low, &mut cur, &mut ans);
        if hl != 1 {
            Self::less(0, hl/2, hl & 1 == 1, &vec_high, &mut cur, &mut ans);
        }
        ans
    }
    fn larger(i: usize, n: usize, more: bool, low: &Vec<char>, cur: &mut Vec<char>, ans: &mut i32) {
        if i >= n {
            if !more {
                if Self::cmp_larger_equal(cur, None, low) {
                    *ans += 1;
                }
                return;
            }
            for c in vec!['8','1','0'] {
                if Self::cmp_larger_equal(cur, Some(c), low) {
                    *ans += 1;
                } else {
                    return;
                }
            }
            return;
        }
        for (idx,&c) in vec!['0', '1', '6', '8', '9'].iter().enumerate() {
            if i == 0 && c == '0' {
                continue;
            }
            if c < low[i] {
                continue;
            } else if c > low[i] {
                let mut t = (5-idx) * 5usize.pow((n-i-1) as u32);
                if more {
                    t *= 3;
                }
                *ans += t as i32;
                return;
            } else {
                cur.push(c);
                Self::larger(i+1,n,more,low, cur, ans);
                cur.pop();
            }
        }
    }
    fn less(i: usize, n: usize, more: bool, high: &Vec<char>, cur: &mut Vec<char>, ans: &mut i32) {
        if i >= n {
            if !more {
                if Self::less_equal(cur, None, high) {
                    *ans += 1;
                }
                return;
            }
            for c in vec!['0','1','8'] {
                if Self::less_equal(cur, Some(c), high) {
                    *ans += 1;
                } else {
                    return;
                }
            }
            return;
        }
        for (idx,&c) in vec!['9','8','6','1','0'].iter().enumerate() {
            if i == 0 && c == '0' {
                continue;
            }
            if c > high[i] {
                continue;
            } else if c < high[i] {
                let mut t = {
                    let q = {
                        if i == 0 {
                            4-idx
                        } else {
                            5-idx
                        }
                    };
                    q * 5usize.pow((n-i-1) as u32)
                };
                if more {
                    t *= 3;
                }
                *ans += t as i32;
                return;
            } else {
                cur.push(c);
                Self::less(i+1,n,more,high,cur,ans);
                cur.pop();
            }
        }
    }
    fn cmp_larger_equal(low: &Vec<char>, mid: Option<char>, target: &Vec<char>) -> bool {
        let n = low.len();
        if mid.is_none() {
            for (&x,&y) in low.iter().rev().zip(target.iter().skip(n)) {
                let mx = Self::mirror(x);
                if mx > y {
                    return true;
                } else if mx < y {
                    return false;
                }
            }
            return true;
        } else {
            let c = mid.unwrap();
            if c > target[n] {
                return true;
            } else if c < target[n] {
                return false;
            }
            for (&x,&y) in low.iter().rev().zip(target.iter().skip(n+1)) {
                let mx = Self::mirror(x);
                if mx > y {
                    return true;
                } else if mx < y {
                    return false;
                }
            }
            return true;
        }
    }
    fn less_equal(cur: &Vec<char>, mid: Option<char>, target: &Vec<char>) -> bool {
        let n = cur.len();
        if mid.is_none() {
            for (&x,&y) in cur.iter().rev().zip(target.iter().skip(n)) {
                let mx = Self::mirror(x);
                if mx < y {
                    return true;
                } else if mx > y {
                    return false;
                }
            }
            return true;
        }
        let c = mid.unwrap();
        if c < target[n] {
            return true;
        } else if c > target[n] {
            return false;
        } else {
            for (&x,&y) in cur.iter().rev().zip(target.iter().skip(n+1)) {
                let mx = Self::mirror(x);
                if mx < y {
                    return true;
                } else if mx > y {
                    return false;
                }
            }
            return true;
        }
    }
    fn mirror(c: char) -> char {
        match c {
            '0' => '0',
            '1' => '1',
            '6' => '9',
            '8' => '8',
            '9' => '6',
            _ => unreachable!(),
        }
    }
    fn mirror_op(c: char) -> Option<char> {
        match c {
            '0' => Some('0'),
            '1' => Some('1'),
            '6' => Some('9'),
            '8' => Some('8'),
            '9' => Some('6'),
            _ => None,
        }
    }
    fn is_strobogrammatic(arr: &Vec<char>) -> bool {
        let n = arr.len();
        for i in 0..n/2 {
            if let Some(m) = Self::mirror_op(arr[n-i-1]) {
                if arr[i] != m {
                    return false;
                }
            } else {
                return false;
            }
        }
        if n&1==1 {
            if let Some(m) = Self::mirror_op(arr[n/2]) {
                return arr[n/2] == m;
            } else {
                return false;
            }
        }
        true
    }
    fn is_a_less_b(a: &Vec<char>, b: &Vec<char>) -> bool {
        if a.len() < b.len() {
            return true;
        }
        if a.len() > b.len() {
            return false;
        }
        for (&x,&y) in a.iter().zip(b.iter()) {
            if x > y {
                return false;
            } else if x < y {
                return true;
            }
        }
        false
    }
}

struct Solution;

#[cfg(test)]
    mod tests {
        use super::Solution;

    #[test]
    fn test_strobogrammatic_in_range() {
        let test_cases = vec![
            ("986", "986", 1),
            ("500", "100", 0),
            ("182", "985", 8),
            ("181", "985", 9),
            ("50", "99", 3),
            ("0", "7", 2),
            ("0", "1", 2),
            ("0", "0", 1),
            ("96", "985", 12),
            ("97", "985", 11),
            ("8", "985", 16),
            ("0", "985", 18),
            ("0", "999", 19),
            ("50", "100", 3),
            ("0", "9", 3),
        ];
        for (low,high,expect) in test_cases {
            assert_eq!(Solution::strobogrammatic_in_range(low.to_string(), high.to_string()), expect, "low: {}, high: {}", low, high);
        }
    }
}