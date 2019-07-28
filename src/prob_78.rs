impl Solution {
    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ans = vec![vec![]];
        for &v in nums.iter() {
            ans.push(vec![v]);
        }
        let n = nums.len();
        let mut head = 1;
        while head < ans.len() {
            let cur = ans[head].clone();
            let pos = nums.binary_search(cur.last().unwrap()).unwrap();
            for i in pos+1..n {
                let mut t = cur.clone();
                t.push(nums[i]);
                ans.push(t);
            }
            head += 1;
        }
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_subsets() {
        let test_cases = vec![
            (
                vec![1,2,3],
                vec![
                    vec![],
                    vec![1],
                    vec![2],
                    vec![3],
                    vec![1,2],
                    vec![1,3],
                    vec![2,3],
                    vec![1,2,3],
                ],
            ),
            (
                vec![1,3,5,7],
                vec![
                    vec![],
                    vec![1],
                    vec![3],
                    vec![5],
                    vec![7],
                    vec![1,3],
                    vec![1,5],
                    vec![1,7],
                    vec![3,5],
                    vec![3,7],
                    vec![5,7],
                    vec![1,3,5],
                    vec![1,3,7],
                    vec![1,5,7],
                    vec![3,5,7],
                    vec![1,3,5,7],
                ],
            ),
        ];
        for (nums, expect) in test_cases {
            assert_eq!(Solution::subsets(nums.clone()), expect, "nums: {:?}", nums);
        }
    }
}