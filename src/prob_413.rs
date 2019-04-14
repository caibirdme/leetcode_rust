impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        let mut total = 0;
        let n = a.len();
        if n < 3 {
            return 0;
        }
        let mut start = 0;
        let n_minus_two = n-2;
        while start < n_minus_two {
            let d = a[start+1]-a[start];
            let mut i = start+2;
            if a[i]-a[i-1] != d {
                start+=1;
                continue;
            }
            while i < n && a[i]-a[i-1] == d {
                i+=1;
            }
            total += Self::calc(i-start);
            start = i-1;
        }
        total
    }
    fn calc(n: usize) -> i32 {
        let t = n-2;
        ((t+1)*t/2) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_number_of_arithmetic_slices() {
        let test_cases = vec![
            (vec![1,2,3,4], 3),
            (vec![1,1,2,5,7], 0),
            (vec![1,3,5,7,9], 6),
            (vec![7,7,7,7], 3),
            (vec![3,-1,-5,-9], 3),
            (vec![1,0,-1,0,1,2,3,4], 11),
        ];
        for (a,expect) in test_cases {
            assert_eq!(expect, Solution::number_of_arithmetic_slices(a.clone()), "a: {:?}", a);
        }
    }
}