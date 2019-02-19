impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid.first().unwrap().len();
        let mut queue = vec![];
        let mut all = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    all += 1;
                } else  if grid[i][j] == 2 {
                    queue.push((i,j,1));
                }
            }
        }
        if all == 0 {
            return 0;
        }
        while !queue.is_empty() {
            let (i,j,t) = queue.remove(0);
            if i > 0 && grid[i-1][j] == 1 {
                grid[i-1][j] = 2;
                all -= 1;
                if all == 0 {
                    return t;
                }
                queue.push((i-1,j,t+1));
            }
            if j > 0 && grid[i][j-1] == 1 {
                grid[i][j-1] = 2;
                all -= 1;
                if all == 0 {
                    return t;
                }
                queue.push((i,j-1, t+1))
            }
            if i+1 < n && grid[i+1][j] == 1 {
                grid[i+1][j] = 2;
                all -= 1;
                if all == 0 {
                    return t;
                }
                queue.push((i+1,j,t+1));
            }
            if j+1 < m && grid[i][j+1] == 1 {
                grid[i][j+1] = 2;
                all -= 1;
                if all == 0 {
                    return t;
                }
                queue.push((i,j+1,t+1));
            }
        }
        -1
    }
}

struct Solution;