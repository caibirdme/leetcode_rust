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
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut mode = -std::i32::MAX-1;
        let mut count = 0;
        let mut pre = None;
        let mut ans = vec![];
        Self::inorder(&root, &mut pre, &mut count, &mut mode, &mut ans);
        ans
    }
    fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, pre: &mut Option<i32>, count: &mut i32, mode: &mut i32, ans: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let br = root.as_ref().unwrap().borrow();
        Self::inorder(&br.left, pre, count, mode, ans);
        if let Some(v) = pre {
            if *v == br.val {
                *count += 1;
                if *count > *mode {
                    ans.clear();
                    ans.push(*v);
                    *mode = *count;
                } else if *count == *mode {
                    ans.push(*v);
                }
            } else {
                *v = br.val;
                *count = 1;
                if *mode == 1 {
                    ans.push(br.val);
                }
            }
        } else {
            *pre = Some(br.val);
            *count = 1;
            ans.push(br.val);
            *mode = 1;
        }
        Self::inorder(&br.right, pre, count, mode, ans);
    }
}

struct Solution;