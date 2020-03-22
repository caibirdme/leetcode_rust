impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for &v in &nums {
            let mut acc = 0;
            let mut sqrt = (v as f64).sqrt() as i32;
            let mut t = 0;
            if sqrt*sqrt == v {
                acc = 1;
                t += sqrt;
                sqrt -= 1;
            }
            for i in 2..=sqrt {
                if v % i == 0 {
                    acc += 2;
                    if acc > 2 {
                        break;
                    }
                    t += i + v/i;
                }
            }
            if acc == 2 {
                sum += t+1+v;
            }
        }
        sum
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_sum_four_divisors() {
        let test_cases = vec![
            (vec![1,2,3,4,5], 0),
            (vec![21,4,7], 32),
        ];
        for (nums, expect) in test_cases {
            assert_eq!(Solution::sum_four_divisors(nums.clone()),expect, "nums: {:?}", nums);
        }
    }
}