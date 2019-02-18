impl Solution {
    pub fn min_k_bit_flips(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let k = k as usize;
        if n < k {
            return -1;
        }
        if k == 1 {
            return (n as i32) - a.iter().sum::<i32>();
        }
        let mut stop = vec![false; n];
        let mut times = 0;
        let mut res = 0;
        for i in 0..n-k+1{
            if a[i] ^ times == 0 {
                stop[i+k-1] = true;
                times ^= 1;
                res += 1;
            }
            if stop[i] {
                times ^= 1;
            }
        }
        for i in n-k+1 .. n {
            if a[i] ^ times == 0 {
                return -1;
            }
            if stop[i] {
                times ^= 1;
            }
        }
        res
    }
}

struct Solution;


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_k_bit_flips() {
        assert_eq!(
            Solution::min_k_bit_flips(vec![0,0,0,1,0,1,1,0], 3),
            3
        );
        assert_eq!(
            Solution::min_k_bit_flips(vec![0,1,0], 1),
            2
        );
        assert_eq!(
            Solution::min_k_bit_flips(vec![1,1,0], 2),
            -1
        );

    }
}