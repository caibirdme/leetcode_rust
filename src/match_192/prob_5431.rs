impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let target = target as usize;
        let mut f = vec![vec![vec![std::i32::MAX; n+1]; target+1]; m+1];
        // 初始化
        if houses[0] > 0 {
            f[0][1][houses[0] as usize] = 0;
        } else {
            for c in 1..=n {
                f[0][1][c] = cost[0][c-1];
            }
        }
        // 枚举房子
        for i in 1..m {
            // 枚举街区
            for j in 1..=target {
                // 如果当前房子已经着色
                if houses[i] > 0 {
                    let hc = houses[i] as usize;
                    // 和前面一样
                    if f[i-1][j][hc] != std::i32::MAX {
                        f[i][j][hc] = f[i-1][j][hc];
                    }
                    // 新街区
                    for k in 1..=n {
                        if k != hc && f[i-1][j-1][k] != std::i32::MAX {
                            f[i][j][hc] = f[i][j][hc].min(f[i-1][j-1][k]);
                        }
                    }
                    continue;
                }
                // 当前房子没颜色，枚举一个
                for c in 1..=n {
                    if f[i-1][j][c] != std::i32::MAX {
                        f[i][j][c] = f[i-1][j][c] + cost[i][c-1];
                    }
                    for k in 1..=n {
                        if k != c && f[i-1][j-1][k] != std::i32::MAX {
                            f[i][j][c] = f[i][j][c].min(f[i-1][j-1][k] + cost[i][c-1]);
                        }
                    }
                }
            }
        }
        let mut ans = std::i32::MAX;
        for c in 1..=n {
            ans = ans.min(f[m-1][target][c]);
        }
        if ans == std::i32::MAX {
            -1
        } else {
            ans
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_cost() {
        let test_cases = vec![
            (
                vec![0,0,0,3],
                vec![
                    vec![2,2,5],
                    vec![1,5,5],
                    vec![5,1,2],
                    vec![5,2,5],
                ],
                4,3,3,
                4
            ),
            (
                vec![3,1,2,3],
                vec![
                    vec![1,1,1],
                    vec![1,1,1],
                    vec![1,1,1],
                    vec![1,1,1],
                    vec![1,1,1],
                ],
                4,3,3,
                -1
            ),
            (
                vec![0,0,0,0,0],
                vec![
                    vec![1,10],
                    vec![10,1],
                    vec![1,10],
                    vec![10,1],
                    vec![1,10],
                ],
                5,2,5,
                5
            ),

            (
                vec![0,2,1,2,0],
                vec![
                    vec![1,10],
                    vec![10,1],
                    vec![10,1],
                    vec![1,10],
                    vec![5,1],
                ],
                5,2,3,
                11
            ),
            (
                vec![0,0,0,0,0],
                vec![
                    vec![1,10],
                    vec![10,1],
                    vec![10,1],
                    vec![1,10],
                    vec![5,1],
                ],
                5,2,3,
                9
            ),

        ];
        for(houses, cost, m,n,target, expect) in test_cases {
            assert_eq!(Solution::min_cost(houses.clone(), cost.clone(), m,n,target), expect, "houses:{:?}, cost: {:?}, m:{},n:{},target:{}",houses,cost,m,n,target);
        }
    }
}