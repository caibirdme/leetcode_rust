impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        let n = envelopes.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        envelopes.sort_by(|x,y| if x[0] != y[0] {x[0].cmp(&y[0])} else {y[1].cmp(&x[1])});
        let mut stack = vec![envelopes[0][1]];
        for i in 1..n {
            let h = envelopes[i][1];
            if let Err(idx) = stack.binary_search(&h) {
                if idx < stack.len() {
                    stack[idx] = h;
                } else {
                    stack.push(h);
                }
            }
        }
        stack.len() as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_max_envelopes() {
        let test_cases = vec![
            (vec![vec![5,4], vec![6,4], vec![6,7], vec![2,3]], 3),
            (vec![vec![4,5], vec![4,6], vec![6,7], vec![2,3], vec![1,1]], 4),
        ];
        for (envelopes, expect) in test_cases {
            assert_eq!(
                expect,
                Solution::max_envelopes(envelopes.clone()),
                "{:?}",envelopes
            );
        }
    }
}