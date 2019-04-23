impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        if n == 2 {
            if nums[0] > nums[1] {
                return nums[0];
            }
            return nums[1];
        }
        let mut a = nums[0];
        let mut i = 1;
        while i < n && nums[i] == a {
            i+=1;
        }
        if i == n {
            return a;
        }
        let mut b = nums[i];
        if a < b {
            let t = a;
            a = b;
            b = t;
        }
        i+=1;
        while i < n && (nums[i] == a || nums[i] == b) {
            i+=1;
        }
        if i == n {
            return a;
        }
        let mut c = nums[i];
        if c > a {
            let t = c;
            c = b;
            b = a;
            a = t;
        } else if c > b {
            let t = c;
            c = b;
            b = t;
        }
        i+=1;
        while i < n {
            let v = nums[i];
            if v == a || v == b || v == c {
                i+=1;
                continue;
            }
            if v > a {
                c = b;
                b = a;
                a = v;
            } else if v > b {
                c = b;
                b = v;
            } else if v > c {
                c = v;
            }
            i+=1;
        }
        return c;
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_third_max() {
        let test_cases = vec![
            (vec![1,2,2,5,3,5], 2),
        ];
        for (nums, expect) in test_cases {
            assert_eq!(expect, Solution::third_max(nums.clone()), "nums: {:?}", nums);
        }
    }
}