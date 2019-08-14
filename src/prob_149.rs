use std::collections::HashMap;
use std::cmp::max;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 1 {
            return n as i32;
        }
        let mut ans = 2;
        for i in 0..n-1 {
            let mut slope = HashMap::new();
            let mut bx = 1;
            let mut by = 1;
            let mut repeat = 1;
            for j in i+1..n {
                let dy = points[i][1]-points[j][1];
                let dx = points[i][0]-points[j][0];
                if dx == 0 && dy == 0 {
                    repeat += 1;
                    bx+=1;
                    by+=1;
                    continue;
                }
                if dx == 0 {
                    bx += 1;
                    continue;
                }
                if dy == 0 {
                    by += 1;
                    continue;
                }
                let k = Self::gcd(dy, dx);
                let mut flag = 0;
                if dy < 0 {
                    flag = 1 - flag;
                }
                if dx < 0 {
                    flag = 1 - flag;
                }
                let ddy = {
                    if dy < 0 {
                        -dy / k
                    } else {
                        dy / k
                    }
                };
                let ddx = {
                    if dx < 0 {
                        -dx / k
                    } else {
                        dx / k
                    }
                };
                match slope.get_mut(&(ddy, ddx, flag)) {
                    Some(v) => {*v+=1;},
                    None => {slope.insert((ddy, ddx, flag), 1);},
                }
            }
            let mut t = 0;
            slope.values().for_each(|&v| t = max(t, v));
            ans = max(ans, t+repeat);
            ans = max(ans, bx);
            ans = max(ans, by);
        }
        ans
    }
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        if a < 0 {
            a = -a;
        }
        if b < 0 {
            b = -b;
        }
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        let mut c = a % b;
        while c != 0 {
            a = b;
            b = c;
            c = a % b;
        }
        b
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_points() {
        let test_cases = vec![
            (vec![vec![1,1], vec![1,1], vec![2,3]], 3),
            (vec![vec![0,0], vec![1,1], vec![0,0]], 3),
            (vec![vec![1,1], vec![1,2], vec![1,3], vec![2,3]], 3),
            (vec![vec![1,1],vec![3,2],vec![5,3],vec![4,1],vec![2,3],vec![1,4]], 4),
            (vec![vec![1,1],vec![2,2],vec![3,3]], 3),
        ];
        for (points, expect) in test_cases {
            assert_eq!(Solution::max_points(points.clone()), expect, "points: {:?}", points);
        }
    }
}