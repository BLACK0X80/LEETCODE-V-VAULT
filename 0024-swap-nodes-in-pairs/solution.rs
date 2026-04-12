impl Solution {
    pub fn swap_pairs(black_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        black_head.and_then(|mut black_n1| match black_n1.next.take() {
            Some(mut black_n2) => {
                black_n1.next = Self::swap_pairs(black_n2.next.take());
                black_n2.next = Some(black_n1);
                Some(black_n2)
            },
            None => Some(black_n1)
        })
    }
}
