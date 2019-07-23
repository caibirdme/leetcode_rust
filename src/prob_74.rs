impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len();
        if n == 0 {
            return false;
        }
        let m = matrix[0].len();
        if m == 0 {
            return false;
        }
        let mut i = n-1;
        let mut j = 0;
        while i >= 0 && j < m {
            let t = matrix[i][j];
            if target == t {
                return true;
            }
            if target < t {
                if i == 0 {
                    return false;
                }
                i-=1;
            } else {
                if j+1 == m {
                    return false;
                }
                j+=1;
            }
        }
        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_search_matrix() {
        let test_cases = vec![
            (vec![
                vec![1],
                vec![2],
                vec![5],
                vec![7],
                vec![8],
            ], 7, true),
            (vec![
                vec![1],
                vec![2],
                vec![5],
                vec![7],
                vec![9],
            ], 10, false),
            (vec![
                vec![1],
                vec![2],
                vec![5],
                vec![7],
                vec![9],
            ], -1, false),
            (vec![
                vec![1],
                vec![2],
                vec![5],
                vec![7],
                vec![9],
            ], 6, false),
            (vec![
                vec![1,3,5,7,9,11],
            ],11,true),
            (vec![
                vec![1,   3,  5,  7],
                vec![10, 11, 16, 20],
                vec![23, 30, 34, 50],
            ], 3, true,),
            (vec![
                vec![1,   3,  5,  7],
                vec![10, 11, 16, 20],
                vec![23, 30, 34, 50],
            ], 13, false,),
        ];
        for (matrix, target, ok) in test_cases {
            assert_eq!(Solution::search_matrix(matrix.clone(), target), ok, "matrix: {:?}, target: {}", matrix, target);
        }
    }
}