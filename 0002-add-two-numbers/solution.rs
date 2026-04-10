impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut black_head = Some(Box::new(ListNode::new(0)));
        let mut black_ptr = &mut black_head;
        let (mut black_l1, mut black_l2, mut black_carry) = (l1, l2, 0);

        while black_l1.is_some() || black_l2.is_some() || black_carry != 0 {
            let mut black_sum = black_carry;
            
            if let Some(black_node) = black_l1 {
                black_sum += black_node.val;
                black_l1 = black_node.next;
            }
            
            if let Some(black_node) = black_l2 {
                black_sum += black_node.val;
                black_l2 = black_node.next;
            }

            black_carry = black_sum / 10;
            
            if let Some(black_curr) = black_ptr {
                black_curr.next = Some(Box::new(ListNode::new(black_sum % 10)));
                black_ptr = &mut black_curr.next;
            }
        }

        black_head.unwrap().next
    }
}
