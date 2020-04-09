impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        if n == 0 {
            return false;
        }
        let m = matrix[0].len();
        for i in 0..n-1 {
            let t = matrix[i][0];
            let (mut x, mut y) = (i,0);
            while x < n && y < m {
                if matrix[x][y] != t {
                    return false;
                }
                x+=1;
                y+=1;
            }
        }
        for i in 1..m-1 {
            let t = matrix[0][i];
            let (mut x, mut y) = (0,i);
            while x < n && y < m {
                if matrix[x][y] != t {
                    return false;
                }
                x+=1;
                y+=1;
            }
        }
        true
    }
}

struct Solution;