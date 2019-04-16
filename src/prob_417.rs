use std::collections::HashSet;

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
        let mut count = vec![vec![0; m]; n];
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue = vec![];
        for i in 0..m {
            visited.insert((0,i));
            queue.push((0usize,i));
            count[0][i] = 1;
        }
        for i in 0..n {
            visited.insert((i,0));
            queue.push((i,0));
            count[i][0] = 1;
        }
        let mut head = 0;
        while head<queue.len() {
            let (x,y) = queue[head];
            head += 1;
            let t = matrix[x][y];
            if x > 0 && t <= matrix[x-1][y] && !visited.contains(&(x-1, y)) {
                queue.push((x-1, y));
                visited.insert((x-1, y));
                count[x-1][y] = 1;
            }
            if x+1 < n && t <= matrix[x+1][y] && !visited.contains(&(x+1, y)) {
                queue.push((x+1, y));
                visited.insert((x+1, y));
                count[x+1][y] = 1;
            }
            if y > 0 && t <= matrix[x][y-1] && !visited.contains(&(x,y-1)) {
                queue.push((x, y-1));
                visited.insert((x, y-1));
                count[x][y-1] = 1;
            }
            if y+1 < m && t<= matrix[x][y+1] && !visited.contains(&(x, y+1)) {
                queue.push((x, y+1));
                visited.insert((x, y+1));
                count[x][y+1] = 1;
            }
        }
        queue.clear();
        head = 0;
        visited.clear();
        let mut ans: Vec<Vec<i32>> = vec![];
        for i in 0..m {
            queue.push((n-1, i));
            visited.insert((n-1, i));
            count[n-1][i] += 1;
            if count[n-1][i] == 2 {
                ans.push(vec![(n-1) as i32, i as i32]);
            }
        }
        for i in 0..n-1 {
            queue.push((i, m-1));
            visited.insert((i, m-1));
            count[i][m-1] += 1;
            if count[i][m-1] == 2 {
                ans.push(vec![i as i32, (m-1) as i32]);
            }
        }
        while head<queue.len() {
            let (x,y) = queue[head];
            head += 1;
            let t = matrix[x][y];
            if x > 0 && t <= matrix[x-1][y] && !visited.contains(&(x-1, y)) {
                queue.push((x-1, y));
                visited.insert((x-1, y));
                count[x-1][y] += 1;
                if count[x-1][y] > 1 {
                    ans.push(vec![(x-1) as i32, y as i32]);
                }
            }
            if x+1 < n && t <= matrix[x+1][y] && !visited.contains(&(x+1, y)) {
                queue.push((x+1, y));
                visited.insert((x+1, y));
                count[x+1][y] += 1;
                if count[x+1][y] > 1 {
                    if x+1 == 4 && y == 4 {
                        println!("qqqqqqqqqqqqqq");
                    }
                    ans.push(vec![(x+1) as i32, y as i32]);
                }
            }
            if y > 0 && t <= matrix[x][y-1] && !visited.contains(&(x,y-1)) {
                queue.push((x, y-1));
                visited.insert((x, y-1));
                count[x][y-1] += 1;
                if count[x][y-1] > 1 {
                    ans.push(vec![x as i32, (y-1) as i32]);
                }
            }
            if y+1 < m && t<= matrix[x][y+1] && !visited.contains(&(x, y+1)) {
                queue.push((x, y+1));
                visited.insert((x, y+1));
                count[x][y+1] += 1;
                if count[x][y+1] > 1 {
                    if x == 4 && y+1 == 4 {
                        println!("ttttttttttttttttttttttttt");
                    }
                    ans.push(vec![x as i32, (y+1) as i32]);
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
    fn test_pacific_atlantic() {
        let test_cases = vec![
            (vec![
                vec![1,2,2,3,5],
                vec![3,2,3,4,4],
                vec![2,4,5,3,1],
                vec![6,7,1,4,5],
                vec![5,1,1,2,4]
            ],
             vec![vec![0, 4], vec![1, 3], vec![1, 4], vec![2, 2], vec![3, 0], vec![3, 1], vec![4, 0]]
            ),
        ];
        for (matrix, mut expect) in test_cases {
            expect.sort();
            let mut actual = Solution::pacific_atlantic(matrix.clone());
            actual.sort();
            assert_eq!(expect, actual, "matrix: {:?}", matrix);
        }
    }
}