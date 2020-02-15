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

 struct Item(Rc<RefCell<TreeNode>>, i32);
 impl Clone for Item {
     fn clone(&self) -> Self {
         Self(self.0.clone(), self.1)
     }
 }
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut q = vec![Item(root.unwrap(), 1)];
        let mut ans = vec![];
        let mut head = 0;
        let mut count = 0;
        let mut total = 0i64;
        let mut pre_dep = 0;
        while head < q.len() {
            let cur = q[head].clone();
            head += 1;
            let (node, dep) = (cur.0, cur.1);
            if dep > pre_dep {
                if pre_dep > 0 {
                    ans.push(total as f64 / count as f64);
                }
                pre_dep = dep;
                total = node.borrow().val as i64;
                count = 1;
            } else {
                total += node.borrow().val as i64;
                count += 1;
            }
            if node.borrow().left.is_some() {
                q.push(Item(node.borrow().left.as_ref().unwrap().clone(), dep+1));
            }
            if node.borrow().right.is_some() {
                q.push(Item(node.borrow().right.as_ref().unwrap().clone(), dep+1));
            }
        }
        ans.push(total as f64/count as f64);
        ans
    }
}

struct Solution;