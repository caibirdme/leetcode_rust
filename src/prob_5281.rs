impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut r = *nums.iter().max().unwrap();
        let mut l = 1;
        while l < r {
            let m = l + (r-l)/2;
            let t = nums.iter().fold(0, |a,&b| {
                if b % m == 0 {
                    a + b/m
                } else {
                    a+b/m+1
                }
            });
            if t<=threshold {
                r = m;
            } else {
                l = m+1;
            }
        }
        r
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
}