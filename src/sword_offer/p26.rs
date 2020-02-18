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
    pub fn is_sub_structure(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if b.is_none() {
            return true;
        }
        if a.is_none() {
            return false;
        }
        let mut ans = false;
        Self::visit(&a, &b, &mut ans);
        ans
    }
    fn visit(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>, ans: &mut bool) {
        if *ans || a.is_none() {
            return;
        }
        if Self::is_sub(b, a) {
            *ans = true;
            return;
        }
        let ar = a.as_ref().unwrap().borrow();
        Self::visit(&ar.left, b, ans);
        if *ans {
            Self::visit(&ar.right, b, ans);
        }
    }
    fn is_sub(b: &Option<Rc<RefCell<TreeNode>>>, a: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if b.is_none() {
            return true;
        }
        if a.is_none() {
            return false;
        }
        let br = b.as_ref().unwrap().borrow();
        let ar = a.as_ref().unwrap().borrow();
        br.val == ar.val && Self::is_sub(&br.left, &ar.left) && Self::is_sub(&br.right, &ar.right)
    }
}

struct Solution;