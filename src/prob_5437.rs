use std::collections::HashSet;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        if m == 0 {
            return 0;
        }
        if n == 1 && m == 1 {
            return 0;
        }
        let mut dist = vec![vec![std::i32::MAX; m]; n];
        let mut q = vec![(0i32,0i32)];
        let mut head = 0;
        let step = [(0i32,0i32),(0,1),(0,-1),(1,0),(-1,0)];
        dist[0][0] = 0;
        let mut inq = HashSet::new();
        inq.insert((0,0));
        while head < q.len() {
            let (x,y) = q[head];
            head += 1;
            inq.remove(&(x,y));
            for i in 1..=4 {
                let nx = x + step[i].0;
                let ny = y + step[i].1;
                if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
                    continue;
                }
                let ux = nx as usize;
                let uy = ny as usize;
                if grid[x as usize][y as usize] == i as i32 {
                    let t = dist[x as usize][y as usize];
                    if dist[ux][uy] > t {
                        dist[ux][uy] = t;
                        if !inq.contains(&(nx,ny)) {
                            q.push((nx,ny));
                            inq.insert((nx,ny));
                        }
                    }
                } else {
                    let t = dist[x as usize][y as usize] + 1;
                    if dist[ux][uy] > t {
                        dist[ux][uy] = t;
                        if !inq.contains(&(nx,ny)) {
                            inq.insert((nx,ny));
                            q.push((nx,ny));
                        }
                    }
                }
            }
        }
        dist[n-1][m-1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_cost() {
        let test_cases = vec![
            (
                vec![
                    vec![3,4,3],vec![2,2,2],vec![2,1,1],vec![4,3,2],
                    vec![2,1,4],vec![2,4,1],vec![3,3,3],vec![1,4,2],
                    vec![2,2,1],vec![2,1,1],vec![3,3,1],vec![4,1,4],
                    vec![2,1,4],vec![3,2,2],vec![3,3,1],vec![4,4,1],
                    vec![1,2,2],vec![1,1,1],vec![1,3,4],vec![1,2,1],
                    vec![2,2,4],vec![2,1,3],vec![1,2,1],vec![4,3,2],
                    vec![3,3,4],vec![2,2,1],vec![3,4,3],vec![4,2,3],vec![4,4,4],
                ],
                18,
            ),
            (
                vec![
                    vec![1,2],
                    vec![4,3],
                ],
                1,
            ),
            (
                vec![vec![4]],
                0,
            ),
            (   vec![
                    vec![1,1,1,1],
                    vec![2,2,2,2],
                    vec![1,1,1,1],
                    vec![2,2,2,2],
                ],
                3
            ),
            (
                vec![
                    vec![1,1,3],
                    vec![3,2,2],
                    vec![1,1,4],
                ],
                0,
            ),

            (
                vec![
                    vec![2,2,2],
                    vec![2,2,2],
                ],
                3,
            ),

        ];
        for (grid, expect) in test_cases {
            assert_eq!(Solution::min_cost(grid.clone()), expect, "grid: {:?}", grid);
        }
    }
}