impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut acc = vec![1; n];
        acc[0] = nums[0];
        for (i,&j) in nums.iter().enumerate().skip(1) {
            acc[i] = acc[i-1]*j;
        }
        let mut rev_acc = 1;
        let mut res = vec![1; n];
        let mut idx = n-1;
        while idx > 0 {
            res[idx] = acc[idx-1]*rev_acc;
            rev_acc *= nums[idx];
            idx -= 1;
        }
        res[0] = rev_acc;
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_product_except_self() {
        assert_eq!(
            Solution::product_except_self(vec![1,2,3,4]),
            vec![24,12,8,6]
        );
        assert_eq!(
            Solution::product_except_self(vec![1,1,1,1]),
            vec![1,1,1,1]
        );
        assert_eq!(
            Solution::product_except_self(vec![1,1,0,1]),
            vec![0,0,1,0]
        );
        assert_eq!(
            Solution::product_except_self(vec![5,9]),
            vec![9,5]
        );
    }
}
