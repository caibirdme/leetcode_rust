impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        if n == 0 {
            return 0;
        }
        let mut ans = 0;
        let (mut l,mut r) = (0 as usize, n-1);
        while l <= r {
            let mid = (l+r) >> 1;
            let t = citations[mid] as usize;
            let m = n-mid;
            if t >= m {
                ans = m;
                if mid == 0 {
                    break;
                }
                r = mid-1;
            } else {
                l= mid+1;
            }
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_h_index() {
        assert_eq!(
            Solution::h_index(vec![1]),
            1
        );
        assert_eq!(
            Solution::h_index(vec![0,1,4,5,6]),
            3
        );
        assert_eq!(
            Solution::h_index(vec![0,1,3,5,6]),
            3
        );

    }
}