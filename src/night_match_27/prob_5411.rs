impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut f = vec![vec![vec![-1; m]; m]; n];
        f[0][0][m-1] = grid[0][0] + grid[0][m-1];
        let im = m as i32;
        for t in 1..n {
            for i in 0..im {
                for j in 0..im {
                    for di in -1..=1 {
                        for dj in -1..=1 {
                            let (ni, nj) = (i+di, j+dj);
                            if ni < 0 || nj < 0 || ni >= im || nj >= im { continue; }
                            let (ui, uj) = (ni as usize, nj as usize);
                            if f[t-1][ui][uj] < 0 { continue; }
                            let ci = i as usize;
                            let cj = j as usize;
                            if i != j {
                                f[t][ci][cj] = f[t][ci][cj].max(f[t-1][ui][uj] + grid[t][ci] + grid[t][cj]);
                            } else {
                                f[t][ci][cj] = f[t][ci][cj].max(f[t-1][ui][uj] + grid[t][ci]);
                            }
                        }
                    }
                }
            }
        }
        let mut ans = 0;
        for i in 0..m {
            for j in 0..m {
                ans = ans.max(f[n-1][i][j]);
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
    fn test_cherry_pickup() {
        let test_cases = vec![
            (vec![
                vec![1,0,0,3],
                vec![0,0,0,3],
                vec![0,0,3,3],
                vec![9,0,3,3]
            ], 22),
            (vec![
                vec![1,1],
                vec![1,1]
            ], 4),
            (vec![
                vec![3,1,1],
                vec![2,5,1],
                vec![1,5,5],
                vec![2,1,1]
            ], 24),
            (vec![
                vec![1,0,0,0,0,0,1],
                vec![2,0,0,0,0,3,0],
                vec![2,0,9,0,0,0,0],
                vec![0,3,0,5,4,0,0],
                vec![1,0,2,3,0,0,6]
            ], 28),
        ];
        for (grid, expect) in test_cases {
            assert_eq!(Solution::cherry_pickup(grid.clone()), expect, "grid: {:?}", grid);
        }
    }
}