impl Solution {
    pub fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        if x1<=x_center && y1 <= y_center && x2>=x_center && y2 >= y_center {
            return true;
        }
        if x1 > x_center+radius || x2 < x_center-radius {
            return false;
        }
        if y1 >= y_center {
            if y1 > y_center + radius {
                false
            } else if y1 == y_center + radius {
                x1 <= x_center && x2 >= x_center
            } else {
                let w = radius.pow(2)-(y1-y_center).pow(2);
                if x2 < x_center && (x2-x_center).pow(2) > w {
                    return false;
                }
                if x1 > x_center && (x1-x_center).pow(2) > w {
                    return false;
                }
                true
            }
        } else if y2 <= y_center {
            if y2 < y_center-radius {
                false
            } else if y2 == y_center-radius {
                x1 <= x_center && x2 >= x_center
            } else if x1<=x_center-radius && x2>=x_center+radius {
                true
            } else {
                let w = radius.pow(2)-(y2-y_center).pow(2);
                if x2 < x_center && (x2-x_center).pow(2) > w {
                    return false;
                }
                if x1 > x_center && (x1-x_center).pow(2) > w {
                    return false;
                }
                true
            }
        } else {
            true
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_check_overlap() {
        let test_cases = vec![
            (1415,807,-784,-733,623,-533,1005, false),
            (1,0,3,7,3,10,6, false),
            (1,0,0,1,-1,3,1, true),
            (1,0,0,-1,0,0,1, true),
            (1,1,1,-3,-3,3,3,true),
            (1,1,1,1,-3,2,-1, false),
        ];
        for (r, xc,yc,x1,y1,x2,y2,ok) in test_cases {
            assert_eq!(Solution::check_overlap(r,xc,yc,x1,y1,x2,y2), ok, "r:{},xc:{},yc:{},x1:{},y1:{},x2:{},y2:{}", r,xc,yc,x1,y1,x2,y2);
        }
    }
}