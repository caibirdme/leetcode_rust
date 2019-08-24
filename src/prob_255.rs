impl Solution {
    pub fn verify_preorder(preorder: Vec<i32>) -> bool {
        if preorder.len() <= 1 {
            return true;
        }
        let mut idx = 1;
        Self::dfs(&mut idx, &preorder, None, Some(preorder[0]-1));
        Self::dfs(&mut idx, &preorder, Some(preorder[0]+1), None);
        idx == preorder.len()
    }
    fn dfs(i: &mut usize, v: &Vec<i32>, l: Option<i32>, r: Option<i32>) {
        if *i >= v.len() {
            return;
        }
        let c = v[*i];
        match (l,r) {
            (None, Some(r_bound)) => {
                if c <= r_bound {
                    *i += 1;
                    Self::dfs(i, v, None, Some(c-1));
                    if c+1 <= r_bound {
                        Self::dfs(i,v,Some(c+1), r);
                    }
                }
            },
            (Some(l_bound), None) => {
                if c >= l_bound {
                    *i += 1;
                    if c-1 >= l_bound {
                        Self::dfs(i, v, l, Some(c-1));
                    }
                    Self::dfs(i,v,Some(c+1), None);
                }
            },
            (Some(l_bound), Some(r_bound)) => {
                if c >= l_bound && c <= r_bound {
                    *i += 1;
                    if c-1 >= l_bound {
                        Self::dfs(i,v,l,Some(c-1));
                    }
                    if c+1 <= r_bound {
                        Self::dfs(i,v,Some(c+1), r);
                    }
                }
            },
            _ => return,
        }
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