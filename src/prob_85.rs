impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        if n == 0 {
            return 0;
        }
        let m = matrix[0].len();
        if m == 0 {
            return 0;
        }
        let mut heights = vec![0; m];
        let mut ans = 0;
        let mut q: Vec<usize> = vec![];
        for i in 0..n {
            q.clear();
            for j in 0..m {
                if matrix[i][j] == '0' {
                    heights[j] = 0;
                } else {
                    heights[j] += 1;
                }
                let cur = heights[j];
                while let Some(&peak) = q.last() {
                    if heights[peak] < cur {
                        break;
                    }
                    q.pop();
                    if q.is_empty() {
                        ans = ans.max(j as i32 * heights[peak]);
                    } else {
                        ans = ans.max((j-*q.last().unwrap()-1) as i32 * heights[peak]);
                    }
                }
                q.push(j);
            }
            while let Some(pos) = q.pop() {
                if q.is_empty() {
                    ans = ans.max(m as i32 * heights[pos]);
                } else {
                    ans = ans.max((m-*q.last().unwrap()-1) as i32 * heights[pos]);
                }
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
    fn test_maximal_rectangle() {
        let test_cases = vec![
            (
                vec![
                  vec!["1","0","1","0","0"],
                  vec!["1","0","1","1","1"],
                  vec!["1","1","1","1","1"],
                  vec!["1","0","0","1","0"],
                ],
                6,
            ),
        ];
        for (matrix, expect) in test_cases {
            assert_eq!(Solution::maximal_rectangle(matrix.iter().map(|v| v.iter().map(|s| s.as_bytes()[0] as char).collect()).collect()), expect, "matrix: {:?}", matrix);
        }
    }
}