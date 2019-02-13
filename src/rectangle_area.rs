impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        if a > e {
            return Self::compute_area(e,f,g,h,a,b,c,d);
        }
        let total = (c-a)*(d-b) + (g-e)*(h-f);
        // no intersection
        if h<=b || f>=d || c<=e || a>=g {
            return total;
        }
        // contain
        if b<=f && c>=g && d>=h {
            return total-(g-e)*(h-f);
        }

        if h>=d {
            if f >= b {
                if g<=c {
                    total-(g-e)*(d-f)
                } else {
                    total-(d-f)*(c-e)
                }
            } else {
                if g<=c {
                    total-(d-b)*(g-e)
                } else {
                    total-(d-b)*(c-e)
                }
            }
        } else {
            if g<=c {
                total-(h-b)*(g-e)
            } else {
                if f>=b {
                    total-(h-f)*(c-e)
                } else {
                    total-(h-b)*(c-e)
                }
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_compute_area() {
        assert_eq!(
            Solution::compute_area(-2,-2,2,2,-3,-3,-1,3),
            24
        );
        assert_eq!(
            Solution::compute_area(-2,-2,2,2,-2,-3,3,3),
            30
        );
        assert_eq!(
            Solution::compute_area(-2,-2,2,2,-2,-3,2,3),
            24
        );
        assert_eq!(
            Solution::compute_area(-2,-2,2,2,-1,-1,1,1),
            16
        );
        assert_eq!(
            Solution::compute_area(-2,-2,2,2,-3,-3,3,-1),
            24
        );
        assert_eq!(
            Solution::compute_area(-3,0,3,4,0,-1,9,2),
            45
        );
        assert_eq!(
            Solution::compute_area(0,-1,9,2,-3,0,3,4),
            45
        );
        assert_eq!(
            Solution::compute_area(0,0,1,1,1,1,2,2),
            2
        );

    }
}