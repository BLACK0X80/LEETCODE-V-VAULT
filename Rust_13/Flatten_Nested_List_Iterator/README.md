# Flatten Nested List Iterator

**Difficulty:** Medium
**Tags:** Stack, Tree, Depth-First Search, Design, Queue, Iterator

---

## Problem

<p>You are given a nested list of integers <code>nestedList</code>. Each element is either an integer or a list whose elements may also be integers or other lists. Implement an iterator to flatten it.</p>

<p>Implement the <code>NestedIterator</code> class:</p>

<ul>
	<li><code>NestedIterator(List&lt;NestedInteger&gt; nestedList)</code> Initializes the iterator with the nested list <code>nestedList</code>.</li>
	<li><code>int next()</code> Returns the next integer in the nested list.</li>
	<li><code>boolean hasNext()</code> Returns <code>true</code> if there are still some integers in the nested list and <code>false</code> otherwise.</li>
</ul>

<p>Your code will be tested with the following pseudocode:</p>

<pre>
initialize iterator with nestedList
res = []
while iterator.hasNext()
    append iterator.next() to the end of res
return res
</pre>

<p>If <code>res</code> matches the expected flattened list, then your code will be judged as correct.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nestedList = [[1,1],2,[1,1]]
<strong>Output:</strong> [1,1,2,1,1]
<strong>Explanation:</strong> By calling next repeatedly until hasNext returns false, the order of elements returned by next should be: [1,1,2,1,1].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nestedList = [1,[4,[6]]]
<strong>Output:</strong> [1,4,6]
<strong>Explanation:</strong> By calling next repeatedly until hasNext returns false, the order of elements returned by next should be: [1,4,6].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nestedList.length &lt;= 500</code></li>
	<li>The values of the integers in the nested list is in the range <code>[-10<sup>6</sup>, 10<sup>6</sup>]</code>.</li>
</ul>



## Solution

```rust
use std::collections::VecDeque;
struct NestedIterator { black_q: VecDeque<i32> }
impl NestedIterator {
    fn new(black_list: Vec<NestedInteger>) -> Self { let mut black_q = VecDeque::new(); fn flatten(l: Vec<NestedInteger>, q: &mut VecDeque<i32>) { for x in l { match x { NestedInteger::Int(v) => q.push_back(v), NestedInteger::List(sub) => flatten(sub, q) } } } flatten(black_list, &mut black_q); Self { black_q } }
    fn next(&mut self) -> i32 { self.black_q.pop_front().unwrap() }
    fn has_next(&self) -> bool { !self.black_q.is_empty() }
}
```