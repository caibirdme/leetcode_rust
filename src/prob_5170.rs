impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut count = vec![0; n as usize];
        for (&lch, &rch) in left_child.iter().zip(right_child.iter()) {
            if lch != -1 {
                count[lch as usize] += 1;
                if count[lch as usize] > 1 {
                    return false;
                }
            }
            if rch != -1 {
                count[rch as usize] += 1;
                if count[rch as usize] > 1 {
                    return false;
                }
            }
        }
        let mut root = None;
        for i in 0..n as usize {
            if count[i] == 0 {
                if root.is_none() {
                    root = Some(i);
                } else {
                    return false;
                }
            }
        }
        root.is_some()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_validate_binary_tree_nodes() {
        let test_cases = vec![
            (4, vec![1,-1,3,-1], vec![2,-1,-1,-1], true),
        ];
        for (n, left, right, ok) in test_cases {
            assert_eq!(Solution::validate_binary_tree_nodes(n, left.clone(), right.clone()), ok, "n:{}, left: {:?}, right: {:?}", n,left,right);
        }
    }
}