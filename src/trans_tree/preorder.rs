use crate::trans_tree::tree::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

fn traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }
    let mut ans = vec![];
    let mut q = vec![Rc::clone(root.as_ref().unwrap())];
    while let Some(node) = q.pop() {
        let br = node.borrow();
        ans.push(br.val);
        if br.right.is_some() {
            q.push(Rc::clone(br.right.as_ref().unwrap()));
        }
        if br.left.is_some() {
            q.push( Rc::clone(br.left.as_ref().unwrap()));
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
        assert_eq!(traverse(r), vec![8,9,3,4,7,10,2,5]);
    }
}