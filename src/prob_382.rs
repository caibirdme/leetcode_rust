// #[derive(PartialEq, Eq, Clone, Debug)]
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
struct Solution {
    list: Option<Box<ListNode>>,
    gen: ThreadRng,
}

use rand::{rngs::ThreadRng, thread_rng, Rng};


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self{
            list: head,
            gen: thread_rng(),
        }
    }

    /** Returns a random node's value. */
    fn get_random(&mut self) -> i32 {
        let mut count = 1;
        let mut v = 0;
        let mut p = self.list.as_ref();
        while let Some(node) = p.take() {
            let d = self.gen.gen_range(0, count);
            if d == 0 {
                v = node.val;
            }
            count += 1;
            p = node.next.as_ref();
        }
        v
    }
}