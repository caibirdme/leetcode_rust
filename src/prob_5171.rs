impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let ((a,b),(c,d)) = (Self::calc(num+1), Self::calc(num+2));
        if b-a < d-c {
            vec![a,b]
        } else {
            vec![c,d]
        }
    }
    fn calc(num: i32) -> (i32, i32) {
        let sqrt = (num as f64).sqrt() as i32;
        for i in (1..=sqrt).rev() {
            if num % i == 0 {
                return (i, num/i);
            }
        }
        unreachable!()
    }

}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_closest_divisors() {
        let test_cases = vec![
            (8, vec![3,3]),
            (123, vec![5,25]),
        ];
        for (num, expect) in test_cases {
            assert_eq!(Solution::closest_divisors(num), expect, "num:{}",num);
        }
    }
}