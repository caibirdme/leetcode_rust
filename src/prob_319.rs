impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 1;
        }
        (n as f64).sqrt().floor() as i32
    }
}

struct Solution;