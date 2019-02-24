impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        if n == 1 {
            return vec![0];
        }
        if n == 2 {
            return vec![0,1];
        }
        let mut graph = vec![vec![false; n]; n];
        let mut edges_count = vec![0; n];
        for edge in edges {
            let (x,y) = (edge[0] as usize, edge[1] as usize);
            edges_count[x] += 1;
            edges_count[y] += 1;
            graph[x][y] = true;
            graph[y][x] = true;
        }
        let mut found = vec![];
        let mut deleted = vec![false; n];
        let mut rest = n;
        while rest > 2 {
            for i in 0..n{
                if edges_count[i] == 1 {
                    edges_count[i] -= 1;
                    rest -= 1;
                    found.push(i);
                    deleted[i] = true;
                }
            }
            for &i in found.iter() {
                for j in 0..n{
                    if graph[i][j] {
                        graph[i][j] = false;
                        graph[j][i] = false;
                        edges_count[j] -= 1;
                    }
                }
            }
            found.clear();
        }
        deleted.into_iter().enumerate().filter(|i| !i.1).map(|x| x.0 as i32).collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_min_height_trees() {
        assert_eq!(
            Solution::find_min_height_trees(4, vec![vec![1,0], vec![1,2], vec![1,3]]),
            vec![1]
        );
        assert_eq!(
            Solution::find_min_height_trees(6, vec![vec![0,3], vec![1,3], vec![2,3], vec![4,3], vec![5,4]]),
            vec![3,4]
        );
    }
}