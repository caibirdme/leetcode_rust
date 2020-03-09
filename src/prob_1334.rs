impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        const INF: i32 = std::i32::MAX/2;
        let mut f = vec![vec![INF; n]; n];
        for edge in edges {
            let (x,y,v) = (edge[0] as usize, edge[1] as usize, edge[2]);
            f[x][y] = v;
            f[y][x] = v;
        }
        for k in 0..n {
            for i in 0..n {
                if i != k {
                    for j in 0..n {
                        if j != i && j != k{
                            let t = f[i][k]+f[k][j];
                            if f[i][j] > t {
                                f[i][j] = t;
                            }
                        }
                    }
                }
            }
        }
        let mut min_cities = n as i32;
        let mut ans = 0;
        for i in 0..n {
            let mut count = 0;
            for j in 0..n {
                if j != i && f[i][j] <= distance_threshold {
                    count+=1;
                }
            }
            if count <= min_cities {
                ans = i;
                min_cities = count;
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
    fn test_find_the_city() {
        let test_cases = vec![
            (4, vec![
                vec![0,1,3],vec![1,2,1],vec![1,3,4],vec![2,3,1],
            ], 4, 3),
            (5, vec![
                vec![0,1,2],vec![0,4,8],vec![1,2,3],vec![1,4,2],vec![2,3,1],vec![3,4,1]
            ], 2, 0),
        ];
        for (n, edges, distance_threshold, expect) in test_cases {
            assert_eq!(Solution::find_the_city(n, edges.clone(), distance_threshold), expect, "n:{}, edges: {:?}, threshold:{}", n, edges, distance_threshold);
        }
    }
}