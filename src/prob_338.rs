impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        if num == 0 {
            return vec![0];
        }
        let mut f = vec![0; num as usize + 1];
        let mut p = 0;
        let mut pv = f[0]; // reduce memory access
        for i in 1..=num as usize{
            if (i>>1) > p {
                p+=1;
                pv = f[p];
            }
            f[i] = pv + (i as i32 & 1);
        }
        f
    }
}

struct Solution;