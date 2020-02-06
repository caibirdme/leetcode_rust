impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let len = strs.len();
        if len == 0 {
            return 0;
        }
        let mut f = vec![vec![0; m as usize+1]; n as usize+1];
        for i in 0..len {
            let mut t0 = 0;
            let mut t1 = 0;
            strs[i].as_bytes().iter().for_each(|&c| {
                if c == b'0' {
                    t0+=1;
                } else {
                    t1+=1;
                }
            });
            for j in (t1..=n as usize).rev() {
                for k in (t0..=m as usize).rev() {
                    f[j][k] = f[j][k].max(f[j-t1][k-t0]+1);
                }
            }
        }
        f[n as usize][m as usize]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_max_form() {
        let test_cases = vec![
            (vec!["10", "0001", "111001", "1", "0"], 5, 3, 4),
            (vec!["10", "0", "1"], 1, 1, 2),
        ];
        for (strs, m, n, expect) in test_cases {
            assert_eq!(Solution::find_max_form(strs.iter().map(|v| v.to_string()).collect(), m, n), expect, "s:{:?}, m:{}, n:{}", strs, m,n);
        }
    }
}