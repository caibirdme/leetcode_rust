impl Solution {
    pub fn missing_number(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let d0 = arr[1]-arr[0];
        let d1 = arr[2]-arr[1];
        if d0 == 0 && d1 == 0 {
            return arr[0];
        }
        if d0 == d1 {
            for i in 3..n {
                let next = arr[i-1]+d0;
                if arr[i] != next {
                    return next;
                }
            }
        } else if d0 > 0 {
            if d0 > d1 {
                return arr[0]+d1;
            } else {
                return arr[1]+d0;
            }
        } else {
            if d0 > d1 {
                return arr[1]+d0;
            } else {
                return arr[0]+d1;
            }
        }
        unreachable!()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_missing_number() {
        let test_cases = vec![
            (vec![0,0,0,0], 0),
            (vec![5,7,11,13], 9),
            (vec![15,13,12], 14),
            (vec![-10, -14, -22], -18),
            (vec![-10, -18, -22], -14),
            (vec![1,2,3,5], 4),
        ];
        for (arr, expect) in test_cases {
            assert_eq!(Solution::missing_number(arr.clone()), expect, "arr: {:?}", arr);
        }
    }
}