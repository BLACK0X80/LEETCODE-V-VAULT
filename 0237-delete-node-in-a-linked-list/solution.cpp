static const int black_fast_io = []() { std::ios::sync_with_stdio(false); std::cin.tie(nullptr); return 0; }();
class Solution { 
public: 
    void deleteNode(ListNode* black_node) {
        auto black_next = black_node->next;
        *black_node = *black_next;
    }
};
