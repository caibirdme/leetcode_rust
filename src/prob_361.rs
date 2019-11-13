use std::cmp::max;

impl Solution {
    pub fn max_killed_enemies(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        if m == 0 {
            return 0;
        }
        let mut l2r = vec![vec![0; m]; n];
        let mut u2d = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                match grid[i][j] {
                    'E' => {
                        if j > 0 {
                            l2r[i][j] = l2r[i][j-1]+1;
                        } else {
                            l2r[i][j] = 1;
                        }
                        if i > 0 {
                            u2d[i][j] = u2d[i-1][j]+1;
                        } else {
                            u2d[i][j] = 1;
                        }
                    },
                    'W' => {
                        l2r[i][j] = 0;
                        u2d[i][j] = 0;
                    },
                    '0' => {
                        if j > 0 {
                            l2r[i][j] = l2r[i][j-1];
                        }
                        if i > 0 {
                            u2d[i][j] = u2d[i-1][j];
                        }
                    },
                    _ => unreachable!(),
                }
            }
        }
        let mut r2l = vec![vec![0; m]; n];
        let mut d2u = vec![vec![0; m]; n];
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                match grid[i][j] {
                    'E' => {
                        if i+1 < n {
                            d2u[i][j] = d2u[i+1][j]+1;
                        } else {
                            d2u[i][j] = 1;
                        }
                        if j+1 < m {
                            r2l[i][j] = r2l[i][j+1]+1;
                        } else {
                            r2l[i][j] = 1;
                        }

                    },
                    'W' => {
                        d2u[i][j] = 0;
                        r2l[i][j] = 0;
                    },
                    '0' => {
                        if i+1 < n {
                            d2u[i][j] = d2u[i+1][j];
                        }
                        if j+1 < m {
                            r2l[i][j] = r2l[i][j+1];
                        }
                    },
                    _ => unreachable!(),
                }
            }
        }
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '0' {
                    ans = max(ans, l2r[i][j]+r2l[i][j]+u2d[i][j]+d2u[i][j]);
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
    fn test_max_killed_enemies() {
        let test_cases = vec![
            (vec![
                vec!['0','E'],
            ], 1),
            (vec![
                vec!['0','E','0','0'],
                vec!['E','0','W','E'],
                vec!['0','E','0','0'],
            ], 3),
        ];
        for (grid, expect) in test_cases {
            assert_eq!(Solution::max_killed_enemies(grid.clone()), expect, "grid: {:?}", grid);
        }
    }
}