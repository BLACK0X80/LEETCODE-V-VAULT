impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut nodes = Vec::new();
        let mut curr = head;

        while let Some(mut node) = curr {
            curr = node.next.take();
            nodes.push(node);
        }

        let n = nodes.len();
        let k = k as usize;

        for chunk in nodes.chunks_mut(k) {
            if chunk.len() == k {
                chunk.reverse();
            }
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        for mut node in nodes {
            node.next = None;
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}
