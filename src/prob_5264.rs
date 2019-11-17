use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

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
struct FindElements {
    set: HashSet<i32>,
}




/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {

    fn new(mut root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        if root.is_none() {
            return Self{
                set: HashSet::new(),
            };
        }
        let mut set = HashSet::new();
        Self::build_tree(&mut set, &root, 0);
        Self{
            set,
        }
    }
    fn build_tree(set: &mut HashSet<i32>, node: &Option<Rc<RefCell<TreeNode>>>, cur: i32) {
        if node.is_none() {
            return;
        }
        let br = node.as_ref().unwrap().borrow();
        if br.left.is_some() {
            let val = 2*cur+1;
            set.insert(val);
            Self::build_tree(set, &br.left, val);
        }
        if br.right.is_some() {
            let val = 2 * cur + 2;
            set.insert(val);
            Self::build_tree(set, &br.right, val);
        }
    }

    fn find(&self, target: i32) -> bool {
        self.set.contains(&target)
    }
}