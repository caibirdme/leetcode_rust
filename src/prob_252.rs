use std::cmp::Ordering;

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        if intervals.is_empty() || intervals.len() == 1 {
            return true;
        }
        intervals.sort_by(|a,b| a[0].cmp(&b[0]));
        for i in 1..intervals.len() {
            if intervals[i][0] < intervals[i-1][1] {
                return false;
            }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_can_attend_meetings() {
        let test_cases = vec![
            (vec![vec![0,30],vec![5,10],vec![15,20]], false),
            (vec![vec![7,10],vec![2,4]], true),
        ];
        for (mut intervals, ok) in test_cases {
            assert_eq!(Solution::can_attend_meetings(intervals.clone()), ok, "intervals: {:?}", intervals);
        }
    }
}