impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut v = 0;
        for i in 0..32 {
            let q = 1<<i;
            let count = nums.iter().filter(|&&v| v&q != 0).count();
            if count%3 != 0 {
                v |= q;
            }
        }
        v
    }
}

struct Solution;