impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut a = 1;
        let mut b = 0;
        let mut n = n;
        while n > 0 {
            let t = n % 10;
            a *= t;
            b += t;
            n /= 10;
        }
        a - b
    }
}

struct Solution;