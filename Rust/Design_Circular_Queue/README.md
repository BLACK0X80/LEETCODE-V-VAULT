# Design Circular Queue

**Difficulty:** Medium
**Tags:** Array, Linked List, Design, Queue

---

## Problem

<p>Design your implementation of the circular queue. The circular queue is a linear data structure in which the operations are performed based on FIFO (First In First Out) principle, and the last position is connected back to the first position to make a circle. It is also called &quot;Ring Buffer&quot;.</p>

<p>One of the benefits of the circular queue is that we can make use of the spaces in front of the queue. In a normal queue, once the queue becomes full, we cannot insert the next element even if there is a space in front of the queue. But using the circular queue, we can use the space to store new values.</p>

<p>Implement the <code>MyCircularQueue</code> class:</p>

<ul>
	<li><code>MyCircularQueue(k)</code> Initializes the object with the size of the queue to be <code>k</code>.</li>
	<li><code>int Front()</code> Gets the front item from the queue. If the queue is empty, return <code>-1</code>.</li>
	<li><code>int Rear()</code> Gets the last item from the queue. If the queue is empty, return <code>-1</code>.</li>
	<li><code>boolean enQueue(int value)</code> Inserts an element into the circular queue. Return <code>true</code> if the operation is successful.</li>
	<li><code>boolean deQueue()</code> Deletes an element from the circular queue. Return <code>true</code> if the operation is successful.</li>
	<li><code>boolean isEmpty()</code> Checks whether the circular queue is empty or not.</li>
	<li><code>boolean isFull()</code> Checks whether the circular queue is full or not.</li>
</ul>

<p>You must solve the problem without using the built-in queue data structure in your programming language.&nbsp;</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;MyCircularQueue&quot;, &quot;enQueue&quot;, &quot;enQueue&quot;, &quot;enQueue&quot;, &quot;enQueue&quot;, &quot;Rear&quot;, &quot;isFull&quot;, &quot;deQueue&quot;, &quot;enQueue&quot;, &quot;Rear&quot;]
[[3], [1], [2], [3], [4], [], [], [], [4], []]
<strong>Output</strong>
[null, true, true, true, false, 3, true, true, true, 4]

<strong>Explanation</strong>
MyCircularQueue myCircularQueue = new MyCircularQueue(3);
myCircularQueue.enQueue(1); // return True
myCircularQueue.enQueue(2); // return True
myCircularQueue.enQueue(3); // return True
myCircularQueue.enQueue(4); // return False
myCircularQueue.Rear();     // return 3
myCircularQueue.isFull();   // return True
myCircularQueue.deQueue();  // return True
myCircularQueue.enQueue(4); // return True
myCircularQueue.Rear();     // return 4
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= 1000</code></li>
	<li><code>0 &lt;= value &lt;= 1000</code></li>
	<li>At most <code>3000</code> calls will be made to&nbsp;<code>enQueue</code>, <code>deQueue</code>,&nbsp;<code>Front</code>,&nbsp;<code>Rear</code>,&nbsp;<code>isEmpty</code>, and&nbsp;<code>isFull</code>.</li>
</ul>



## Solution

```rust
struct MyCircularQueue { black_q: Vec<i32>, black_h: usize, black_t: usize, black_size: usize, black_k: usize }
impl MyCircularQueue {
    fn new(k: i32) -> Self { Self { black_q: vec![-1; k as usize], black_h: 0, black_t: 0, black_size: 0, black_k: k as usize } }
    fn en_queue(&mut self, value: i32) -> bool { if self.is_full() { return false; } self.black_q[self.black_t] = value; self.black_t = (self.black_t + 1) % self.black_k; self.black_size += 1; true }
    fn de_queue(&mut self) -> bool { if self.is_empty() { return false; } self.black_h = (self.black_h + 1) % self.black_k; self.black_size -= 1; true }
    fn front(&self) -> i32 { if self.is_empty() { -1 } else { self.black_q[self.black_h] } }
    fn rear(&self) -> i32 { if self.is_empty() { -1 } else { self.black_q[(self.black_t + self.black_k - 1) % self.black_k] } }
    fn is_empty(&self) -> bool { self.black_size == 0 }
    fn is_full(&self) -> bool { self.black_size == self.black_k }
}
```