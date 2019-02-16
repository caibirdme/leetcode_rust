// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
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
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }
        let mut count = 0;
        {
            let mut fast = head.as_ref();
            while let Some(node) = fast.take() {
                count += 1;
                fast = node.next.as_ref();
            }
        }
        if count == 1 {
            return true;
        }
        count = if count % 2 == 0 {
            count/2+1
        } else {
            count/2+2
        };
        let mut mid = &mut head;
        for _ in 1..count {
            mid = &mut mid.as_mut().unwrap().next;
        }
        let mut cur = mid.take();
        let mut prev = None;
        while let Some(mut cur_inner) = cur.take() {
            let next = cur_inner.next.take();
            cur_inner.next = prev.take();
            prev = Some(cur_inner);
            cur = next;
        }
        let mut new_head = prev.as_ref();
        let mut old_head = head.as_ref();
        while let Some(n_node) = new_head.take() {
            if let Some(o_node) = old_head.take() {
                if n_node.val != o_node.val {
                    return false;
                }
                old_head = o_node.next.as_ref();
            } else {
                return false;
            }
            new_head = n_node.next.as_ref();
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_palindrome() {
        assert_eq!(
            Solution::is_palindrome(Some(Box::new(ListNode{
                val: 1,
                next: Some(Box::new(ListNode{
                    val: 2,
                    next: Some(Box::new(ListNode{
                        val: 3,
                        next: Some(Box::new(ListNode{
                            val: 2,
                            next: Some(Box::new(ListNode{
                                val: 1,
                                next: None,
                            })),
                        })),
                    })),
                })),
            }))),
            true,
        );
        assert_eq!(
            Solution::is_palindrome(Some(Box::new(ListNode{
                val: 1,
                next: Some(Box::new(ListNode{
                    val: 2,
                    next: Some(Box::new(ListNode{
                        val: 2,
                        next: Some(Box::new(ListNode{
                            val: 1,
                            next: None,
                        })),
                    })),
                })),
            }))),
            true,
        );
        assert_eq!(
            Solution::is_palindrome(Some(Box::new(ListNode{
                val: 1,
                next: Some(Box::new(ListNode{
                    val: 2,
                    next: Some(Box::new(ListNode{
                        val: 3,
                        next: Some(Box::new(ListNode{
                            val: 1,
                            next: None,
                        })),
                    })),
                })),
            }))),
            false,
        );
        assert_eq!(
            Solution::is_palindrome(Some(Box::new(ListNode{
                val: 1,
                next: None,
            }))),
            true,
        );
        assert_eq!(
            Solution::is_palindrome(Some(Box::new(ListNode{
                val: 1,
                next: Some(Box::new(ListNode{
                    val: 1,
                    next: None,
                })),
            }))),
            true,
        );
        assert_eq!(
            Solution::is_palindrome(Some(Box::new(ListNode{
                val: 1,
                next: Some(Box::new(ListNode{
                    val: 2,
                    next: Some(Box::new(ListNode{
                        val: 3,
                        next: Some(Box::new(ListNode{
                            val: 3,
                            next: Some(Box::new(ListNode{
                                val: 2,
                                next: Some(Box::new(ListNode{
                                    val: 1,
                                    next: None,
                                })),
                            })),
                        })),
                    })),
                })),
            }))),
            true,
        );
    }
}