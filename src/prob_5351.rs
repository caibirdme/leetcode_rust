
impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        let n = slices.len();
        if n == 3 {
            return *slices.iter().max().unwrap();
        }
        let k = n / 3;
        let mut f = vec![vec![0; k+1]; n];
        f[0][1] = slices[0];
        f[1][1] = slices[0].max(slices[1]);
        for i in 2..n-1 {
            for j in 1..=k {
                f[i][j] = f[i-1][j].max(f[i-2][j-1]+slices[i]);
            }
        }
        let mut ans = f[n-2][k];
        for i in 0..n {
            for j in 0..=k {
                f[i][j] = 0;
            }
        }
        f[1][1] = slices[1];
        for i in 2..n {
            for j in 1..=k {
                f[i][j] = f[i-1][j].max(f[i-2][j-1]+slices[i]);
            }
        }
        ans.max(f[n-1][k])
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_size_slices() {
        let test_cases = vec![
            (vec![78,89,84,23,34,21,71,94,9,12,3,73,20,54,76,23,41,19,22,41,68,63,24,41,7,76,92,58,55,64,81,4,67,53,68,100,47,83,25,84,80,92,40,93,27,80,71,4,41,19,22,25,12,78,38,79,54,89,89,82,15,45,92,28,24,17,13,63,7,70,40,67,95,39,84,98,74,80,26,22,83,66,58,59,6,65,100,33,2,65,54,46,69], 2479),
            (vec![8,9,8,6,1,1], 16),
            (vec![3,1,2], 3),
            (vec![4,1,2,5,8,3,1,9,7], 21),
        ];
        for (slices, expect) in test_cases {
            assert_eq!(Solution::max_size_slices(slices.clone()), expect, "slice: {:?}", slices);
        }
    }
}