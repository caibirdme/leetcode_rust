impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let t = nums.iter().fold(0 as i32, |a,b| a^b );
        let mut q = 1;
        while t&q == 0 {
            q <<= 1;
        }
        let (mut a,mut b) = (0,0);
        for i in nums {
            if i&q == q {
                a ^= i;
            } else {
                b ^= i;
            }
        }
        vec![a,b]
    }
}

struct Solution;

mod tests {
    use super::Solution;
    #[test]
    fn test_single_number() {
        assert_eq!(
            Solution::single_number(vec![1,2,1,3,2,5]),
            vec![3,5]
        );
    }
}