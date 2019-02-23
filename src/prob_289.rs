impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let dir = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
        let n = board.len();
        let m = board.first().unwrap().len();
        for i in 0..n {
            for j in 0..m {
                let mut count = 0;
                for (a,b) in dir.iter() {
                    if (i == 0 && *a == -1) ||  (j == 0 && *b == -1) || (i+1 == n && *a == 1) || (j+1==m && *b==1){
                        continue;
                    }
                    let x = if *a >= 0 {i+ *a as usize} else {i-((-(*a)) as usize)};
                    let y = if *b >= 0 {j+ *b as usize} else {j-((-(*b)) as usize)};
                    if x >= 0 && y >= 0 {
                        if board[x][y] == 1 || board[x][y] == 3 {
                            count += 1;
                        }
                    }
                }
                if board[i][j] == 0 {
                    if count == 3 {
                        board[i][j] = 2;
                    }
                } else {
                    if count < 2 {
                        board[i][j] = 3;
                    } else if count > 3 {
                        board[i][j] = 3;
                    }
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                if board[i][j] == 2 {
                    board[i][j] = 1;
                } else if board[i][j] == 3 {
                    board[i][j] = 0;
                }
            }
        }
    }
}

struct Solution;