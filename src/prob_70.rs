impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 3 {
            return n;
        }
        let (mut a, mut b) = (1,2);
        for _ in 3..=n {
            let next = a+b;
            a = b;
            b = next;
        }
        b
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_climb_stairs() {
        let test_cases = vec![
            (2, 2),
            (3, 3),
            (0 ,0),
            (1, 1),
            (4, 5),
            (5, 8),
        ];
        for (n, expect) in test_cases {
            assert_eq!(Solution::climb_stairs(n), expect, "n: {}", n);
        }
    }
}