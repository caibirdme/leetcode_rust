use std::cmp::min;

impl Solution {
    pub fn min_available_duration(mut slots1: Vec<Vec<i32>>, mut slots2: Vec<Vec<i32>>, duration: i32) -> Vec<i32> {
        let n = slots1.len();
        let m = slots2.len();
        if n == 0 || m == 0 {
            return vec![];
        }
        slots1.sort_by(|a,b| a[0].cmp(&b[0]));
        slots2.sort_by(|a,b| a[0].cmp(&b[0]));
        let n = slots1.len();
        let m = slots2.len();
        let (mut i, mut j) = (0,0);
        while i<n && j<m {
            if let Some((s,e)) = Self::get_overlap((slots1[i][0], slots1[i][1]), (slots2[j][0], slots2[j][1])) {
                if s+duration <= e {
                    return vec![s, s+duration];
                }
            }
            if slots1[i][1] >= slots2[j][1] {
                j += 1;
            } else {
                i += 1;
            }
        }
        vec![]
    }
    fn get_overlap(x: (i32, i32), y: (i32, i32)) -> Option<(i32, i32)> {
        if x.1 > y.0 && x.0 <= y.0 {
            return Some((y.0, min(x.1, y.1)));
        }
        if y.1 > x.0 && y.0 <= x.0{
            return Some((x.0, min(y.1,x.1)));
        }
        None
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_available_duration() {
        let test_cases = vec![
            (vec![vec![10,50],vec![60,120],vec![140,210]], vec![vec![0,15],vec![60,70]], 12, vec![]),
            (vec![vec![10,50],vec![60,120],vec![140,210]], vec![vec![0,15],vec![60,70]], 8, vec![60, 68]),
        ];
        for (s1, s2, duration, expect) in test_cases {
            assert_eq!(Solution::min_available_duration(s1.clone(), s2.clone(), duration), expect, "s1: {:?}, s2: {:?}, d: {}", s1, s2, duration);
        }
    }
}