impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        // 相当于求用 多少位k进制数可以表示0~buckets （k=minutes_to_test/minutes_to_die+1）
        let k = minutes_to_test/minutes_to_die+1;
        (buckets as f64).log(k as f64).ceil() as i32
    }
}

struct Solution;


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_poor_pigs() {
        let test_cases = vec![
            (1000, 15, 60, 5),
            (4, 1, 2, 2),
            (4, 1, 1, 2),
        ];
        for (buckets, minutes_to_die, minutes_to_test, expect) in test_cases {
            assert_eq!(Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test), expect, "buckets {}, minute_to_die {}, minutes_to_test {}", buckets, minutes_to_die, minutes_to_test);
        }
    }
}