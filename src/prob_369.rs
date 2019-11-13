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
    pub fn plus_one(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut list = Self::reverse(head);
        let mut cur = list.as_mut();
        while let Some(v) = cur.take() {
            v.val += 1;
            if v.val < 10 {
                break;
            }
            v.val = 0;
            if v.next.is_none() {
                v.next = Some(Box::new(ListNode::new(0)));
            }
            cur = v.next.as_mut();
        }
        Self::reverse(list)
    }
    fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = None;
        while let Some(mut node) = head.take() {
            head = node.next.take();
            node.next = new_head;
            new_head = Some(node);
        }
        new_head
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::prob_369::ListNode;

    fn build_list_node(arr: &Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &v in arr.iter().rev() {
            let node = Some(Box::new(ListNode{
                val: v,
                next: head.take(),
            }));
            head = node;
        }
        head
    }

    #[test]
    fn test_plus_one() {
        let test_cases = vec![
            (vec![1,2,3], vec![1,2,4]),
            (vec![9,9,9], vec![1,0,0,0]),
            (vec![8,9,9], vec![9,0,0]),
            (vec![0], vec![1]),
        ];
        for (a,expect) in test_cases {
            let mut head = build_list_node(&a);
            let expect_head = build_list_node(&expect);
            assert_eq!(Solution::plus_one(head), expect_head, "nums: {:?}", a);
        }
    }
}