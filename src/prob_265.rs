impl Solution {
    pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
        let n = costs.len();
        if n == 0 {
            return 0;
        }
        let k = costs[0].len();
        if k == 0 {
            return 0;
        }
        if k == 1 {
            return costs[0][0];
        }
        let mut min_pos = (0,0);
        let mut other_pos = (0,0);
        let fina = costs.into_iter().fold(vec![0; k], |v, color| {
            let mut next = vec![0; k];
            let mut next_min_pos = (::std::i32::MAX, k+1);
            let mut next_other_pos = (::std::i32::MAX, k+1);
            for i in 0..k {
                if i != min_pos.1 {
                    next[i] = min_pos.0 + color[i];
                } else {
                    next[i] = other_pos.0 + color[i];
                }
                if next[i] < next_min_pos.0 {
                    next_other_pos = next_min_pos;
                    next_min_pos = (next[i], i);
                } else if next[i] < next_other_pos.0 {
                    next_other_pos = (next[i], i);
                }
            }
            min_pos = next_min_pos;
            other_pos = next_other_pos;
            next
        });
        min_pos.0
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_cost_ii() {
        let test_cases = vec![
            (vec![vec![1,5,3],vec![2,9,4]], 5),
            (vec![vec![8]], 8),
            (vec![vec![1,2,3,4,5,6]], 1),
            (vec![vec![1,2,3,4,5,6]], 1),
            (vec![vec![1,5,3],vec![2,1,0], vec![9,9,1]], 3),
        ];
        for (costs, expect) in test_cases {
            assert_eq!(Solution::min_cost_ii(costs.clone()), expect, "costs: {:?}", costs);
        }
    }
}