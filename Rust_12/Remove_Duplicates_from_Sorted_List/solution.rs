impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        while let Some(node) = cur {
            while node.next.as_ref().map_or(false, |n| n.val == node.val) {
                node.next = node.next.take().unwrap().next;
            }
            cur = &mut cur.as_mut().unwrap().next;
        }
        head
    }
}