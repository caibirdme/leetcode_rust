impl Solution {
    pub fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        if m == 0 {
            return 0;
        }
        let mut row = vec![0; n];
        let mut col = vec![0; m];
        let mut total = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    row[i] += 1;
                    col[j] += 1;
                    total += 1;
                }
            }
        }
        let mid = (total + 1) >> 1;
        let (x,y) = (Self::get_index(&row, mid), Self::get_index(&col, mid));
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    if x > i {
                        ans += x-i;
                    } else {
                        ans += i-x;
                    }
                    if y > j {
                        ans += y-j;
                    } else {
                        ans += j-y;
                    }
                }
            }
        }
        ans as i32
    }
    #[inline]
    fn get_index(v : &Vec<i32>, target: i32) -> usize {
        let mut t = 0;
        let n = v.len();
        for i in 0..n {
            t += v[i];
            if t >= target {
                return i;
            }
        }
        n-1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_total_distance() {
        let test_cases = vec![
            (vec![
                vec![1,0,0,0,1],
                vec![0,0,0,0,0],
                vec![0,0,1,0,0],
            ], 6),
        ];
        for (grid, expect) in test_cases {
            assert_eq!(Solution::min_total_distance(grid.clone()), expect, "grid: {:?}", grid);
        }
    }
}