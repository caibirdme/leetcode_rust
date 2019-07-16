impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len()-1;
        if n == 0 {
            return;
        }
        let mut i = n-1;
        while i > 0 && nums[i] >= nums[i+1] {
            i-=1;
        }
        // 全是倒序
        if i == 0 && nums[0] >= nums[1] {
            nums.sort();
            return;
        }
        let piv = nums[i];
        let mut j = n;
        while j > i && nums[j] <= piv {
            j-=1;
        }
        let mut ans = nums[..i].to_vec();
        ans.push(nums[j]);
        let mut k = n;
        while k > j {
            ans.push(nums[k]);
            k-=1;
        }
        ans.push(piv);
        k = j-1;
        while k > i {
            ans.push(nums[k]);
            k-=1;
        }
        *nums = ans;
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_next_permutation() {
        let test_cases = vec![
            (vec![1,2,4,3], vec![1,3,2,4]),
            (vec![1,2,3], vec![1,3,2]),
            (vec![1,], vec![1,]),
            (vec![1,2], vec![2,1]),
            (vec![1,1,5], vec![1,5,1]),
            (vec![3,2,1], vec![1,2,3]),
            (vec![2,6,9,8,4,2], vec![2,8,2,4,6,9]),
        ];
        for (mut nums, expect) in test_cases {
            let cache = nums.to_vec();
            Solution::next_permutation(&mut nums);
            assert_eq!(nums, expect, "nums: {:?}", cache);
        }
    }
}