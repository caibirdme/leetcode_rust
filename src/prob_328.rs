// Definition for singly-linked list.
//#[derive(PartialEq, Eq, Debug)]
//pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
//}
//
//impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
//}

use crate::linkedlist::ListNode;


impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let mut odd_head = head.as_mut().unwrap().next.take();
        let mut even_ptr = &mut head;
        let mut odd_ptr = &mut odd_head;
        while odd_ptr.is_some() && odd_ptr.as_ref().unwrap().next.is_some() {
            let mut next = odd_ptr.as_mut().unwrap().next.take();
            let mut next_next = next.as_mut().unwrap().next.take();
            even_ptr.as_mut().unwrap().next = next;
            odd_ptr.as_mut().unwrap().next = next_next;
            even_ptr = &mut even_ptr.as_mut().unwrap().next;
            odd_ptr = &mut odd_ptr.as_mut().unwrap().next;
        }
        match even_ptr {
            Some(v) => {
                v.next = odd_head;
            },
            None => {
                *even_ptr = odd_head;
            }
        }
        head
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::linkedlist::ListNode;
    use super::Solution;
    #[test]
    fn test_odd_even_list() {
        let test_cases = vec![
            (vec![1], vec![1]),
            (vec![1,2], vec![1,2]),
            (vec![1,2,3], vec![1,3,2]),
            (vec![1,2,3,4], vec![1,3,2,4]),
            (vec![1,2,3,4,5], vec![1,3,5,2,4]),
            (vec![1,2,3,4,5,6], vec![1,3,5,2,4,6]),
        ];
        for (arr, expect) in test_cases {
            assert_eq!(
                Solution::odd_even_list(ListNode::construct(&arr)),
                ListNode::construct(&expect)
            );
        }
    }
}