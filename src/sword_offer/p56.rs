impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut t = 0;
        for &v in &nums {
            t ^= v;
        }
        let mut i = 0;
        while i < 32 && t & (1<<i) == 0 { i+=1; }

        let mut a = 0;
        for &v in &nums {
            if v & (1<<i) != 0 {
                a ^= v;
            }
        }
        vec![a, t ^ a]
    }
}

struct Solution;