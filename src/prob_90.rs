impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![];
        }
        nums.sort();
        let mut ans = vec![vec![]];
        let n = nums.len();
        ans.push(vec![(nums[0], 0)]);
        let mut pre = nums[0];
        for i in 1..n {
            if nums[i] != pre {
                pre = nums[i];
                ans.push(vec![(pre, i)]);
            }
        }
        let mut i = 1;
        while i < ans.len() {
            let cur = ans[i].clone();
            let pos = cur.last().unwrap().1;
            if pos >= n-1 {
                i+=1;
                continue;
            }
            let mut pre = nums[pos+1];
            let mut next = cur.clone();
            next.push((pre, pos+1));
            ans.push(next);
            for j in pos+2..n {
                if nums[j] != pre {
                    pre = nums[j];
                    next = cur.clone();
                    next.push((pre, j));
                    ans.push(next);
                }
            }
            i+=1;
        }
        ans.into_iter().map(|v| v.into_iter().map(|t| t.0).collect()).collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_subsets_with_dup() {
        let test_cases = vec![
            (vec![1,2,3], vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![1,2],
                vec![1,3],
                vec![2,3],
                vec![1,2,3],
            ]),
            (vec![1,2], vec![
                vec![],
                vec![1],
                vec![2],
                vec![1,2],
            ]),
            (vec![1,1,1], vec![
                vec![],
                vec![1],
                vec![1,1],
                vec![1,1,1],
            ]),
            (vec![1,2,2], vec![
                vec![],
                vec![1],
                vec![2],
                vec![1,2],
                vec![2,2],
                vec![1,2,2],
            ]),
        ];
        for (mut nums, expect) in test_cases {
            let cache = nums.clone();
            assert_eq!(Solution::subsets_with_dup(nums), expect, "nums: {:?}", cache);
        }
    }
}