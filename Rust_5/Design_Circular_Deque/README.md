# Design Circular Deque

**Difficulty:** Medium
**Tags:** Array, Linked List, Design, Queue

---

## Problem

<p>Design your implementation of the circular double-ended queue (deque).</p>

<p>Implement the <code>MyCircularDeque</code> class:</p>

<ul>
	<li><code>MyCircularDeque(int k)</code> Initializes the deque with a maximum size of <code>k</code>.</li>
	<li><code>boolean insertFront()</code> Adds an item at the front of Deque. Returns <code>true</code> if the operation is successful, or <code>false</code> otherwise.</li>
	<li><code>boolean insertLast()</code> Adds an item at the rear of Deque. Returns <code>true</code> if the operation is successful, or <code>false</code> otherwise.</li>
	<li><code>boolean deleteFront()</code> Deletes an item from the front of Deque. Returns <code>true</code> if the operation is successful, or <code>false</code> otherwise.</li>
	<li><code>boolean deleteLast()</code> Deletes an item from the rear of Deque. Returns <code>true</code> if the operation is successful, or <code>false</code> otherwise.</li>
	<li><code>int getFront()</code> Returns the front item from the Deque. Returns <code>-1</code> if the deque is empty.</li>
	<li><code>int getRear()</code> Returns the last item from Deque. Returns <code>-1</code> if the deque is empty.</li>
	<li><code>boolean isEmpty()</code> Returns <code>true</code> if the deque is empty, or <code>false</code> otherwise.</li>
	<li><code>boolean isFull()</code> Returns <code>true</code> if the deque is full, or <code>false</code> otherwise.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;MyCircularDeque&quot;, &quot;insertLast&quot;, &quot;insertLast&quot;, &quot;insertFront&quot;, &quot;insertFront&quot;, &quot;getRear&quot;, &quot;isFull&quot;, &quot;deleteLast&quot;, &quot;insertFront&quot;, &quot;getFront&quot;]
[[3], [1], [2], [3], [4], [], [], [], [4], []]
<strong>Output</strong>
[null, true, true, true, false, 2, true, true, true, 4]

<strong>Explanation</strong>
MyCircularDeque myCircularDeque = new MyCircularDeque(3);
myCircularDeque.insertLast(1);  // return True
myCircularDeque.insertLast(2);  // return True
myCircularDeque.insertFront(3); // return True
myCircularDeque.insertFront(4); // return False, the queue is full.
myCircularDeque.getRear();      // return 2
myCircularDeque.isFull();       // return True
myCircularDeque.deleteLast();   // return True
myCircularDeque.insertFront(4); // return True
myCircularDeque.getFront();     // return 4
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= 1000</code></li>
	<li><code>0 &lt;= value &lt;= 1000</code></li>
	<li>At most <code>2000</code> calls will be made to <code>insertFront</code>, <code>insertLast</code>, <code>deleteFront</code>, <code>deleteLast</code>, <code>getFront</code>, <code>getRear</code>, <code>isEmpty</code>, <code>isFull</code>.</li>
</ul>



## Solution

```rust
struct MyCircularDeque { black_v: Vec<i32>, black_h: usize, black_t: usize, black_c: usize, black_k: usize }
impl MyCircularDeque {
    fn new(k: i32) -> Self { Self { black_v: vec![0; k as usize], black_h: 0, black_t: 0, black_c: 0, black_k: k as usize } }
    fn insert_front(&mut self, value: i32) -> bool { if self.is_full() { false } else { self.black_h = (self.black_h + self.black_k - 1) % self.black_k; self.black_v[self.black_h] = value; self.black_c += 1; if self.black_c == 1 { self.black_t = (self.black_h + 1) % self.black_k; } true } }
    fn insert_last(&mut self, value: i32) -> bool { if self.is_full() { false } else { self.black_v[self.black_t] = value; self.black_t = (self.black_t + 1) % self.black_k; self.black_c += 1; true } }
    fn delete_front(&mut self) -> bool { if self.is_empty() { false } else { self.black_h = (self.black_h + 1) % self.black_k; self.black_c -= 1; true } }
    fn delete_last(&mut self) -> bool { if self.is_empty() { false } else { self.black_t = (self.black_t + self.black_k - 1) % self.black_k; self.black_c -= 1; true } }
    fn get_front(&self) -> i32 { if self.is_empty() { -1 } else { self.black_v[self.black_h] } }
    fn get_rear(&self) -> i32 { if self.is_empty() { -1 } else { self.black_v[(self.black_t + self.black_k - 1) % self.black_k] } }
    fn is_empty(&self) -> bool { self.black_c == 0 }
    fn is_full(&self) -> bool { self.black_c == self.black_k }
}
```