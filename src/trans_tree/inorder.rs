use std::rc::Rc;
use std::cell::RefCell;
use crate::trans_tree::tree::TreeNode;

fn traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }
    let mut ans = vec![];
    let mut q = vec![(Rc::clone(root.as_ref().unwrap()), 0)];
    while let Some((node, status)) = q.pop() {
        if status == 0 {
            q.push((Rc::clone(&node), 1));
            if node.borrow().left.is_some() {
                q.push((Rc::clone(node.borrow().left.as_ref().unwrap()), 0));
            }
        } else {
            ans.push(node.borrow().val);
            if node.borrow().right.is_some() {
                q.push((Rc::clone(node.borrow().right.as_ref().unwrap()), 0));
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_traverse() {
        let r = Some(Rc::new(RefCell::new(TreeNode{
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode{
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 7,
                left:Some(Rc::new(RefCell::new(TreeNode{
                    val: 10,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode{
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(traverse(r), vec![3,9,4,8,2,10,7,5]);
    }
}