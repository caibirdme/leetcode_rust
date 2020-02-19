impl Solution {
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = nums.len();
        Self::merge_sort(&mut nums, 0, n-1, &mut ans);
        ans as i32
    }
    fn merge_sort(nums: &mut Vec<i32>, l: usize, r: usize, ans: &mut usize) {
        if l >= r {
            return;
        }
        let mid = (l+r)/2;
        Self::merge_sort(nums, l, mid, ans);
        Self::merge_sort(nums, mid+1, r, ans);
        let mut i = l;
        let mut j = mid+1;
        let mut tmp = Vec::with_capacity(r-l+1);
        while i <= mid && j <= r {
            if nums[j] < nums[i] {
                *ans += mid-i+1;
                tmp.push(nums[j]);
                j+=1;
            } else {
                tmp.push(nums[i]);
                i+=1;
            }
        }
        while i <= mid {tmp.push(nums[i]); i+=1;}
        while j <= r {tmp.push(nums[j]); j+=1;}
        for i in 0..r-l+1 {
            nums[i+l] = tmp[i];
        }
    }
}

struct Solution;