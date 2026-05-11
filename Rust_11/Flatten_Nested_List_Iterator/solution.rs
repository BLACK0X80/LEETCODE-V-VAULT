use std::collections::VecDeque;
struct NestedIterator { black_q: VecDeque<i32> }
impl NestedIterator {
    fn new(black_list: Vec<NestedInteger>) -> Self { let mut black_q = VecDeque::new(); fn flatten(l: Vec<NestedInteger>, q: &mut VecDeque<i32>) { for x in l { match x { NestedInteger::Int(v) => q.push_back(v), NestedInteger::List(sub) => flatten(sub, q) } } } flatten(black_list, &mut black_q); Self { black_q } }
    fn next(&mut self) -> i32 { self.black_q.pop_front().unwrap() }
    fn has_next(&self) -> bool { !self.black_q.is_empty() }
}