// Definition for singly-linked list.
use crate::linkedlist::ListNode;
use std::mem;

pub struct Solution;

impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        while cur.is_some() {
            match cur.as_ref() {
                Some(node) if node.val == val => {
                    let mut move_out = cur.take();
                    mem::swap(cur, &mut move_out.as_mut().unwrap().next);
                    continue;
                }
                _ => {}
            }
            cur = &mut cur.as_mut().unwrap().next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::linkedlist::ListNode;
    #[test]
    fn test_remove_elements() {
        assert_eq!(
            Solution::remove_elements(ListNode::construct(&vec![1,1]), 1),
            ListNode::construct(&vec![])
        );
        assert_eq!(
            Solution::remove_elements(ListNode::construct(&vec![1,2,6,3,4,5,6]), 6),
            ListNode::construct(&vec![1,2,3,4,5])
        );
        assert_eq!(
            Solution::remove_elements(ListNode::construct(&vec![1,2,3]), 3),
            ListNode::construct(&vec![1,2])
        );
        assert_eq!(
            Solution::remove_elements(ListNode::construct(&vec![1,2,1]), 1),
            ListNode::construct(&vec![2])
        );
        assert_eq!(
            Solution::remove_elements(ListNode::construct(&vec![1,2,1]), 2),
            ListNode::construct(&vec![1,1])
        );
    }
}
