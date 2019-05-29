impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut count = 0;
        let mut t = x ^ y;
        while t > 0 {
            if t & 1 == 1 {
                count += 1;
            }
            t >>= 1;
        }
        count
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_hamming_distance() {
        let test_cases = vec![
            (3, 7, 1),
            (0, 7, 3),
            (4, 1, 2),
        ];
        for (x,y,exp) in test_cases {
            assert_eq!(Solution::hamming_distance(x,y), exp, "x {}, y {}",x,y);
        }
    }
}