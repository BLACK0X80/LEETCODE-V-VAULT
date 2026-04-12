impl Solution {
    pub fn reverse_between(mut black_h: Option<Box<ListNode>>, black_l: i32, black_r: i32) -> Option<Box<ListNode>> {
        let mut black_dummy = Some(Box::new(ListNode { val: 0, next: black_h }));
        let mut black_pre = black_dummy.as_mut().unwrap();
        for _ in 1..black_l { black_pre = black_pre.next.as_mut().unwrap(); }
        let mut black_curr = black_pre.next.take();
        for _ in 0..black_r - black_l {
            let mut black_next = black_curr.as_mut().unwrap().next.take();
            black_curr.as_mut().unwrap().next = black_next.as_mut().unwrap().next.take();
            black_next.as_mut().unwrap().next = black_pre.next.take();
            black_pre.next = black_next;
        }
        let mut black_tail = black_pre;
        while black_tail.next.is_some() { black_tail = black_tail.next.as_mut().unwrap(); }
        black_tail.next = black_curr;
        black_dummy.unwrap().next
    }
}
