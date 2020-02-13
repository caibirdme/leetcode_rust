// Definition for a binary tree node.
 #[derive(Debug, PartialEq, Eq)]
 pub struct TreeNode {
   pub val: i32,
   pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
 }

 impl TreeNode {
   #[inline]
   pub fn new(val: i32) -> Self {
     TreeNode {
       val,
       left: None,
       right: None
     }
   }
 }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut hash = HashMap::new();
        Self::post_order(&root, &mut hash);
        let t = *hash.values().max().unwrap();
        let mut ans = vec![];
        hash.iter().for_each(|(&k,&v)| {
            if v == t {
                ans.push(k);
            }
        });
        ans
    }
    fn post_order(root: &Option<Rc<RefCell<TreeNode>>>, count: &mut HashMap<i32, i32>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let br = root.as_ref().unwrap().borrow();
        let (a,b) = (Self::post_order(&br.left, count), Self::post_order(&br.right, count));
        let sum = a+b+br.val;
        *count.entry(sum).or_insert(0) += 1;
        sum
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_foo() {
        assert_eq!(1+1, 2);
    }
}