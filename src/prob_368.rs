impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n < 2 {
            return nums
        }
        nums.sort();
        let mut f = vec![0; n];
        let impossible = n+10;
        let mut pre = vec![impossible; n];
        f[0] = 1;
        let mut ans = 1;
        let mut idx = 0;
        for i in 1..n {
            let t = nums[i];
            let upper = {
                match nums.binary_search(&(t/2)) {
                    Ok(pos) => pos,
                    Err(pos) => {
                        if pos == i {
                            pos-1
                        } else {
                            pos
                        }
                    },
                }
            };
            for j in 0..=upper {
                if t % nums[j] == 0 && f[j] > f[i] {
                    f[i] = f[j];
                    pre[i] = j;
                }
            }
            f[i]+=1;
            if f[i] > ans {
                ans = f[i];
                idx = i;
            }
        }
        let mut out = vec![nums[idx]];
        while pre[idx] != impossible {
            out.push(nums[pre[idx]]);
            idx = pre[idx];
        }
        out
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_largest_divisible_subset() {
        let test_cases = vec![
            (vec![1,2,3,4], vec![1,2,4]),
            (vec![1,5], vec![1,5]),
            (vec![2,5], vec![2]),
            (vec![8], vec![8]),
            (vec![2,3,4,5,6,7,8,9,10], vec![2,4,8]),
        ];
        for (nums, expect) in test_cases {
            let mut actual = Solution::largest_divisible_subset(nums.clone());
            actual.sort();
            assert_eq!(expect, actual, "{:?}", nums);
        }
    }
}