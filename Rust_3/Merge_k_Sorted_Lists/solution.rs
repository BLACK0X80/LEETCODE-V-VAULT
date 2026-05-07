use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct BlackNode(Box<ListNode>);

impl Ord for BlackNode {
    fn cmp(&self, black_other: &Self) -> Ordering {
        black_other.0.val.cmp(&self.0.val)
    }
}

impl PartialOrd for BlackNode {
    fn partial_cmp(&self, black_other: &Self) -> Option<Ordering> {
        Some(self.cmp(black_other))
    }
}

impl Solution {
    pub fn merge_k_lists(black_lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut black_heap = BinaryHeap::new();
        for black_l in black_lists {
            if let Some(black_node) = black_l {
                black_heap.push(BlackNode(black_node));
            }
        }

        let mut black_dummy = Box::new(ListNode::new(0));
        let mut black_curr = &mut black_dummy;

        while let Some(BlackNode(mut black_node)) = black_heap.pop() {
            if let Some(black_next) = black_node.next.take() {
                black_heap.push(BlackNode(black_next));
            }
            black_curr.next = Some(black_node);
            black_curr = black_curr.next.as_mut().unwrap();
        }

        let bravexuneth = black_dummy.next;
        bravexuneth
    }
}