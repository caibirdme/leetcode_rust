use std::cmp::max;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        if n == 0 {
            return 0;
        }
        let m = matrix.first().unwrap().len();
        if m == 0 {
            return 0;
        }
        let mut f: Vec<Vec<i32>> = vec![vec![0; m]; n];
        let mut ans = 1;
        for i in 0..n {
            for j in 0..m {
                ans = max(ans, Self::dfs(&matrix, n,m,i,j,&mut f));
            }
        }
        ans
    }
    fn dfs(matrix: &Vec<Vec<i32>>,n:usize,m:usize, x: usize, y: usize, f: &mut Vec<Vec<i32>>) -> i32 {
        if f[x][y] != 0 {
            return f[x][y];
        }
        let mut ans = 0;
        let cur = matrix[x][y];
        if x > 0 && matrix[x-1][y] > cur {
            ans = max(ans, Self::dfs(matrix, n,m,x-1, y, f));
        }
        if x+1<n && matrix[x+1][y]>cur {
            ans = max(ans, Self::dfs(matrix,n,m,x+1,y,f));
        }
        if y>0 && matrix[x][y-1] > cur {
            ans = max(ans, Self::dfs(matrix, n,m,x,y-1,f));
        }
        if y+1<m && matrix[x][y+1] > cur {
            ans = max(ans, Self::dfs(matrix,n,m,x,y+1,f));
        }
        ans += 1;
        f[x][y] = ans;
        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_longest_increasing_path() {
        let test_cases  = vec![
            (vec![vec![3,4,5],vec![3,2,6],vec![2,2,1]], 4),
            (vec![vec![9,9,4], vec![6,6,8],vec![2,1,1]], 4),
        ];
        for (matrix, expect) in test_cases {
            assert_eq!(
                Solution::longest_increasing_path(matrix),
                expect
            );
        }
    }
}