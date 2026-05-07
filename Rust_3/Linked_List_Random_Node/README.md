# Linked List Random Node

**Difficulty:** Medium
**Tags:** Linked List, Math, Reservoir Sampling, Randomized

---

## Problem

<p>Given a singly linked list, return a random node&#39;s value from the linked list. Each node must have the <strong>same probability</strong> of being chosen.</p>

<p>Implement the <code>Solution</code> class:</p>

<ul>
	<li><code>Solution(ListNode head)</code> Initializes the object with the head of the singly-linked list <code>head</code>.</li>
	<li><code>int getRandom()</code> Chooses a node randomly from the list and returns its value. All the nodes of the list should be equally likely to be chosen.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/03/16/getrand-linked-list.jpg" style="width: 302px; height: 62px;" />
<pre>
<strong>Input</strong>
[&quot;Solution&quot;, &quot;getRandom&quot;, &quot;getRandom&quot;, &quot;getRandom&quot;, &quot;getRandom&quot;, &quot;getRandom&quot;]
[[[1, 2, 3]], [], [], [], [], []]
<strong>Output</strong>
[null, 1, 3, 2, 2, 3]

<strong>Explanation</strong>
Solution solution = new Solution([1, 2, 3]);
solution.getRandom(); // return 1
solution.getRandom(); // return 3
solution.getRandom(); // return 2
solution.getRandom(); // return 2
solution.getRandom(); // return 3
// getRandom() should return either 1, 2, or 3 randomly. Each element should have equal probability of returning.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the linked list will be in the range <code>[1, 10<sup>4</sup>]</code>.</li>
	<li><code>-10<sup>4</sup> &lt;= Node.val &lt;= 10<sup>4</sup></code></li>
	<li>At most <code>10<sup>4</sup></code> calls will be made to <code>getRandom</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow up:</strong></p>

<ul>
	<li>What if the linked list is extremely large and its length is unknown to you?</li>
	<li>Could you solve this efficiently without using extra space?</li>
</ul>



## Solution

```rust
struct Solution { black_h: Option<Box<ListNode>>, black_s: std::cell::Cell<u64> }
impl Solution {
    fn new(black: Option<Box<ListNode>>) -> Self { Self { black_h: black, black_s: std::cell::Cell::new(12345) } }
    fn get_random(&self) -> i32 { let (mut black_res, mut black_curr, mut black_i) = (0, &self.black_h, 1.0); while let Some(black_node) = black_curr { self.black_s.set(self.black_s.get().wrapping_mul(6364136223846793005).wrapping_add(1)); if (self.black_s.get() as f64 / u64::MAX as f64) < (1.0 / black_i) { black_res = black_node.val; } black_curr = &black_node.next; black_i += 1.0; } black_res }
}
```