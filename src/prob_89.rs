impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ans = vec![0];
        if n == 0 {
            return ans;
        }
        for t in 0..n {
            let q = 1<<t;
            let upper = ans.len();
            for m in (0..upper).rev() {
                let v = ans[m];
                ans.push(v | q);
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
    fn test_gray_code() {
        let test_cases = vec![
            (0, vec![0]),
            (1, vec![0,1]),
            (2, vec![0,1,3,2]),
            (3, vec![0,1,3,2,6,7,5,4]),
        ];
        for (n, expect) in test_cases {
            assert_eq!(Solution::gray_code(n), expect, "n: {}", n);
        }
    }
}