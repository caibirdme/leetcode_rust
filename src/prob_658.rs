impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let n = arr.len();
        if n < 2 {
            return arr;
        }
        let uk = k as usize;
        if x <= arr[0] {
            return Vec::from(&arr[0..uk]);
        }
        if x >= arr[n-1] {
            return Vec::from(&arr[n-uk..]);
        }
        let mut l = 0;
        let mut r = n-uk;
        while l < r {
            let m = (l+r)/2;
            if x-arr[m] <= arr[m+uk]-x {
                r = m;
            } else {
                l = m+1;
            }
        }
        Vec::from(&arr[l..l+uk])
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_closest_elements() {
        let test_cases = vec![
            (vec![1,3,5,9,14], 2, 4, vec![3,5]),
            (vec![1,2,3,4,5],4,3, vec![1,2,3,4]),
            (vec![1,2,3,4,5], 4, -1, vec![1,2,3,4]),
        ];
        for (nums, k, x, expect) in test_cases {
            assert_eq!(Solution::find_closest_elements(nums.clone(), k, x), expect, "nums: {:?}, k: {}, x: {}", nums, k, x);
        }
    }
}