impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n < 4 {
            return vec![];
        }
        nums.sort_unstable();
        let mut ans = vec![];
        for i in 0..n-3 {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }
            if nums[i]+nums[i+1]+nums[i+2]+nums[i+3] > target {
                break;
            }

            for j in i+1..n-2 {
                if j > i+1 && nums[j] == nums[j-1]{
                    continue;
                }
                let p = target-nums[i]-nums[j];
                if nums[j+1] >= 0 && nums[j+1] > p {
                    break;
                }
                let mut l = j+1;
                let mut r = n-1;
                while l < r {
                    let t = nums[l] + nums[r];
                    if t == p {
                        ans.push(vec![nums[i],nums[j],nums[l],nums[r]]);
                        while l < r && nums[l] == nums[l+1] { l+=1; }
                        while l < r && nums[r-1] == nums[r] { r-=1; }
                        l+=1;
                        r-=1;
                    } else if t > p {
                        r-=1;
                    } else {
                        l+=1;
                    }
                }
            }
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_four_sum() {
        let test_cases = vec![
            (vec![-3,-2,-1,0,0,1,2,3],0,
                vec![
                    vec![-3,-2,2,3],
                    vec![-3,-1,1,3],
                    vec![-3,0,0,3],
                    vec![-3,0,1,2],
                    vec![-2,-1,0,3],
                    vec![-2,-1,1,2],
                    vec![-2,0,0,2],
                    vec![-1,0,0,1]
                ],
            )
        ];
        for (nums, target, expect) in test_cases {
            assert_eq!(Solution::four_sum(nums.clone(), target), expect, "nums: {:?}, target: {}", nums, target);
        }
    }
}