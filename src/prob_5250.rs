impl Solution {
    pub fn is_good_array(nums: Vec<i32>) -> bool {
        let mut d = nums[0];
        let n = nums.len();
        for i in 1..n {
            d = Self::gcd(nums[i], d);
        }
        d == 1
    }
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        if a < b {
            return Self::gcd(b, a);
        }
        let mut t = a % b;
        while t != 0 {
            a = b;
            b = t;
            t = a % b;
        }
        b
    }
}

struct Solution;