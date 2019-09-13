impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        let n = rooms.len();
        if n == 0 {
            return;
        }
        let m = rooms[0].len();
        if m == 0 {
            return;
        }
        const INF: i32 = std::i32::MAX;
        const GATE: i32 = 0;
        const WALL: i32 = -1;
        let mut q = vec![];
        for i in 0..n{
            for j in 0..m {
                if rooms[i][j] == GATE {
                    q.push((i,j));
                }
            }
        }
        let mut head = 0;
        while head <= q.len() {
            let (x,y) = q[head];
            head += 1;
            let v = rooms[x][y];
            if x > 0 && rooms[x-1][y] == INF {
                q.push((x-1,y));
                rooms[x-1][y] = v + 1;
            }
            if y > 0 && rooms[x][y-1] == INF {
                q.push((x,y-1));
                rooms[x][y-1] = v + 1;
            }
            if x+1 < n && rooms[x+1][y] == INF {
                q.push((x+1, y));
                rooms[x+1][y] = v+1;
            }
            if y+1 < m && rooms[x][y+1] == INF {
                q.push((x,y+1));
                rooms[x][y+1] = v + 1;
            }
        }
    }
}

struct Solution;