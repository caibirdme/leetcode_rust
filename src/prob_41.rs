impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 1;
        }
        let mut ok = false;
        for &v in &nums {
            if v == 1 {
                ok = true;
                break;
            }
        }
        if !ok {
            return 1;
        }
        if n == 1 && nums[0] == 1 {
            return 2;
        }
        for i in 0..n {
            if nums[i] <=0 || nums[i] > n as i32 {
                nums[i] = 1;
            }
        }
        for i in 0..n {
            let w = nums[i];
            let t = if w > 0 {w} else {-w};
            if t == n as i32 {
                nums[0] = -t;
                continue;
            }
            let v = nums[t as usize];
            if v > 0 {
                nums[t as usize] = -v;
            }
        }
        for i in 1..n {
            if nums[i] > 0 {
                return i as i32;
            }
        }
        if nums[0] > 0 {n as i32} else {n as i32+1}
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_first_missing_positive() {
        let test_cases = vec![
            (vec![1,1], 2),
            (vec![1,2,0], 3),
            (vec![1], 2),
            (vec![3,4,-1,1],2),
            (vec![7,8,9,11,12], 1),
        ];
        for (arr, expect) in test_cases {
            assert_eq!(Solution::first_missing_positive(arr.clone()), expect, "arr: {:?}", arr);
        }
    }
}