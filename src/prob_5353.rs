impl Solution {
    pub fn num_times_all_blue(light: Vec<i32>) -> i32 {
        let mut n = light.len();
        if n == 1 {
            return 1;
        }
        n += 1;
        let mut f = vec![0; n];
        let mut ans = 0;
        let mut cur_max = 0;
        for v in light {
            let mut i = v as usize;
            while i < n {
                f[i] += 1;
                i += Self::lowbit(i as i32) as usize;
            }
            cur_max = cur_max.max(v);
            let mut t = 0;
            i = cur_max as usize;
            while i > 0 {
                t += f[i];
                i -= Self::lowbit(i as i32) as usize;
            }
            if t == cur_max {
                ans += 1;
            }
        }
        ans
    }
    fn lowbit(x: i32) -> i32 {
        x & (-x)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_times_all_blue() {
        let test_cases = vec![
            (vec![4,1,2,3], 1),
            (vec![2,1,3,5,4], 3),
            (vec![3,2,4,1,5], 2),
            (vec![2,1,4,3,6,5], 3),
            (vec![1,2,3,4,5,6], 6),
        ];
        for (light, expect) in test_cases {
            assert_eq!(Solution::num_times_all_blue(light.clone()), expect, "light: {:?}", light);
        }
    }
}