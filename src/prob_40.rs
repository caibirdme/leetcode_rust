impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = candidates.len();
        if n == 0 {
            return vec![];
        }
        candidates.sort();
        let mut cur = vec![];
        let mut used = vec![false; n];
        let mut res = vec![];
        Self::dfs(0, &candidates, target, &mut cur, &mut used, &mut res);
        res
    }

    fn dfs(pos: usize, candidates: &Vec<i32>, rest: i32, cur: &mut Vec<i32>, used: &mut Vec<bool>, res : &mut Vec<Vec<i32>>) {
        if rest == 0 {
            res.push(cur.clone());
            return;
        }
        if pos >= candidates.len() {
            return;
        }

        for i in pos..candidates.len() {
            let v = candidates[i];
            if v > rest {
                break;
            }
            if i>0 && v == candidates[i-1] && !used[i-1] {
                continue;
            }
            used[i] = true;
            cur.push(v);
            Self::dfs(i+1, candidates, rest-v, cur, used, res);
            used[i] = false;
            cur.pop();
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_combination_sum2() {
        let test_cases = vec![
            (vec![10,1,2,7,6,1,5], 8, vec![
            vec![1,1,6],vec![1,2,5], vec![1,7], vec![2,6],
            ]),
            (vec![2,5,2,1,2], 5, vec![
            vec![1,2,2], vec![5],
            ]),
        ];
        for (nums, target, expect) in test_cases {
            assert_eq!(Solution::combination_sum2(nums.clone(), target), expect, "nums: {:?}, target: {}", nums, target);
        }
    }
}