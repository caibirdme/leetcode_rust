impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n = n as i64;
        let mut width = 1;
        let mut count = 1;
        // 确定位数
        loop {
            let next =  count*9*width;
            if n <= next {
                break;
            }
            n -= next;
            count *= 10;
            width+=1;
        }
        let pos = {
            if n%width==0 {
                n/width
            } else {
                n/width + 1
            }
        };
        let mut t = count + pos - 1;
        if n % width == 0 {
            (t%10) as i32
        } else {
            let q = width-(n-n/width * width);
            for _ in 0..q {
                t/=10;
            }
            (t%10) as i32
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_nth_digit() {
        let test_cases = vec![
            (1000000000, 1),
            (190,1),
            (191,0),
            (192,0),
            (186,9),
            (187,8),
            (188,9),
            (189,9),
            (100,5),
            (3,3),
            (11,0),
            (12,1),
            (13,1),
            (14,1),
            (15,2),
        ];
        for (n,e) in test_cases {
            assert_eq!(e, Solution::find_nth_digit(n), "n: {}", n);
        }
    }
}