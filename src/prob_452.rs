impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Ordering;
        let n = points.len();
        if n == 0 {
            return 0;
        }
        points.sort_by(|a,b| {
            let t = a[0].cmp(&b[0]);
            match t {
                Ordering::Equal => a[1].cmp(&b[1]),
                _ => t
            }
        });
        let mut count = 0;
        let mut end = points[0][1];
        let mut i = 1;
        while i < n {
            if points[i][0] > end {
                count+=1;
                end = points[i][1];
            } else if points[i][1] < end {
                end = points[i][1];
            }
            i+=1;
        }
        count+1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_foo() {
        let test_data = vec![
            (vec![vec![9,12],vec![1,10],vec![4,11],vec![8,12],vec![3,9],vec![6,9],vec![6,7]], 2),
            (vec![vec![3,9],vec![7,12],vec![3,8],vec![6,8],vec![9,10],vec![2,9],vec![0,9],vec![3,9],vec![0,6],vec![2,8]], 2),
        ];
        for (points, expect) in test_data {
            assert_eq!(expect, Solution::find_min_arrow_shots(points));
        }
    }
}