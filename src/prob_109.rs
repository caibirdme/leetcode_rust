 #[derive(PartialEq, Eq, Clone, Debug)]
 pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }

 impl ListNode {
   #[inline]
   fn new(val: i32) -> Self {
     ListNode {
       next: None,
       val
     }
   }
 }
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
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if head.is_none() {
            return None;
        }
        let mut p = head.as_ref();
        let mut n = 0;
        while let Some(node) = p.take() {
            n+=1;
            p = node.next.as_ref();
        }
        Self::build(&mut head, 0, n-1)
    }
    fn build(head: &mut Option<Box<ListNode>>, l: usize, r: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if l == r {
            let node = TreeNode::new(head.as_ref().unwrap().val);
            *head = head.take().unwrap().next;
            return Some(Rc::new(RefCell::new(node)));
        }
        let mid = (l+r) / 2;
        let lch = {
            if mid > l {
                Self::build(head, l, mid-1)
            } else {
                None
            }
        };
        let mut root = TreeNode::new(head.as_ref().unwrap().val);
        *head = head.take().unwrap().next;
        root.left = lch;
        root.right = Self::build(head, mid+1, r);
        Some(Rc::new(RefCell::new(root)))
    }
}

 struct Solution;

 #[cfg(test)]
 mod tests {
     use super::*;

     #[test]
     fn test_sorted_list_to_bst() {
             let mut l = Some(Box::new(ListNode{
                 val: -10,
                 next: Some(Box::new(ListNode{
                     val: -3,
                     next: Some(Box::new(ListNode{
                         val: 0,
                         next: Some(Box::new(ListNode{
                             val: 5,
                             next: Some(Box::new(ListNode{
                                 val: 9,
                                 next: None,
                             })),
                         })),
                     })),
                 })),
             }));
         Solution::sorted_list_to_bst(l);
         assert_eq!(1, 1);
     }
 }