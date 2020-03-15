impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len();
        if n == 0 {
            return vec![];
        }
        let m = matrix[0].len();
        let mut cols = vec![std::i32::MIN; m];
        let mut rows = vec![std::i32::MAX; n];
        for i in 0..n {
            for j in 0..m {
                let t = matrix[i][j];
                rows[i] = rows[i].min(t);
                cols[j] = cols[j].max(t);
            }
        }
        let mut ans = vec![];
        for i in 0..n {
            for j in 0..m {
                let t = matrix[i][j];
                if t == rows[i] && t == cols[j] {
                    ans.push(t);
                }
            }
        }
        ans
    }
}

struct Solution;