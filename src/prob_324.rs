impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n < 2 {
            return;
        }
        if n == 2 {
            if nums[0]>nums[1] {
                nums.swap(0,1);
            }
            return;
        }
        let v_n = n | 1;
        let mid = Self::nth_element(nums, (n+1)/2);
        let v_idx = |m: usize| (2*m+1)%v_n;
        let mut begin = 0;
        let mut end = n-1;
        let mut i = 0;
        while i <= end {
            let vi = v_idx(i);
            if nums[vi] > mid {
                nums.swap(vi,v_idx(begin));
                i+=1;
                begin+=1;
            } else if nums[vi] < mid {
                nums.swap(vi, v_idx(end));
                end-=1;
            } else {
                i+=1;
            }
        }
    }
    pub fn nth_element(nums: &mut Vec<i32>, n: usize) -> i32 {
        let r = nums.len()-1;
        Self::kth_element(nums, 0, r, n);
        nums[n-1]
    }
    fn kth_element(nums: &mut Vec<i32>, l:usize, r:usize, k:usize) {
        const LEN: usize = 3;
        let n = r-l+1;
        if n<=LEN {
            Self::sort(nums, l, r);
            return;
        }
        let piv = nums[r];
        let mut less = l;
        for i in l..r {
            if nums[i]<piv {
                nums.swap(i,less);
                less+=1;
            }
        }
        nums.swap(less,r);
        let len = less-l+1;
        if len == k {
            return;
        }
        if len > k {
            Self::kth_element(nums, l, less-1, k);
        } else {
            Self::kth_element(nums, less+1, r, k-len);
        }
    }
    fn sort(nums: &mut Vec<i32>,l:usize,r:usize) {
        let n = r-l;
        if n == 0 {
            return;
        }
        for i in 0..n {
            for j in l..r-i {
                if nums[j] > nums[j+1] {
                    nums.swap(j,j+1);
                }
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_nth_element() {
        let test_cases = vec![
            (vec![3,2,1,3], 2),
            (vec![5,3,1,2,6,7,8,5,5], 4),
            (vec![4,3,2,1,0], 2),
            (vec![1,2,3,4,5,6,7,8,9], 6),
            (vec![1,5,4,23,3,2,5,6,7,8,9,0,1,32,3,3,4,2,3,43,5,6,7,8,3,1,0,9,8,6,4,3,5,7,4,5,6,7,3,6,9,2,5], 12),
        ];
        for (mut nums,k) in test_cases {
            let mut nums_1 = nums.clone();
            nums_1.sort();
            assert_eq!(
                Solution::nth_element(&mut nums, k),
                nums_1[k-1]
            );
        }
    }
    #[test]
    fn test_wiggle_sort() {
        let test_cases = vec![
            vec![3,2,1,3],
            vec![1,2,3],
            vec![5,3,1,2,6,7,8,5,5],
            vec![1,2,5,7,3,54,6,2,5,8,1,4,2,7,5,4,3,21,6,4,8],
            vec![4,5,5,6],
        ];
        for mut nums in test_cases {
            Solution::wiggle_sort(&mut nums);
            for i in 0..nums.len()-1 {
                if i&1==0 {
                    assert!(nums[i]<nums[i+1]);
                } else {
                    assert!(nums[i]>nums[i+1]);
                }
            }
        }
    }
}

