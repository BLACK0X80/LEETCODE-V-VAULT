impl Solution {
    pub fn remove_nth_from_end(mut black_head: Option<Box<ListNode>>, black_n: i32) -> Option<Box<ListNode>> {
        let mut black_sz = 0;
        let mut black_curr = &black_head;
        while let Some(node) = black_curr { black_sz += 1; black_curr = &node.next; }
        if black_sz == black_n { return black_head.and_then(|node| node.next); }
        let mut black_curr = black_head.as_mut();
        for _ in 0..black_sz - black_n - 1 { black_curr = black_curr.and_then(|node| node.next.as_mut()); }
        black_curr.map(|node| node.next = node.next.take().and_then(|n| n.next));
        black_head
    }
}