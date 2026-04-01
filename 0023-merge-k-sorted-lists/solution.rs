use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();

        for list in lists {
            let mut node = list;
            while let Some(n) = node {
                heap.push(Reverse(n.val));
                node = n.next;
            }
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;

        while let Some(Reverse(val)) = heap.pop() {
            curr.next = Some(Box::new(ListNode::new(val)));
            curr = curr.next.as_mut().unwrap();
        }

        dummy.next
    }
}
