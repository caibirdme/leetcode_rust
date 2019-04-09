extern crate rand;
use rand::{Rng, thread_rng};

struct Solution {
    nums: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        Self{nums,}
    }

    fn pick(&self, target: i32) -> i32 {
        let mut rng = thread_rng();
        let res: Vec<usize> = self.nums.iter().enumerate()
            .filter(|(_,&v)| v == target)
            .map(|(i,_)| i)
            .collect();
        let idx = res[rng.gen_range(0, res.len())];
        idx as i32
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_pick() {
        let s = Solution::new(vec![1,2,3,3,3]);
        let n = 1000000;
        let (mut a,mut b,mut c) = (0,0,0);
        for _ in 0..n {
            let idx = s.pick(3);
            if idx == 2 {
                a+=1;
            } else if idx==3 {
                b+=1;
            } else if idx==4 {
                c+=1;
            } else {
                assert_eq!(true, false);
            }
        }
        let pa = (a as f64/n as f64 - 1f64 / 3f64).abs();
        let pb = (b as f64/n as f64 - 1f64 / 3f64).abs();
        let pc = (c as f64/n as f64 - 1f64 / 3f64).abs();
        const one_percent: f64 = 1f64/100f64;
        assert_eq!(true, pa < one_percent, "pa: {}", pa);
        assert_eq!(true, pb < one_percent, "pb: {}", pb);
        assert_eq!(true, pc < one_percent, "pc: {}", pc);
    }
}