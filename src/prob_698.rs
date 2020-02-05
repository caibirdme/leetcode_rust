impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, mut k: i32) -> bool {
        if k == 1 {
            return true;
        }
        if nums.len() == 0 {
            return false;
        }
        nums.sort();
        let sum: i32 = nums.iter().sum();
        if sum % k != 0 {
            return false;
        }
        let part = sum / k;
        let mut i = nums.len()-1;
        if nums[i] > part {
            return false;
        }
        while nums[i] == part {
            k -= 1;
            if i == 0 {
                break
            }
            i-=1;
        }
        if i == 0 || k == 1 {
            return true;
        }
        let mut f = vec![None; 1<<(nums.len()+1)];
        Self::dfs(0, k as usize, part, 0, &nums, &mut f)
    }
    fn dfs(group: i32, k: usize, target: i32, state: usize, nums: &Vec<i32>, f: &mut Vec<Option<bool>>) -> bool {
        if k==0 {
            return true;
        }
        if let Some(ok) = f[state] {
            return ok;
        }
        for (i,&v) in nums.iter().rev().enumerate() {
            if state & (1<<i) == 0 {
                let t = v+group;
                    if t == target {
                        if Self::dfs(0, k-1, target, state|(1<<i), nums, f) {
                            f[state] = Some(true);
                            return true;
                        }
                    } else if t<target && Self::dfs(t, k, target, state|(1<<i), nums, f){
                        f[state] = Some(true);
                        return true;
                    }
            }
        }
        f[state] = Some(false);
        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_can_partition_k_subsets() {
        let test_cases = vec![
            (vec![4, 3, 2, 3, 5, 2, 1], 4, true),
        ];
        for (nums, k, ok) in test_cases {
            assert_eq!(Solution::can_partition_k_subsets(nums.clone(), k), ok, "nums: {:?}, k:{}", nums, k);
        }
    }
}