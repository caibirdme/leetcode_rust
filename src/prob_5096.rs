impl Solution {
    pub fn transform_array(arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        if n <= 2 {
            return arr;
        }
        let mut darr = [arr, vec![0; n]];
        darr[1][0] = darr[0][0];
        darr[1][n-1] = darr[0][n-1];
        let mut pos = 0;
        loop {
            let mut change = false;
            let np = pos ^ 1;
            for i in 1..n-1 {
                if darr[pos][i] > darr[pos][i-1] && darr[pos][i] > darr[pos][i+1] {
                    change = true;
                    darr[np][i] = darr[pos][i]-1;
                } else if darr[pos][i] < darr[pos][i-1] && darr[pos][i] < darr[pos][i+1] {
                    change = true;
                    darr[np][i] = darr[pos][i]+1;
                } else {
                    darr[np][i] = darr[pos][i];
                }
            }
            if !change {
                break;
            }
            pos = np;
        }
        darr[pos].clone()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_transform_array() {
        let test_cases = vec![
            (vec![6,2,3,4], vec![6,3,3,4]),
            (vec![1,6,3,4,3,5], vec![1,4,4,4,4,5]),
        ];
        for (arr, expect) in test_cases {
            assert_eq!(Solution::transform_array(arr.clone()), expect, "arr: {:?}", arr);
        }
    }
}