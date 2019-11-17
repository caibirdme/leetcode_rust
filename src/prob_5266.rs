use std::collections::HashSet;

impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let mut visited: HashSet<(usize,usize,usize,usize)> = HashSet::new();
        let n = grid.len();
        let m = grid[0].len();
        let mut wpos = (0,0);
        let mut bpos = (0,0);
        let mut tpos = (0,0);
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 'S' {
                    wpos = (i,j);
                } else if grid[i][j] == 'B' {
                    bpos = (i,j);
                } else if grid[i][j] == 'T' {
                    tpos = (i,j);
                }
            }
        }
        let mut q = vec![];
        let state = State::new(wpos.0, wpos.1,bpos.0,bpos.1,0);
        visited.insert(state.spread());
        q.push(state);
        let mut head = 0;
        let steps = [(0,1),(0,-1),(1,0),(-1,0)];
        while head < q.len() {
            let cur = q[head];
            head += 1;
            for i in 0..4 {
                let (dx,dy) = steps[i];
                let (bx, by) = cur.bpos();
                let (nx, ny) = (bx+dx, by+dy);
                if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
                    continue;
                }
                let (ux, uy) = (nx as usize, ny as usize);
                if grid[ux][uy] == '#' {
                    continue;
                }
                let (px, py) = (bx-dx, by-dy);
                if px < 0 || py < 0 || px >= n as i32 || py >= m as i32 {
                    continue;
                }
                let (upx, upy) = (px as usize, py as usize);
                let available = Self::can_go(&cur, &grid);
                if !available[upx][upy] {
                    continue;
                }
                if ux == tpos.0 && uy == tpos.1 {
                    return cur.step+1;
                }
                if visited.contains(&(bx as usize, by as usize, ux, uy)) {
                    continue;
                }
                visited.insert((bx as usize, by as usize, ux, uy));
                q.push(State::new(bx as usize, by as usize, ux, uy, cur.step+1));
            }
        }
        -1
    }
    fn can_go(state: &State, grid: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
        let steps = [(0,1),(0,-1),(1,0),(-1,0)];
        let n = grid.len();
        let m = grid[0].len();
        let mut f = vec![vec![false; m]; n];
        let (ux,uy) = state.worker;
        f[ux][uy] = true;
        let mut q = vec![(ux as i32,uy as i32)];
        let mut head = 0;
        while head < q.len() {
            let (x,y) = q[head];
            head += 1;
            for i in 0..4usize {
                let (dx, dy) = steps[i];
                let (nx, ny) = (x+dx, y+dy);
                if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
                    continue;
                }
                let (ux, uy) = (nx as usize, ny as usize);
                if ux == state.cargo.0 && uy == state.cargo.1 {
                    continue;
                }
                if f[ux][uy] || grid[ux][uy] == '#' {
                    continue;
                }
                f[ux][uy] = true;
                q.push((nx,ny));
            }
        }
        f
    }
}

#[derive(Copy, Clone, Debug)]
struct State {
    worker: (usize,usize),
    cargo: (usize,usize),
    step: i32,
}

impl State {
    fn new(ux: usize, uy: usize, bx: usize, by: usize, step: i32) -> Self {
        Self{
            worker: (ux,uy),
            cargo: (bx, by),
            step,
        }
    }
    fn spread(&self) -> (usize,usize,usize,usize) {
        (self.worker.0, self.worker.1,self.cargo.0,self.cargo.1)
    }
    fn bpos(&self) -> (i32, i32) {
        (self.cargo.0 as i32, self.cargo.1 as i32)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_push_box() {
        let test_cases = vec![
            (vec![
                vec!['#','#','#','#','#','#',],
                vec!['#','T','#','#','#','#',],
                vec!['#','.','.','B','.','#',],
                vec!['#','.','#','#','.','#',],
                vec!['#','.','.','.','S','#',],
                vec!['#','#','#','#','#','#',],
            ], 3),
            (vec![
                vec!['#','#','#','#','#','#',],
                vec!['#','T','#','#','#','#',],
                vec!['#','.','.','B','.','#',],
                vec!['#','#','#','#','.','#',],
                vec!['#','.','.','.','S','#',],
                vec!['#','#','#','#','#','#',],
            ], -1),
            (vec![
                vec!['#','#','#','#','#','#',],
                vec!['#','T','.','.','#','#',],
                vec!['#','.','#','B','.','#',],
                vec!['#','.','.','.','.','#',],
                vec!['#','.','.','.','S','#',],
                vec!['#','#','#','#','#','#',],
            ], 5),
        ];
        for (grid, expect) in test_cases {
            assert_eq!(Solution::min_push_box(grid.clone()), expect, "grid: {:?}", grid);
        }
    }
}