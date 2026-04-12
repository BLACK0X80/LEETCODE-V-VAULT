impl Solution {
    pub fn rotate_right(mut black_h: Option<Box<ListNode>>, black_k: i32) -> Option<Box<ListNode>> {
        if black_h.is_none() || black_k == 0 { return black_h; }
        let mut black_n = 0;
        let mut black_curr = &black_h;
        while let Some(node) = black_curr { black_n += 1; black_curr = &node.next; }
        let black_k = black_k % black_n;
        if black_k == 0 { return black_h; }
        let mut black_curr = black_h.as_mut().unwrap();
        for _ in 0..black_n - black_k - 1 { black_curr = black_curr.next.as_mut().unwrap(); }
        let mut black_new_h = black_curr.next.take();
        let mut black_tail = black_new_h.as_mut().unwrap();
        while black_tail.next.is_some() { black_tail = black_tail.next.as_mut().unwrap(); }
        black_tail.next = black_h;
        black_new_h
    }
}
