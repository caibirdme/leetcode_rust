use std::cmp::min;

impl Solution {
    pub fn calculate_minimum_hp(mut dungeon: Vec<Vec<i32>>) -> i32 {
        let n = dungeon.len();
        if n == 0 {
            return 0;
        }
        let m = dungeon[0].len();
        if dungeon[n-1][m-1] >= 0 {
            dungeon[n-1][m-1] = 1;
        } else {
            dungeon[n-1][m-1] = 1 - dungeon[n-1][m-1];
        }
        let last = n-1;
        for j in (0..m-1).rev() {
            if dungeon[last][j] >= dungeon[last][j+1] {
                dungeon[last][j] = 1;
            } else {
                dungeon[last][j] = dungeon[last][j+1] - dungeon[last][j];
            }
        }
        let last = m-1;
        for i in (0..n-1).rev() {
            if dungeon[i][last] >= dungeon[i+1][last] {
                dungeon[i][last] = 1;
            } else {
                dungeon[i][last] = dungeon[i+1][last] - dungeon[i][last];
            }
        }
        for i in (0..n-1).rev() {
            for j in (0..m-1).rev() {
                let m = min(dungeon[i+1][j], dungeon[i][j+1]);
                if dungeon[i][j] >= m {
                    dungeon[i][j] = 1;
                } else {
                    dungeon[i][j] = m - dungeon[i][j];
                }
            }
        }
        dungeon[0][0]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_calculate_minimum_hp() {
        let test_cases = vec![
            (vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5],
            ], 7),
            (vec![
                vec![0,0,0],
                vec![1,1,-1],
            ], 1)
        ];
        for (dungeon, expect) in test_cases {
            assert_eq!(Solution::calculate_minimum_hp(dungeon.clone()), expect, "dungeon: {:?}", dungeon);
        }
    }
}