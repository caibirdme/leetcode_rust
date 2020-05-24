impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        if n == 1 && m == 1 {
            return nums1[0]*nums2[0];
        }
        let mut f = vec![vec![std::i32::MIN; m+1]; n+1];
        for i in 0..=m {
            f[0][i] = 0;
        }
        for i in 0..=n {
            f[i][0] = 0;
        }
        let mut ans = std::i32::MIN;
        for i in 1..=n {
            for j in 1..=m {
                f[i][j] = f[i-1][j-1] + nums1[i-1]*nums2[j-1];
                ans = ans.max(f[i][j]);
                f[i][j] = f[i][j].max(f[i-1][j]);
                f[i][j] = f[i][j].max(f[i][j-1]);
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
    fn test_max_dot_product() {
        let test_cases = vec![
            (vec![-3,-8,3,-10,1,3,9],vec![9,2,3,7,-9,1,-8,5,-1,-1],200),
            (vec![2,1,-2,5], vec![3,0,-6], 18),
            (vec![3, -2], vec![2,-6,7], 21),
            (vec![-1,-1],vec![1,1], -1),
        ];
        for (nums1, nums2, expect) in test_cases {
            assert_eq!(Solution::max_dot_product(nums1.clone(), nums2.clone()), expect, "nums1: {:?}, nums2: {:?}", nums1, nums2);
        }
    }

}