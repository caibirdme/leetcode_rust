use std::cmp::Ordering;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let n = grid[0].len();
        let mut ans = 0;
        for arr in grid.into_iter() {
            match arr.binary_search_by(|&probe| {
                if probe <= -1 {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }) {
                Err(v) => {
                    ans += n-v
                },
                _ => unreachable!(),
            }
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_negatives() {
        let test_cases = vec![
            (
                vec![
                    vec![4,3,2,-1],
                    vec![3,2,1,-1],
                    vec![1,1,-1,-2],
                    vec![-1,-1,-2,-3]
                ],
                8,
            ),
        ];
        for (grid, expect) in test_cases {
            assert_eq!(Solution::count_negatives(grid.clone()), expect, "grid:{:?}", grid);
        }
    }
}