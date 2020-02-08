use std::cmp::min;
impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let s = (minutes as f64) *6 as f64;
        let m =(hour*30) as f64+minutes as f64/(2 as f64);
        let delta = (s-m).abs();
        delta.min(360f64-delta)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_angle_clock() {
        let test_cases = vec![
            (3, 30, 75.0),
            (12, 30, 165.0),
            (4, 50, 155.0),
            (3, 15, 7.5),
            (12, 0, 0.0),
        ];
        for (h,m,expect) in test_cases {
            let t = Solution::angle_clock(h,m);
            assert_eq!(t-expect < 1e-5, true, "h:{}. m:{}, result:{}", h, m, t);
        }
    }
}