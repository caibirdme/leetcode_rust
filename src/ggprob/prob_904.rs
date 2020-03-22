use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(tree: Vec<i32>) -> i32 {
        let n = tree.len();
        let mut count = HashMap::new();
        let mut j = 0;
        let mut i = 0;
        let mut ans = 0;
        while j < n {
            *count.entry(tree[j]).or_insert(0) += 1;
            j+=1;
            if count.len() > 2 {
                let mut should_del = false;
                if let Some(p) = count.get_mut(&tree[i]) {
                    *p -= 1;
                    if *p == 0 {
                        should_del = true;
                    }
                }
                if should_del {
                    count.remove(&tree[i]);
                }
                i+=1;
            } else {
                ans = ans.max(j-i);
            }
        }
        ans as i32
    }
    pub fn total_fruit_dp(tree: Vec<i32>) -> i32 {
        let n = tree.len();
        let mut i = 0;
        let mut j = i;
        while j < n && tree[j]==tree[i] {j+=1;}
        let mut ans = j-i;
        let mut pp = tree[i];
        i = j;
        if i >= n {
            return ans as i32;
        }
        while j < n && tree[j] == tree[i] {j+=1;}
        ans += j-i;
        let mut p = (tree[i], ans, j-i);
        i = j;
        while i < n {
            let mut j = i;
            while j < n && tree[i] == tree[j] {j+=1;}
            let len = j-i;
            if tree[i] == pp {
                pp = p.0;
                p = (tree[i], p.1+len, len);
                ans = ans.max(p.1);
            } else {
                pp = p.0;
                p = (tree[i], p.2+len, len);
                ans = ans.max(p.1);
            }
            i = j;
        }
        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_total_fruit() {
        let test_cases = vec![
            (vec![0], 1),
            (vec![1,2,3,2,2], 4),
            (vec![3,3,3,1,2,1,1,2,3,3,4], 5),
        ];
        for (tree, expect) in test_cases {
            assert_eq!(Solution::total_fruit_dp(tree.clone()), expect, "tree: {:?}", tree);
        }
    }
}