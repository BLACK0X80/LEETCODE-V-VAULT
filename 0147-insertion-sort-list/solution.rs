impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = head;
        while let Some(mut node) = cur {
            cur = node.next.take();
            let mut prev = &mut dummy;
            while prev.next.as_ref().map_or(false, |n| n.val < node.val) {
                prev = prev.next.as_mut().unwrap();
            }
            node.next = prev.next.take();
            prev.next = Some(node);
        }
        dummy.next
    }
}
