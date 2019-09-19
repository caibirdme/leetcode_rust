impl Solution {
    pub fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
        let (a,b) = Self::find_col(&image, y as usize);
        let (c,d) = Self::find_row(&image, x as usize);
        ((b-a+1)*(d-c+1)) as i32
    }
    fn find_col(img: &Vec<Vec<char>>, x: usize) -> (usize, usize) {
        let (mut l,mut r) = (0,x);
        while l<r {
            let m = (l+r)/2;
            if Self::check_col(img, m) {
                r = m;
            } else {
                l = m+1;
            }
        }
        let ll = r;
        l = x;
        r = img[0].len()-1;
        let mut rr = x;
        while l <= r {
            let m = (l+r)/2;
            if Self::check_col(img, m) {
                rr = m;
                l = m+1;
            } else {
                r = m-1;
            }
        }
        (ll, rr)
    }
    fn check_col(img: &Vec<Vec<char>>, m: usize) -> bool {
        let n = img.len();
        for i in 0..n {
            if img[i][m] == '1' {
                return true;
            }
        }
        false
    }
    fn check_row(img: &Vec<Vec<char>>, m: usize) -> bool {
        let n = img[0].len();
        for i in 0..n {
            if img[m][i] == '1' {
                return true;
            }
        }
        false
    }
    fn find_row(img: &Vec<Vec<char>>, x: usize) -> (usize, usize) {
        let (mut l,mut r) = (0,x);
        while l<r {
            let m = (l+r)/2;
            if Self::check_row(img, m) {
                r = m;
            } else {
                l = m+1;
            }
        }
        let ll = r;
        l = x;
        r = img.len()-1;
        let mut rr = x;
        while l <= r {
            let m = (l+r)/2;
            if Self::check_row(img, m) {
                rr = m;
                l = m+1;
            } else {
                r = m-1;
            }
        }
        (ll, rr)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_area() {
        let test_cases = vec![
            (vec![
                "0010",
                "0110",
                "0100"
            ], 0,2, 6),
            (vec![
                "0000000",
                "0000000",
                "0000100",
                "0000000",
            ], 2,4, 1),
            (vec![
                "0000000",
                "0000100",
                "0000110",
                "0000000",
            ], 2,4, 4),
        ];
        for (image, x,y, expect) in test_cases {
            let img = image.clone()
                .into_iter()
                .map(|v| v.as_bytes().into_iter().map(|&c| c as char).collect())
                .collect();
            assert_eq!(Solution::min_area(img, x,y), expect, "image: {:?}", image);
        }
    }
}