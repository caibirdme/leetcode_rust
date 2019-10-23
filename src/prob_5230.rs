impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        if coordinates.is_empty() {
            return false;
        }
        let n = coordinates.len();
        if n == 1 {
            return true;
        }
        let (x0,y0) = (coordinates[0][0], coordinates[0][1]);
        let (x1,y1) = (coordinates[1][0], coordinates[1][1]);
        let mut dy = y1-y0;
        if dy == 0 {
            for i in 2..n {
                if coordinates[i][1] != y0 {
                    return false;
                }
                return true;
            }
        }
        let mut dx = x1-x0;
        if dx == 0 {
            for i in 2..n {
                if coordinates[i][0] != x0 {
                    return false;
                }
                return true;
            }
        }
        let g = Self::abs_gcd(dy, dx);
        dy /= g;
        dx /= g;
        let flag = Self::get_f(dx, dy);
        dx = Self::abs(dx);
        dy = Self::abs(dy);
        for i in 2..n {
            let nx = coordinates[i][0];
            let ny = coordinates[i][1];
            let mut dnx = nx-x0;
            let mut dny = ny-y0;
            if dnx == 0 || dny == 0 {
                return false;
            }
            let g = Self::abs_gcd(dny, dnx);
            dnx /= g;
            dny /= g;
            if Self::get_f(dnx,dny) != flag {
                return false;
            }
            if Self::abs(dnx) != dx || Self::abs(dny) != dy {
                return false;
            }
        }
        true
    }
    fn abs(x: i32) -> i32 {
        if x < 0 {
            -x
        } else {
            x
        }
    }
    fn get_f(x: i32, y: i32) -> bool {
        if (x < 0 && y < 0) || (x>0&&y>0) {
            true
        } else {
            false
        }
    }
    fn abs_gcd(mut x: i32, mut y: i32) -> i32 {
        if x < 0 {
            x = -x;
        }
        if y < 0 {
            y = -y;
        }
        Self::gcd(x, y)
    }
    fn gcd(mut x: i32, mut y: i32) -> i32 {
        if y > x {
            return Self::gcd(y, x);
        }
        while x % y != 0 {
            let t = x%y;
            x = y;
            y = t;
        }
        y
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_check_straight_line() {
        let test_cases = vec![
            (vec![vec![0,1],vec![1,3],vec![-4,-7],vec![5,11]], true),
            (vec![vec![1,1],vec![2,2],vec![3,4],vec![4,5],vec![5,6],vec![7,7]], false),
            (vec![vec![1,1],vec![2,2],vec![3,3],vec![4,4],vec![5,5],vec![7,7]], true),
            (vec![vec![-3,-2],vec![-1,-2],vec![2,-2],vec![-2,-2],vec![0,-2]], true),
            (vec![vec![0,1],vec![1,3],vec![-4,-7],vec![5,11]], true),

        ];
        for (coordinates, expect) in test_cases {
            assert_eq!(Solution::check_straight_line(coordinates.clone()), expect, "coordinates: {:?}", coordinates);
        }
    }
}