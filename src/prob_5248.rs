impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let uk = k as usize;
        let n = nums.len();
        let mut arr = vec![];
        nums.into_iter().enumerate().for_each(|(idx,v)| {
            if v & 1 == 1 {
                arr.push(idx);
            }
        });
        if arr.len() < uk {
            return 0;
        }
        let mut ans = 0;
        for i in uk-1..arr.len() {
            let l = {
                if i+1-uk == 0 {
                    arr[0]
                } else {
                    arr[i+1-uk]-arr[i-uk]-1
                }
            };
            let r = {
                if i+1 == arr.len() {
                    n-arr[i]-1
                } else {
                    arr[i+1]-arr[i]-1
                }
            };
            ans += (r+1)*l+r+1;
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_number_of_subarrays() {
        let test_cases = vec![
            (vec![1,1,2,1,1], 3, 2),
            (vec![2,4,6], 1, 0),
            (vec![2,2,2,1,2,2,1,2,2,2], 2, 16),
        ];
        for (nums, k, expect) in test_cases {
            assert_eq!(Solution::number_of_subarrays(nums.clone(), k), expect, "nums: {:?}, k: {}", nums, k);
        }
    }
}