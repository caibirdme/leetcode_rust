impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        n&(n-1) == 0
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_is_power_of_two() {
        assert_eq!(
            Solution::is_power_of_two(1),
            true
        );
        assert_eq!(
            Solution::is_power_of_two(16),
           true
        );
        assert_eq!(
            Solution::is_power_of_two(218),
            false
        );
        assert_eq!(
            Solution::is_power_of_two(1024),
            true
        );
    }
}