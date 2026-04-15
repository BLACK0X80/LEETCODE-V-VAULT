struct Node* copyRandomList(struct Node* head) {
    if (!head) return NULL;
    
    struct Node* cur = head;
    while (cur) {
        struct Node* copy = (struct Node*)malloc(sizeof(struct Node));
        copy->val = cur->val;
        copy->next = cur->next;
        copy->random = NULL;
        cur->next = copy;
        cur = copy->next;
    }
    
    cur = head;
    while (cur) {
        if (cur->random) {
            cur->next->random = cur->random->next;
        }
        cur = cur->next->next;
    }
    
    struct Node* new_head = head->next;
    cur = head;
    while (cur) {
        struct Node* copy = cur->next;
        cur->next = copy->next;
        if (copy->next) {
            copy->next = copy->next->next;
        }
        cur = cur->next;
    }
    
    return new_head;
}