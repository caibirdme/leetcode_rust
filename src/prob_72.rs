use std::cmp::min;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (n,m) = (word1.len(), word2.len());
        if n == 0 {
            return m as i32;
        } else if m == 0 {
            return n as i32;
        }
        let mut f = [vec![0; m+1], vec![0; m+1]];
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        let c1 = w1[0];
        for j in 0..m {
            if c1 == w2[j] {
                f[0][j+1] = j;
            } else {
                f[0][j+1] = f[0][j]+1;
            }
        }
        let mut idx = 1;
        for i in 1..n {
            let c1 = w1[i];
            let ni = idx ^ 1;
            for j in 0..m {
                let lj = j+1;
                let c2 = w2[j];
                if c1 == c2 {
                    if j == 0 {
                        f[idx][lj] = i;
                    } else {
                        f[idx][lj] = f[ni][j];
                    }
                } else if j == 0 {
                    f[idx][lj] = f[ni][1]+1;
                } else {
                    let mut t = min(f[ni][lj], f[idx][j]);
                    t = min(t, f[ni][j]);
                    f[idx][lj] = t + 1;
                }
            }
            idx = ni;
        }
        f[idx^1][m] as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_distance() {
        let test_cases = vec![
            ("ab", "a", 1),
            ("tea", "tee", 1),
            ("horse", "ros", 3),
            ("intention", "execution", 5),
        ];
        for (w1,w2,expect) in test_cases {
            let ww1 = w1.to_string();
            let ww2 = w2.to_string();
            assert_eq!(Solution::min_distance(ww1, ww2), expect);
        }
    }
}