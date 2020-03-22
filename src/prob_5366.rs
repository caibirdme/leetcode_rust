impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        let m = grid[0].len();
        if grid[0][0] == 5 {
            return false;
        }
        let mut visited = vec![vec![false; m]; n];
        visited[0][0] = true;
        let go = vec![
            vec![],
            vec![([0,1], vec![1,3,5]), ([0,-1], vec![1,4,6])],
            vec![([1,0], vec![2,5,6]), ([-1,0], vec![2,3,4])],
            vec![([0,-1], vec![1,4,6]), ([1,0], vec![2,5,6])],
            vec![([0,1], vec![1,3,5]), ([1,0], vec![2,5,6])],
            vec![([-1,0], vec![2,3,4]), ([0,-1], vec![1,4,6])],
            vec![([-1,0], vec![2,3,4]), ([0,1], vec![1,3,5])],
        ];
        Self::dfs(&grid, 0,0,&mut visited, &go)
    }
    fn dfs(grid: &Vec<Vec<i32>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>, go: &Vec<Vec<([i32; 2], Vec<i32>)>>) -> bool {
        if x == grid.len()-1 && y == grid[0].len()-1 {
            return true;
        }
        let t = grid[x][y] as usize;
        let px = x as i32;
        let py = y as i32;
        for step in &go[t] {
            let (dx,dy) = (step.0[0], step.0[1]);
            let nx = px+dx;
            let ny = py+dy;
            if nx < 0 || ny < 0 || nx >= grid.len() as i32 || ny >= grid[0].len() as i32 {
                continue;
            }
            let ux = nx as usize;
            let uy = ny as usize;
            if visited[ux][uy] {continue;}
            let q = grid[ux][uy];
            let mut ok = false;
            for &x in &step.1 {
                if x == q {
                    ok = true;
                    break;
                }
            }
            if ok {
                visited[ux][uy] = true;
                if Self::dfs(grid, ux, uy, visited, go) {
                    return true;
                }
                visited[ux][uy] = false;
            }
        }
        return false;
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_has_valid_path() {
        let test_cases = vec![
            (vec![
                vec![2,4,3],vec![6,5,2],
            ], true),
            (vec![vec![1,1,1,1,1,3]], true),
            (vec![
                vec![1,2,1],vec![1,2,1],
            ], false),
            (vec![vec![1,1,2]], false),
        ];
        for (grid, ok) in test_cases {
            assert_eq!(Solution::has_valid_path(grid.clone()), ok, "grid: {:?}", grid);
        }
    }
}