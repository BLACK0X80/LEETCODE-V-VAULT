class PeekingIterator : public Iterator {
    int black_v; bool black_has;
public:
    PeekingIterator(const vector<int>& nums) : Iterator(nums) { black_has = Iterator::hasNext(); if (black_has) black_v = Iterator::next(); }
    int peek() { return black_v; }
    int next() { int res = black_v; black_has = Iterator::hasNext(); if (black_has) black_v = Iterator::next(); return res; }
    bool hasNext() const { return black_has; }
};