impl Solution {
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = matrix.len();
        if n == 0 {
            return vec![];
        }
        let m = matrix[0].len();
        if m == 0 {
            return vec![];
        }
        if n == 1 {
            let mut ans = vec![];
            for i in 0..m {ans.push(vec![0, i as i32]);}
            return ans;
        }
        if m == 1 {
            let mut ans = vec![];
            for i in 0..n {ans.push(vec![i as i32, 0]);}
            return ans;
        }
        let mut pacific:Vec<Vec<Option<bool>>> = vec![vec![None;m];n];
        let mut atlantic:Vec<Vec<Option<bool>>> = vec![vec![None;m];n];
        let mut ans: Vec<Vec<i32>> = vec![];
        ans
    }
    fn dfs(matrix: &Vec<Vec<i32>>)
}

struct Solution;