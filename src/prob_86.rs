// Definition for singly-linked list.
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
impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut before = vec![];
        let mut after = vec![];
        while let Some(v) = head.take() {
            if v.val < x {
                before.push(v.val);
            } else {
                after.push(v.val);
            }
            head = v.next;
        }
        let mut pHead = None;
        for v in after.into_iter().rev() {
            let t = Some(Box::new(ListNode{val: v, next: pHead.take()}));
            pHead = t;
        }
        for v in before.into_iter().rev() {
            let t = Some(Box::new(ListNode{val: v, next: pHead.take()}));
            pHead = t;
        }
        pHead
    }
}

struct Solution;