impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        if m == 0 {
            return 0;
        }
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 {
                    if !Self::bfs(&mut grid, i as i32,j as i32) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    fn bfs(g: &mut Vec<Vec<i32>>, xx: i32, yy: i32) -> bool {
        let n = g.len() as i32;
        let m = g[0].len() as i32;
        let mut q = vec![];
        let mut head = 0;
        q.push((xx,yy));
        g[xx as usize][yy as usize] = 1;
        let steps = [(0,1),(1,0),(0,-1),(-1,0)];
        let mut touch = {
            if xx == 0 || yy == 0 || xx == n-1 || yy == m-1 {
                true
            } else {
                false
            }
        };
        while head < q.len() {
            let (x,y) = q[head];
            head += 1;
            for (dx,dy) in steps.iter() {
                let nx = x+*dx;
                let ny = y+*dy;
                if nx < 0 || ny < 0 || nx >= n || ny >= m {
                    continue;
                }
                let ux = nx as usize;
                let uy = ny as usize;
                if g[ux][uy] == 1 {
                    continue;
                } else {
                    g[ux][uy] = 1;
                }
                if !touch && (nx == 0 || nx == n-1 || ny == 0 || ny == m-1) {
                    touch = true;
                }
                q.push((nx,ny));
            }
        }
        touch
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_closed_island() {
        let test_cases = vec![
            (vec![
                vec![0,0,1,1,0,1,0,0,1,0],
                vec![1,1,0,1,1,0,1,1,1,0],
                vec![1,0,1,1,1,0,0,1,1,0],
                vec![0,1,1,0,0,0,0,1,0,1],
                vec![0,0,0,0,0,0,1,1,1,0],
                vec![0,1,0,1,0,1,0,1,1,1],
                vec![1,0,1,0,1,1,0,0,0,1],
                vec![1,1,1,1,1,1,0,0,0,0],
                vec![1,1,1,0,0,1,0,1,0,1],
                vec![1,1,1,0,1,1,0,1,1,0],
            ], 5),
            (vec![vec![1,1,1,1,1,1,1,0],vec![1,0,0,0,0,1,1,0],vec![1,0,1,0,1,1,1,0],vec![1,0,0,0,0,1,0,1],vec![1,1,1,1,1,1,1,0]], 2),
            (vec![vec![0,0,1,0,0],vec![0,1,0,1,0],vec![0,1,1,1,0]], 1),
        ];
        for (mut grid, expect) in test_cases {
            assert_eq!(Solution::closed_island(grid.clone()), expect, "grid: {:?}", grid);
        }
    }
}