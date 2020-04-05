impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut arr = vec![];
        for &c in s.as_bytes().iter().rev() {
            arr.push(c-b'0');
        }
        let mut idx = 0;
        let mut ans = 0;
        while idx < arr.len()-1 {
            ans += 1;
            if arr[idx] == 0 {
                idx += 1;
            } else {
                arr[idx] = 0;
                let mut flag = false;
                for j in idx+1..arr.len() {
                    arr[j] += 1;
                    if arr[j] == 2 {
                        flag = true;
                        arr[j] = 0;
                    } else {
                        flag = false;
                        break;
                    }
                }
                if flag {
                    arr.push(1);
                }
            }
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_steps() {
        let test_cases = vec![
            ("1111", 5),
            ("1101", 6),
            ("10", 1),
            ("1", 0),
        ];
        for (s, expect) in test_cases {
            assert_eq!(Solution::num_steps(s.to_string()), expect, "s:{}", s);
        }
    }
}