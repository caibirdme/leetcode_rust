use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Elem(i32, i32, i32);

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}
impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let n = height_map.len();
        if n < 2 {
            return 0;
        }
        let m = height_map[0].len();
        let mut visited = vec![vec![false; m];n];
        let mut hp = BinaryHeap::new();
        for i in 0..n {
            hp.push(Elem(height_map[i][0], i as i32, 0));
            hp.push(Elem(height_map[i][m-1], i as i32, m as i32-1));
            visited[i][0] = true;
            visited[i][m-1] = true;
        }
        for i in 1..m-1 {
            hp.push(Elem(height_map[0][i], 0, i as i32));
            hp.push(Elem(height_map[n-1][i], n as i32-1, i as i32));
            visited[0][i] = true;
            visited[n-1][i] = true;
        }
        let mut ans = 0;
        let steps = [(-1,0),(1,0),(0,-1),(0,1)];
        while !hp.is_empty() {
            let Elem(h, x, y) = hp.pop().unwrap();
            visited[x as usize][y as usize] = true;
            for &(dx,dy) in steps.iter() {
                let (nx, ny) = (x+dx, y+dy);
                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 || visited[nx as usize][ny as usize] {
                    continue;
                }
                let (ux,uy) = (nx as usize,ny as usize);
                let nh = height_map[ux][uy];
                visited[ux][uy] = true;
                hp.push(Elem(nh.max(h), nx, ny));
                ans += 0.max(h-nh);
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
    fn test_trap_rain_water() {
        let test_cases = vec![
            (vec![
                vec![1,4,3,1,3,2],
                vec![3,2,1,3,2,4],
                vec![2,3,3,2,3,1],
            ], 4),
        ];
        for (map, expect) in test_cases {
            assert_eq!(Solution::trap_rain_water(map.clone()), expect, "map: {:?}", map);
        }
    }
}