impl Solution {
    pub fn subarrays_div_by_k_dp(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let mut sum = vec![0; n+1];
        for (i,&v) in a.iter().enumerate() {
            sum[i+1] = sum[i]+v;
        }
        let mut f = vec![0; n+1];
        for i in 1..=n {
            for j in (0..i).rev() {
                if (sum[i]-sum[j]) % k == 0 {
                    f[i] = f[j]+1;
                    break;
                }
            }
        }
        f.iter().sum()
    }
    pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let mut sum = vec![0; n+1];
        for (i,&v) in a.iter().enumerate() {
            sum[i+1] = sum[i]+v;
        }
        let mut hash = vec![0; k as usize];
        hash[0] = 1;
        for i in 1..=n {
           hash[((sum[i]%k+k)%k) as usize] += 1;
        }
        let mut count = 0;
        for &v in &hash {
            if v >= 2 {
                count += v*(v-1)/2;
            }
        }
        count
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_subarrays_div_by_k() {
        let test_cases = vec![
            (vec![-1,2,9], 2, 2),
            (vec![4,5,0,-2,-3,1], 5, 7),
        ];
        for (a, k, expect) in test_cases {
            assert_eq!(Solution::subarrays_div_by_k(a.clone(), k), expect, "a: {:?}, k: {}", a,k);
        }
    }
}