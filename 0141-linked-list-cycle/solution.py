class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        black = head
        fast = head
        while fast and fast.next:
            black = black.next
            fast = fast.next.next
            if black == fast:
                return True
        return False
