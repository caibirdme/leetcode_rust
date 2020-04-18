macro_rules! val {
    ($e: expr) => {
        ($e-b'0') as i64
    };
}

impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let n = s.len();
        let bs = s.as_bytes();
        let mut f = vec![0i64; n];
        let k = k as i64;
        if val!(bs[0]) > k {
            return 0;
        }
        f[0] = 1;
        for i in 1..n {
            let mut t = val!(bs[i]);
            let mut q = 1;
            if t > k {
                return 0;
            }
            if f[i-1] > 0 && t != 0{
                f[i] += f[i-1];
            }
            for j in (0..i).rev() {
                q *= 10;
                if q > k {
                    break;
                }
                let w = val!(bs[j]);
                if w == 0 {
                    continue;
                }
                t += w*q;
                if t > k {
                    break;
                }
                if j > 0 {
                    f[i] += f[j-1];
                } else if j == 0 {
                    f[i] += 1;
                }
            }
            f[i] %= MOD;
        }
        f[n-1] as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_number_of_arrays() {
        let test_cases = vec![
            ("1000", 10000, 1),
            ("1000", 10, 0),
            ("1317", 2000, 8),
            ("2020", 30, 1),
            ("1234567890", 90, 34),
        ];
        for (s, k, expect) in test_cases {
            assert_eq!(Solution::number_of_arrays(s.to_string(), k), expect, "s: {}, k: {}",s,k);
        }
    }
}