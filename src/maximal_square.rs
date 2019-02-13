use std::cmp::{max,min};
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        if n == 0 {
            return 0;
        }
        let m = matrix.first().unwrap().len();
        if m == 0 {
            return 0;
        }
        let mut col= vec![vec![0 as usize; m]; n];
        let mut row= vec![vec![0 as usize; m]; n];
        let mut ans = 0 as usize;
        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] != '1' {
                    continue;
                }
                if j > 0 {
                    row[i][j] = row[i][j-1] + 1;
                } else {
                    row[i][j] = 1;
                }
                if ans == 0 {
                    ans = 1;
                }
                let mut t = row[i][j];
                let mut temp_i = i;
                let mut count = 2;
                while temp_i > 0 && count <= t{
                    temp_i-=1;
                    let q = row[temp_i][j];
                    if q >= count {
                        ans = max(ans, count*count);
                        t = min(t, q);
                        count+=1;
                    } else {
                        break;
                    }
                }
            }
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_maximal_square() {
        assert_eq!(
          Solution::maximal_square(vec![
              vec!['1','0','1','0','0'],
              vec!['1','0','1','1','1'],
              vec!['1','1','1','1','1'],
              vec!['1','0','0','1','0'],
          ]),
            4
        );
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1','0','1','0','0'],
                vec!['1','1','1','1','1'],
                vec!['1','1','1','1','1'],
                vec!['1','1','1','1','0'],
            ]),
            9
        );
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1','1','1','1','0'],
                vec!['1','1','1','1','1'],
                vec!['1','1','1','1','1'],
                vec!['1','1','1','1','0'],
            ]),
            16
        );
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1','1','1','1','0'],
                vec!['1','1','1','1','1'],
                vec!['1','1','0','1','1'],
                vec!['1','1','1','1','0'],
            ]),
            4
        );
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['0','0','0','0','0'],
                vec!['0','0','0','0','0'],
                vec!['0','0','0','0','0'],
                vec!['0','0','0','0','0'],
            ]),
            0
        );
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1','0','0','0','0'],
                vec!['0','0','0','0','0'],
                vec!['0','0','0','0','0'],
                vec!['0','0','0','0','0'],
            ]),
            1
        );
    }
}