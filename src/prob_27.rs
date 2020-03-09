impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut j = n-1;
        if j == 0 {
            if nums[0] == val {
                return 0;
            }
            return 1;
        }
        let mut i = 0;
        while i <= j {
            if nums[i] == val {
                nums[i] = nums[j];
                if j == 0 {
                    break;
                }
                j-=1;
                continue;
            } else {
                i+=1;
            }
        }
        i as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_remove_element() {
        let test_cases = vec![
            (vec![3,2,2,3], 3, 2),
        ];
        for (nums, val, expect) in test_cases {
            assert_eq!(Solution::remove_element(&mut nums.clone(), val), expect, "nums: {:?}, val: {}", nums, val);
        }
    }
}