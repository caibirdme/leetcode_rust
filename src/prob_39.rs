impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if candidates.is_empty() {
            return vec![];
        }
        candidates.sort();
        let mut current = vec![];
        let mut ans = vec![];
        Self::dfs(0, &candidates, target as i64, &mut current, &mut ans);
        ans
    }
    fn dfs(sum: i64, candidates: &[i32], target: i64, current: &mut Vec<(usize, i32)>, ans: &mut Vec<Vec<i32>>) {
        if sum == target {
            let mut v = vec![];
            for (n, val) in current {
                v.append(&mut vec![*val; *n]);
            }
            ans.push(v);
            return;
        }
        if candidates.len() == 0 {
            return;
        }
        let mut i = 0;
        loop {
            let now = sum + i * candidates[0] as i64;
            if now > target {
                break;
            }
            current.push((i as usize, candidates[0]));
            Self::dfs(now, &candidates[1..], target, current, ans);
            current.pop();
            i+=1;
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_combination_sum() {
        let test_cases = vec![
            (vec![2,3,6,7], 7, vec![
            vec![7], vec![2,2,3],
            ]),
            (vec![2,3,5], 8, vec![
                vec![2,2,2,2], vec![2,3,3], vec![3,5]
            ]),
        ];
        for (candicates, target, expect) in test_cases {
            assert_eq!(Solution::combination_sum(candicates.clone(), target), expect, "candidate: {:?}, target: {}", candicates, target);
        }
    }
}