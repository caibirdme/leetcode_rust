impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut l, mut r) = (1, (n-1) as i32);
        let mut ans = 1;
        while l <= r {
            let mid = (l+r) >> 1;
            let mut count = 0;
            for &i in nums.iter() {
                if i <= mid {
                    count += 1;
                }
            }
            if count > mid {
                r = mid-1;
                ans = mid;
            } else {
                l = mid+1;
            }
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_duplicate() {
        assert_eq!(
            Solution::find_duplicate(vec![3,1,3,4,2]),
            3,
        );
        assert_eq!(
            Solution::find_duplicate(vec![1,3,4,2,2]),
            2,
        );
    }
}