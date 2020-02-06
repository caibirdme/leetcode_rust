impl Solution {
    pub fn max_vacation_days(flights: Vec<Vec<i32>>, days: Vec<Vec<i32>>) -> i32 {
        let n = flights.len();
        let K = days[0].len();
        let mut f = [vec![-1; n], vec![-1; n]];
        f[0][0] = days[0][0];
        for i in 1..n {
            if flights[0][i]==1 {
                f[0][i] = days[i][0];
            }
        }
        let mut idx = 1;
        for k in 1..K {
            let nidx = idx ^ 1;
            for i in 0..n {
                for j in 0..i {
                    if f[nidx][j] != -1 && flights[j][i] == 1 {
                        f[idx][i] = f[idx][i].max(f[nidx][j]+days[i][k]);
                    }
                }
                if f[nidx][i] != -1 {
                    f[idx][i] = f[idx][i].max(f[nidx][i]+days[i][k]);
                }
                for j in i+1..n {
                    if f[nidx][j] != -1 && flights[j][i] == 1 {
                        f[idx][i] = f[idx][i].max(f[nidx][j]+days[i][k]);
                    }
                }
            }
            idx = nidx;
        }
        *f[idx ^ 1].iter().max().unwrap()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_vacation_days() {
        let test_cases = vec![
            (
                vec![
                    vec![0,1,1],
                    vec![1,0,1],
                    vec![1,1,0],
                ],
                vec![
                    vec![1,3,1],
                    vec![6,0,3],
                    vec![3,3,3],
                ],
                12
            ),
            (
                vec![
                    vec![0,0,0],
                    vec![0,0,0],
                    vec![0,0,0],
                ],
                vec![
                    vec![1,1,1],
                    vec![7,7,7],
                    vec![7,7,7],
                ],
                3
            ),
            (
                vec![
                    vec![0,1,1],
                    vec![1,0,1],
                    vec![1,1,0],
                ],
                vec![
                    vec![7,0,0],
                    vec![0,7,0],
                    vec![0,0,7],
                ],
                21
            ),
        ];
        for (flights, days, expect) in test_cases {
            assert_eq!(Solution::max_vacation_days(flights.clone(), days.clone()), expect, "flights: {:?}, days: {:?}", flights,days);
        }
    }
}