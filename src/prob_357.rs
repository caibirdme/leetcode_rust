impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 10;
        }
        if n > 10 {
            return Self::count_numbers_with_unique_digits(10);
        }
        let mut f9 = 9;
        let mut next = 9;
        let mut last = 10;
        for i in 2..=n {
            f9 *= next;
            next-=1;
            last += f9;
        }
        last as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_count_numbers_with_unique_digits() {
        assert_eq!(91, Solution::count_numbers_with_unique_digits(2));
        assert_eq!(739, Solution::count_numbers_with_unique_digits(3));
    }
}