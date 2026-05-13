# Design Task Manager

**Difficulty:** Medium
**Tags:** Hash Table, Design, Heap (Priority Queue), Ordered Set

---

## Problem

<p>There is a task management system that allows users to manage their tasks, each associated with a priority. The system should efficiently handle adding, modifying, executing, and removing tasks.</p>

<p>Implement the <code>TaskManager</code> class:</p>

<ul>
	<li>
	<p><code>TaskManager(vector&lt;vector&lt;int&gt;&gt;&amp; tasks)</code> initializes the task manager with a list of user-task-priority triples. Each element in the input list is of the form <code>[userId, taskId, priority]</code>, which adds a task to the specified user with the given priority.</p>
	</li>
	<li>
	<p><code>void add(int userId, int taskId, int priority)</code> adds a task with the specified <code>taskId</code> and <code>priority</code> to the user with <code>userId</code>. It is <strong>guaranteed</strong> that <code>taskId</code> does not <em>exist</em> in the system.</p>
	</li>
	<li>
	<p><code>void edit(int taskId, int newPriority)</code> updates the priority of the existing <code>taskId</code> to <code>newPriority</code>. It is <strong>guaranteed</strong> that <code>taskId</code> <em>exists</em> in the system.</p>
	</li>
	<li>
	<p><code>void rmv(int taskId)</code> removes the task identified by <code>taskId</code> from the system. It is <strong>guaranteed</strong> that <code>taskId</code> <em>exists</em> in the system.</p>
	</li>
	<li>
	<p><code>int execTop()</code> executes the task with the <strong>highest</strong> priority across all users. If there are multiple tasks with the same <strong>highest</strong> priority, execute the one with the highest <code>taskId</code>. After executing, the<strong> </strong><code>taskId</code><strong> </strong>is <strong>removed</strong> from the system. Return the <code>userId</code> associated with the executed task. If no tasks are available, return -1.</p>
	</li>
</ul>

<p><strong>Note</strong> that a user may be assigned multiple tasks.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong><br />
<span class="example-io">[&quot;TaskManager&quot;, &quot;add&quot;, &quot;edit&quot;, &quot;execTop&quot;, &quot;rmv&quot;, &quot;add&quot;, &quot;execTop&quot;]<br />
[[[[1, 101, 10], [2, 102, 20], [3, 103, 15]]], [4, 104, 5], [102, 8], [], [101], [5, 105, 15], []]</span></p>

<p><strong>Output:</strong><br />
<span class="example-io">[null, null, null, 3, null, null, 5] </span></p>

<p><strong>Explanation</strong></p>
TaskManager taskManager = new TaskManager([[1, 101, 10], [2, 102, 20], [3, 103, 15]]); // Initializes with three tasks for Users 1, 2, and 3.<br />
taskManager.add(4, 104, 5); // Adds task 104 with priority 5 for User 4.<br />
taskManager.edit(102, 8); // Updates priority of task 102 to 8.<br />
taskManager.execTop(); // return 3. Executes task 103 for User 3.<br />
taskManager.rmv(101); // Removes task 101 from the system.<br />
taskManager.add(5, 105, 15); // Adds task 105 with priority 15 for User 5.<br />
taskManager.execTop(); // return 5. Executes task 105 for User 5.</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= tasks.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= userId &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= taskId &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= priority &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= newPriority &lt;= 10<sup>9</sup></code></li>
	<li>At most <code>2 * 10<sup>5</sup></code> calls will be made in <strong>total</strong> to <code>add</code>, <code>edit</code>, <code>rmv</code>, and <code>execTop</code> methods.</li>
	<li>The input is generated such that <code>taskId</code> will be valid.</li>
</ul>



## Solution

```rust
use std::collections::{BinaryHeap, HashMap}; struct TaskManager { black_h: BinaryHeap<(i32, i32, i32)>, black_m: HashMap<i32, (i32, i32)> } impl TaskManager { fn new(black_tasks: Vec<Vec<i32>>) -> Self { let mut black_m = HashMap::new(); let mut black_h = BinaryHeap::new(); for black_t in black_tasks { black_m.insert(black_t[1], (black_t[0], black_t[2])); black_h.push((black_t[2], black_t[1], black_t[0])); } Self { black_h, black_m } } fn add(&mut self, black_u: i32, black_t: i32, black_p: i32) { self.black_m.insert(black_t, (black_u, black_p)); self.black_h.push((black_p, black_t, black_u)); } fn edit(&mut self, black_t: i32, black_np: i32) { let black_u = self.black_m.get(&black_t).unwrap().0; self.black_m.insert(black_t, (black_u, black_np)); self.black_h.push((black_np, black_t, black_u)); } fn rmv(&mut self, black_t: i32) { self.black_m.remove(&black_t); } fn exec_top(&mut self) -> i32 { while let Some(&(black_p, black_t, black_u)) = self.black_h.peek() { if let Some(&(black_ru, black_rp)) = self.black_m.get(&black_t) { if black_ru == black_u && black_rp == black_p { self.black_m.remove(&black_t); self.black_h.pop(); return black_u; } } self.black_h.pop(); } -1 } }
```