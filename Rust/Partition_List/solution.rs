impl Solution {
    pub fn partition(mut black_h: Option<Box<ListNode>>, black_x: i32) -> Option<Box<ListNode>> {
        let (mut black_l1, mut black_l2) = (Box::new(ListNode::new(0)), Box::new(ListNode::new(0)));
        let (mut black_p1, mut black_p2) = (&mut black_l1, &mut black_l2);
        while let Some(mut black_node) = black_h {
            black_h = black_node.next.take();
            if black_node.val < black_x { black_p1.next = Some(black_node); black_p1 = black_p1.next.as_mut().unwrap(); }
            else { black_p2.next = Some(black_node); black_p2 = black_p2.next.as_mut().unwrap(); }
        }
        black_p1.next = black_l2.next;
        black_l1.next
    }
}