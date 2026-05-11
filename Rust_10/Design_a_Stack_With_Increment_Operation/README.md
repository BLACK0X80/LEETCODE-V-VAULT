# Design a Stack With Increment Operation

**Difficulty:** Medium
**Tags:** Array, Stack, Design

---

## Problem

<p>Design a stack that supports increment operations on its elements.</p>

<p>Implement the <code>CustomStack</code> class:</p>

<ul>
	<li><code>CustomStack(int maxSize)</code> Initializes the object with <code>maxSize</code> which is the maximum number of elements in the stack.</li>
	<li><code>void push(int x)</code> Adds <code>x</code> to the top of the stack if the stack has not reached the <code>maxSize</code>.</li>
	<li><code>int pop()</code> Pops and returns the top of the stack or <code>-1</code> if the stack is empty.</li>
	<li><code>void inc(int k, int val)</code> Increments the bottom <code>k</code> elements of the stack by <code>val</code>. If there are less than <code>k</code> elements in the stack, increment all the elements in the stack.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;CustomStack&quot;,&quot;push&quot;,&quot;push&quot;,&quot;pop&quot;,&quot;push&quot;,&quot;push&quot;,&quot;push&quot;,&quot;increment&quot;,&quot;increment&quot;,&quot;pop&quot;,&quot;pop&quot;,&quot;pop&quot;,&quot;pop&quot;]
[[3],[1],[2],[],[2],[3],[4],[5,100],[2,100],[],[],[],[]]
<strong>Output</strong>
[null,null,null,2,null,null,null,null,null,103,202,201,-1]
<strong>Explanation</strong>
CustomStack stk = new CustomStack(3); // Stack is Empty []
stk.push(1);                          // stack becomes [1]
stk.push(2);                          // stack becomes [1, 2]
stk.pop();                            // return 2 --&gt; Return top of the stack 2, stack becomes [1]
stk.push(2);                          // stack becomes [1, 2]
stk.push(3);                          // stack becomes [1, 2, 3]
stk.push(4);                          // stack still [1, 2, 3], Do not add another elements as size is 4
stk.increment(5, 100);                // stack becomes [101, 102, 103]
stk.increment(2, 100);                // stack becomes [201, 202, 103]
stk.pop();                            // return 103 --&gt; Return top of the stack 103, stack becomes [201, 202]
stk.pop();                            // return 202 --&gt; Return top of the stack 202, stack becomes [201]
stk.pop();                            // return 201 --&gt; Return top of the stack 201, stack becomes []
stk.pop();                            // return -1 --&gt; Stack is empty return -1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= maxSize, x, k &lt;= 1000</code></li>
	<li><code>0 &lt;= val &lt;= 100</code></li>
	<li>At most <code>1000</code> calls will be made to each method of <code>increment</code>, <code>push</code> and <code>pop</code> each separately.</li>
</ul>


## Hints

1. Use an array to represent the stack. Push will add new integer to the array. Pop removes the last element in the array and increment will add val to the first k elements of the array.
2. This solution run in O(1) per push and pop and O(k) per increment.

## Solution

```rust
struct CustomStack { black_stack: Vec<i32>, black_inc: Vec<i32>, black_size: usize }
impl CustomStack {
    fn new(max_size: i32) -> Self { Self { black_stack: Vec::with_capacity(max_size as usize), black_inc: vec![0; max_size as usize], black_size: max_size as usize } }
    fn push(&mut self, x: i32) { if self.black_stack.len() < self.black_size { self.black_stack.push(x); } }
    fn pop(&mut self) -> i32 { let i = self.black_stack.len() as i32 - 1; if i < 0 { return -1; } if i > 0 { self.black_inc[i as usize - 1] += self.black_inc[i as usize]; } let res = self.black_stack.pop().unwrap() + self.black_inc[i as usize]; self.black_inc[i as usize] = 0; res }
    fn increment(&mut self, k: i32, val: i32) { let i = (k as usize).min(self.black_stack.len()).saturating_sub(1); if !self.black_stack.is_empty() { self.black_inc[i] += val; } }
}
```