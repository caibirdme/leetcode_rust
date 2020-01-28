use std::cmp::max;

impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let n = a.len();
        let m = b.len();
        if n < m {
            return Self::find_length(b, a);
        }
        if m == 0 {
            return 0;
        }
        let mut f = vec![0; m+1];
        let mut ans = 0;
        for &xa in a.iter() {
            for j in (0..m).rev() {
                let xb = b[j];
                let nj = j+1;
                if xa == xb {
                    f[nj] = f[j]+1;
                    ans = max(ans, f[nj]);
                } else {
                    f[nj] = 0;
                }
            }
        }
        ans
    }
    pub fn find_length_subseq(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let n = a.len();
        let m = b.len();
        if n < m {
            return Self::find_length(b, a);
        }
        if m == 0 {
            return 0;
        }
        let mut f = [vec![0; m+1], vec![0; m+1]];
        let mut idx: usize = 0;
        for &xa in a.iter() {
            let np = idx ^ 1;
            for (j, &xb) in b.iter().enumerate() {
                let lj = j+1;
                if xa == xb {
                    f[idx][lj] = f[np][j]+1;
                } else {
                    f[idx][lj] = max(f[np][lj], f[idx][j]);
                }
            }
            idx = np;
        }
        f[idx^1][m]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_length() {
        let test_cases = vec![
            (vec![0,1,1,1,1], vec![1,0,1,0,1], 2),
            (vec![0,1,2,3,1,2,1], vec![1,2,3,1,1,], 4),
            (vec![1,2,3,2,1], vec![3,2,1,4,7], 3),
        ];
        for (a,b,expect) in test_cases {
            assert_eq!(Solution::find_length(a,b), expect);
        }
    }
}