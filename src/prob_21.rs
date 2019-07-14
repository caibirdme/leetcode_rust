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
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ans = vec![];
        while l1.is_some() && l2.is_some() {
            let v1 = l1.as_ref().unwrap().val;
            let v2 = l2.as_ref().unwrap().val;
            if v1 < v2 {
                ans.push(v1);
                l1 = l1.unwrap().next;
            } else {
                ans.push(v2);
                l2 = l2.unwrap().next;
            }
        }
        let mut rest = if l1.is_none() { l2 } else { l1 };
        while let Some(v) = rest.take() {
            ans.push(v.val);
            rest = v.next;
        }
        let mut t = None;
        for &e in ans.iter().rev() {
            let q = Box::new(ListNode{val:e,next: t.take()});
            t = Some(q);
        }
        t
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        let test_cases = vec![
            (vec![1,2,4], vec![1,3,4], vec![1,1,2,3,4,4]),
        ];
        for (l1, l2, expect) in test_cases {
            assert_eq!(Solution::merge_two_lists(build(&l1), build(&l2)), build(&expect), "l1: {:?}, l2: {:?}", l1, l2);
        }
    }
    fn build(v: &Vec<i32>) -> Option<Box<ListNode>> {
        let mut t = None;
        for &e in v.iter().rev() {
            let q = Box::new(ListNode{val:e,next: t.take()});
            t = Some(q);
        }
        t
    }
}