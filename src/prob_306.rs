impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let n = num.len();
        let nums = num.as_bytes();
        for i in 1..=n*2/3 {
            for j in 1..=i {
                let (l1,l2) = (j,i-j+1);
                if l2 > 1 && nums[j] == b'0' || l1 > 1 && nums[0] == b'0' {
                    continue;
                }
                let res = n-i-1;
                if l1 > res || l2 > res {
                    continue;
                }
                if let Ok(a) = &num[..l1].parse::<i64>() {
                    if let Ok(b) = &num[j..i+1].parse::<i64>() {
                        if Self::is_additive(*a,*b, &num[i+1..]) {
                            return true;
                        }
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }

            }
        }
        false
    }
    fn is_additive(a: i64, b: i64, num: &str) -> bool {
        let p = a.checked_add(b);
        let c: i64;
        match p {
            None => return false,
            Some(v) => {c=v;},
        }
        let cs: String = c.to_string();
        let n = cs.len();
        if n > num.len() {
            return false;
        }
        if cs != &num[..n] {
            return false;
        }
        if n == num.len() {
            true
        } else {
            Self::is_additive(b, c, &num[n..])
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_is_additive_number() {
        assert_eq!(
            Solution::is_additive_number("211738".to_string()),
            true
        );
        assert_eq!(
            Solution::is_additive_number("011".to_string()),
            true
        );
        assert_eq!(
            Solution::is_additive_number("001".to_string()),
            false
        );
        assert_eq!(
            Solution::is_additive_number("112358".to_string()),
            true
        );
        assert_eq!(
            Solution::is_additive_number("199100199".to_string()),
            true
        );
        assert_eq!(
            Solution::is_additive_number("221474836472147483649".to_string()),
            true
        );
        assert_eq!(
            Solution::is_additive_number("101".to_string()),
            true
        );

        assert_eq!(
            Solution::is_additive_number("0001".to_string()),
            false
        );
        assert_eq!(
            Solution::is_additive_number("221474836472147483648".to_string()),
            false
        );
    }
}