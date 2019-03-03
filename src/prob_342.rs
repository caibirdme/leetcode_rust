impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        let max = i32::max_value() >> 2;
        let mut t = 1;
        while t<max && t<num{
            t<<=2;
        }
        t == num
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_is_power_of_four() {
        let test_cases = vec![
            (16, true),
            (5,false),
            (64, true),
            (256, true),
            (32, false),
        ];
        for (num, ok) in test_cases {
            assert_eq!(
                Solution::is_power_of_four(num),
                ok,
                "{}", num
            );
        }
    }
}