struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut sum = 0;
        for i in (1..=9).rev().take(k as usize) {
            sum += i;
        }
        if sum < n {
            return vec![];
        }
        if sum == n {
            return vec![(1..=9).rev().take(k as usize).collect()];
        }
        Self::find(k, 1, n)
    }
    fn find(k: i32, start: i32, n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if k == 0 {
            return res;
        }
        if k == 1 {
            if start > n || n > 9 {
                return res;
            }
            return vec![
                vec![n]
            ];
        }
        for i in start..=9 {
            let subsets = Self::find(k-1, i+1, n-i);
            if subsets.len() == 0 {
                continue;
            }
            for set in subsets {
                let mut new_set = vec![i];
                for val in set {
                    new_set.push(val);
                }
                res.push(new_set);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_combination_sum3() {
        assert_eq!(
            Solution::combination_sum3(3, 15),
            vec![
                vec![1,5,9],
                vec![1,6,8],
                vec![2,4,9],
                vec![2,5,8],
                vec![2,6,7],
                vec![3,4,8],
                vec![3,5,7],
                vec![4,5,6]
            ]
        );
        assert_eq!(
            Solution::combination_sum3(1, 9),
            vec![
                vec![9]
            ]
        );

        assert_eq!(
            Solution::combination_sum3(3, 24),
            vec![
                vec![9,8,7]
            ]
        );
        assert_eq!(
            Solution::combination_sum3(3, 7),
            vec![
                vec![1,2,4]
            ]
        );
        assert_eq!(
            Solution::combination_sum3(3, 9),
            vec![
                vec![1,2,6],
                vec![1,3,5],
                vec![2,3,4]
            ]
        );
    }
}