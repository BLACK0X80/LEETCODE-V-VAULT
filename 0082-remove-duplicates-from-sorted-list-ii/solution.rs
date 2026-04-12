impl Solution {
    pub fn delete_duplicates(black_h: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut black_dummy = Some(Box::new(ListNode { val: 0, next: black_h }));
        let mut black_curr = black_dummy.as_mut();
        while let Some(black_p) = black_curr {
            if let Some(mut black_n) = black_p.next.take() {
                if black_n.next.as_ref().map_or(false, |next| next.val == black_n.val) {
                    let black_v = black_n.val;
                    while black_n.next.as_ref().map_or(false, |next| next.val == black_v) { black_n = black_n.next.unwrap(); }
                    black_p.next = black_n.next;
                    black_curr = Some(black_p); 
                } else {
                    black_p.next = Some(black_n);
                    black_curr = black_p.next.as_mut();
                }
            } else { break; }
        }
        black_dummy.unwrap().next
    }
}
