impl Solution {
    pub fn range_bitwise_and(m: i32, mut n: i32) -> i32 {
        if m == n {
            return m;
        }
        while n > m {
            n = n & (n-1);
        }
        n & m
    }
}

struct Solution;