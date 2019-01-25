// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn construct(list: &[i32]) -> Option<Box<ListNode>> {
        let mut linked_list = linkedList::new();
        let mut v = list.to_vec();
        v.reverse();
        for val in v {
            linked_list.push(val);
        }
        linked_list.head
    }
}

struct linkedList {
    head: Option<Box<ListNode>>,
}

impl linkedList {
    fn new() -> Self {
        linkedList { head: None }
    }
    fn push(&mut self, val: i32) {
        let new_node = Box::new(ListNode{
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
}
