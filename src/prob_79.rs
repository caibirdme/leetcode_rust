impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let n = board.len();
        if n == 0 {
            return false;
        }
        let m = board[0].len();
        if word.is_empty() {
            return false;
        }
        let mut used = vec![vec![false; m]; n];
        let word = word.as_bytes();
        let w_0 = word[0];
        for i in 0..n {
            for j in 0..m {
                let c = board[i][j] as u8;
                if c == w_0 {
                    used[i][j] = true;
                    if Self::search(&board, i, j, &mut used, &word[1..]) {
                        return true;
                    }
                    used[i][j] = false;
                }
            }
        }
        false
    }
    fn search(board: &Vec<Vec<char>>, i: usize, j: usize, used: &mut Vec<Vec<bool>>, word: &[u8]) -> bool {
        if word.len() == 0 {
            return true;
        }
        let c = word[0];
        if i > 0 && !used[i-1][j] && board[i-1][j] as u8 == c {
            used[i-1][j] = true;
            if Self::search(board, i-1, j, used, &word[1..]) {
                return true;
            }
            used[i-1][j] = false;
        }
        if j > 0 && !used[i][j-1] && board[i][j-1] as u8 == c {
            used[i][j-1] = true;
            if Self::search(board, i, j-1, used, &word[1..]) {
                return true;
            }
            used[i][j-1] = false;
        }
        if i+1 < board.len() && !used[i+1][j] && board[i+1][j] as u8 == c {
            used[i+1][j] = true;
            if Self::search(board, i+1, j, used, &word[1..]) {
                return true;
            }
            used[i+1][j] = false;
        }
        if j+1 < board[0].len() && !used[i][j+1] && board[i][j+1] as u8 == c {
            used[i][j+1] = true;
            if Self::search(board, i, j+1, used, &word[1..]) {
                return true;
            }
            used[i][j+1] = false;
        }
        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_exist() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E'],
        ];
        let test_cases = vec![
            ("ABCCED", true),
            ("SEE", true),
            ("ABCB",false),
        ];
        for (word, ok) in test_cases {
            assert_eq!(Solution::exist(board.clone(), word.to_string()), ok, "word: {}", word);
        }
    }
}