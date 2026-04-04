impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odds = vec![];
        let mut evens = vec![];
        let mut i = 1;
        let mut cur = head;
        while let Some(node) = cur {
            if i % 2 == 1 { odds.push(node.val); } else { evens.push(node.val); }
            cur = node.next;
            i += 1;
        }
        let vals: Vec<i32> = odds.into_iter().chain(evens).collect();
        let mut head = None;
        for &v in vals.iter().rev() {
            let mut node = Box::new(ListNode::new(v));
            node.next = head;
            head = Some(node);
        }
        head
    }
}
