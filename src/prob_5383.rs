const MOD: i64 = 1000000007;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        if n == 1 {
            return 12;
        }
        let mut available = vec![];
        for i in 1..64usize {
            let c1 = i & 3;
            let c2 = (i>>2)&3;
            let c3 = (i>>4)&3;
            if c1 !=0 && c2 != 0 && c3 != 0 && c3 != c2 && c2 != c1 {
                available.push(i);
            }
        }
        assert_eq!(12, available.len());
        let mut count = [0; 64];
        for &v in &available {
            count[v as usize] = 1i64;
        }
        for _ in 1..n {
            let mut new_count = [0; 64];
            for &pre_color in &available {
                let num = count[pre_color];
                if num == 0 {
                    continue;
                }
                let c1 = pre_color & 3;
                let c2 = (pre_color>>2)&3;
                let c3 = (pre_color>>4)&3;
                for &color in &available {
                    let t1 = color & 3;
                    let t2 = (color>>2)&3;
                    let t3 = (color>>4)&3;
                    if t1 != c1 && t2 != c2 && t3 != c3 {
                        new_count[color] += num;
                        new_count[color] %= MOD;
                    }
                }
            }
            count = new_count;
        }
        ((count.iter().sum::<i64>()) % MOD) as i32
    }
    pub fn num_of_ways_mat(n: i32) -> i32 {
        if n == 1 {
            return 12;
        }
        let ((a,b),(c,d)) = Self::mat_pow(((3,2), (2,2)), n as usize-1);
        ((6*(a+b+c+d)) % MOD) as i32
    }
    fn mat_pow(mat: ((i64, i64), (i64,i64)), n: usize) -> ((i64, i64), (i64,i64)) {
        if n == 1 {
            return mat;
        }
        let t = n / 2;
        let q= Self::mat_pow(mat, t);
        let w = Self::mat_mul(q, q);
        if n & 1 == 1 {
            Self::mat_mul(w, mat)
        } else {
            w
        }
    }
    fn mat_mul(x: ((i64, i64), (i64,i64)), y: ((i64, i64), (i64,i64))) -> ((i64, i64), (i64,i64)) {
        let ((x11, x12),(x21,x22)) = x;
        let ((y11,y12), (y21,y22)) = y;
        let z11 = (x11*y11+x12*y21) % MOD;
        let z12 = (x11*y12+x12*y22) % MOD;
        let z21 = (x21*y11+x22*y21) % MOD;
        let z22 = (x21*y21+x22*y22) % MOD;
        ((z11, z12), (z21, z22))
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_of_ways() {
        let test_cases = vec![
            (2, 54),
            (3, 246),
            (7, 106494),
            (5000, 30228214),
        ];
        for (n, expect) in test_cases {
            //assert_eq!(Solution::num_of_ways(n), expect, "n:{}", n);
            assert_eq!(Solution::num_of_ways_mat(n), expect, "mat, n: {}", n);
        }
    }
}