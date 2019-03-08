impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let (n1,n2) = (nums1.len(),nums2.len());
        if n1 == 0 || n2 == 0 {
            return vec![];
        }
        if n1 > n2 {
            return Self::intersect(nums2, nums1);
        }
        let mut used = vec![false; n1];
        nums1.sort();
        let mut ans = vec![];
        for num  in nums2 {
            let (mut l, mut r) = (0, n1-1);
            let mut idx = None;
            while l<=r {
                let mid = (l+r)/2;
                if nums1[mid] == num {
                    if used[mid] {
                        if mid == 0 {
                            break;
                        }
                        r = mid - 1;
                    } else {
                        idx = Some(mid);
                        l = mid+1;
                    }
                } else if nums1[mid] > num {
                    if mid == 0 {
                        break;
                    }
                    r = mid -1;
                } else {
                    l = mid+1;
                }
            }
            if let Some(idx) = idx {
                ans.push(num);
                used[idx] = true;
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
    fn test_intersect() {
        let test_cases = vec![
            (vec![1,2,2,1], vec![2,2], vec![2,2]),
            (vec![4,9,5], vec![9,4,9,8,4], vec![9,4]),
            (vec![2,3,1,4,5,1,6,1,2], vec![1,1,2,2,3,8,10], vec![1,1,2,2,3]),
            (vec![1,1,1,2,3,4,5], vec![1,], vec![1]),
            (vec![1,1,1,2,3,4,5], vec![6,], vec![]),

        ];
        for (nums1, nums2, mut expect) in test_cases {
            let mut actual = Solution::intersect(nums1, nums2);
            actual.sort();
            expect.sort();
            assert_eq!(actual, expect);
        }
    }
}