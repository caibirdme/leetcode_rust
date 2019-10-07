impl Solution {
    pub fn multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = a.len();
        if n == 0 {
            return vec![];
        }
        let m = a[0].len();
        let k = b[0].len();
        let mut ans = vec![vec![0; k]; n];
        for i in 0..n {
            for j in 0..k {
                let mut c = 0;
                for t in 0..m {
                    c += a[i][t]*b[t][j];
                }
                ans[i][j] = c;
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
    fn test_multiply() {
        let test_cases = vec![
            (
                vec![
                    vec![1, 0, 0],
                    vec![-1,0, 3],
                ],
                vec![
                    vec![7,0,0],
                    vec![0,0,0],
                    vec![0,0,1],
                ],
                vec![
                    vec![7,0,0],
                    vec![-7,0,3],
                ],
            ),
        ];
        for (a,b,c) in test_cases {
            assert_eq!(Solution::multiply(a.clone(), b.clone()), c, "a: {:?}, b: {:?}",a,b);
        }
    }
}