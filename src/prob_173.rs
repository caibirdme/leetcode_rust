use std::rc::Rc;
use std::cell::RefCell;

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
struct BSTIterator {
    stack: Vec<(Rc<RefCell<TreeNode>>, bool)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        if root.is_none() {
            Self{
                stack: vec![],
            }
        } else {
            Self{
                stack: vec![(root.unwrap(), false)],
            }
        }
    }

    /** @return the next smallest number */
    fn next(&mut self) -> i32 {
        if !self.has_next() {
            return -1;
        }
        let (mut cur,back) = self.stack.pop().unwrap();
        if back {
            if cur.borrow().right.is_some() {
                self.stack.push((Rc::clone(cur.borrow().right.as_ref().unwrap()), false));
            }
            cur.borrow().val
        } else if cur.borrow().left.is_none() {
            self.stack.push((cur, true));
            self.next()
        } else {
            self.stack.push((Rc::clone(&cur), true));
            let lch = Rc::clone(cur.borrow().left.as_ref().unwrap());
            self.stack.push((Rc::clone(&lch), false));
            self.next()
        }
    }

    /** @return whether we have a next smallest number */
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_iterator() {
        let root = Some(Rc::new(RefCell::new(TreeNode{
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 15,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 9,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 20,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let mut obj = BSTIterator::new(root);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 3);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 7);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 9);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 15);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 20);
        assert_eq!(obj.has_next(), false);
    }

    #[test]
    fn test_bst_iterator2() {
        let root = Some(Rc::new(RefCell::new(TreeNode{
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 4,
                left: None,
                right: None,
            }))),
        })));
        let mut obj = BSTIterator::new(root);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 1);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 2);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 3);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 4);
        assert_eq!(obj.has_next(), false);
    }
}