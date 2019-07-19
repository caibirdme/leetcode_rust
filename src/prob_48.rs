impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let mut l = n-1;
        let mut x = 0;
        while l > 0 {
            for i in 0..l {
                let temp = matrix[x][x+i];
                matrix[x][x+i] = matrix[x+l-i][x];
                matrix[x+l-i][x] = matrix[x+l][x+l-i];
                matrix[x+l][x+l-i] = matrix[x+i][x+l];
                matrix[x+i][x+l] = temp;
            }
            x+=1;
            if l > 2 {
                l -= 2;
            } else {
                break;
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_rotate() {
        let test_cases = vec![
            (
                vec![
                    vec![1,2],
                    vec![3,4],
                ],
                vec![
                    vec![3,1],
                    vec![4,2],
                ],
            ),
            (
                vec![vec![1]],
                vec![vec![1]],
                ),
            (
                vec![
                    vec![1,2,3],
                    vec![4,5,6],
                    vec![7,8,9],
                ],
                vec![
                    vec![7,4,1],
                    vec![8,5,2],
                    vec![9,6,3],
                ],
            ),
            (
                vec![
                    vec![1,2,3,4],
                    vec![5,6,7,8],
                    vec![9,10,11,12],
                    vec![13,14,15,16],
                ],
                vec![
                    vec![13,9,5,1],
                    vec![14,10,6,2],
                    vec![15,11,7,3],
                    vec![16,12,8,4],
                ],
            ),
        ];
        for (mut matrix, expect) in test_cases {
            let cache = matrix.clone();
            Solution::rotate(&mut matrix);
            assert_eq!(matrix, expect, "matrix: {:?}", cache);
        }
    }
}