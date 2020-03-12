impl Solution {
    pub fn verify_preorder(preorder: Vec<i32>) -> bool {
        if preorder.len() <= 1 {
            return true;
        }
        let mut q = vec![];
        let mut lower_bound = std::i32::MIN;
        for v in preorder{
            if v < lower_bound {
                return false;
            }
            while let Some(&top) = q.last() {
                if v > top {
                    q.pop();
                    lower_bound = top;
                } else {
                    break;
                }
            }
            q.push(v);
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_verify_preorder() {
        let test_cases = vec![
            (vec![5,2,6,1,3], false),
            (vec![5,2,1,3,6], true),
            (vec![5,8,7,6,10,9,11], true),
            (vec![5,8,7,6,10,9,4], false),
        ];
        for (preorder, ok) in test_cases {
            assert_eq!(Solution::verify_preorder(preorder.clone()),ok,"preorder: {:?}", preorder);
        }
    }
}