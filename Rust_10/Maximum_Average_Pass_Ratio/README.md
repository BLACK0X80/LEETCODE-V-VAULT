# Maximum Average Pass Ratio

**Difficulty:** Medium
**Tags:** Array, Greedy, Heap (Priority Queue)

---

## Problem

<p>There is a school that has classes of students and each class will be having a final exam. You are given a 2D integer array <code>classes</code>, where <code>classes[i] = [pass<sub>i</sub>, total<sub>i</sub>]</code>. You know beforehand that in the <code>i<sup>th</sup></code> class, there are <code>total<sub>i</sub></code> total students, but only <code>pass<sub>i</sub></code> number of students will pass the exam.</p>

<p>You are also given an integer <code>extraStudents</code>. There are another <code>extraStudents</code> brilliant students that are <strong>guaranteed</strong> to pass the exam of any class they are assigned to. You want to assign each of the <code>extraStudents</code> students to a class in a way that <strong>maximizes</strong> the <strong>average</strong> pass ratio across <strong>all</strong> the classes.</p>

<p>The <strong>pass ratio</strong> of a class is equal to the number of students of the class that will pass the exam divided by the total number of students of the class. The <strong>average pass ratio</strong> is the sum of pass ratios of all the classes divided by the number of the classes.</p>

<p>Return <em>the <strong>maximum</strong> possible average pass ratio after assigning the </em><code>extraStudents</code><em> students. </em>Answers within <code>10<sup>-5</sup></code> of the actual answer will be accepted.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> classes = [[1,2],[3,5],[2,2]], <code>extraStudents</code> = 2
<strong>Output:</strong> 0.78333
<strong>Explanation:</strong> You can assign the two extra students to the first class. The average pass ratio will be equal to (3/4 + 3/5 + 2/2) / 3 = 0.78333.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> classes = [[2,4],[3,9],[4,5],[2,10]], <code>extraStudents</code> = 4
<strong>Output:</strong> 0.53485
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= classes.length &lt;= 10<sup>5</sup></code></li>
	<li><code>classes[i].length == 2</code></li>
	<li><code>1 &lt;= pass<sub>i</sub> &lt;= total<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= extraStudents &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Pay attention to how much the pass ratio changes when you add a student to the class. If you keep adding students, what happens to the change in pass ratio? The more students you add to a class, the smaller the change in pass ratio becomes.
2. Since the change in the pass ratio is always decreasing with the more students you add, then the very first student you add to each class is the one that makes the biggest change in the pass ratio.
3. Because each class's pass ratio is weighted equally, it's always optimal to put the student in the class that makes the biggest change among all the other classes.
4. Keep a max heap of the current class sizes and order them by the change in pass ratio. For each extra student, take the top of the heap, update the class size, and put it back in the heap.

## Solution

```rust
use std::collections::BinaryHeap;
#[derive(PartialOrd, PartialEq)] struct Class(f64, i32, i32);
impl Eq for Class {} 
impl Ord for Class { fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal) } }
impl Solution { pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra: i32) -> f64 { let mut black_h = classes.iter().map(|c| Class((c[0]+1) as f64/(c[1]+1) as f64 - c[0] as f64/c[1] as f64, c[0], c[1])).collect::<BinaryHeap<_>>(); for _ in 0..extra { if let Some(Class(_, p, t)) = black_h.pop() { black_h.push(Class((p+2) as f64/(t+2) as f64 - (p+1) as f64/(t+1) as f64, p+1, t+1)); } } black_h.iter().map(|&Class(_, p, t)| p as f64 / t as f64).sum::<f64>() / classes.len() as f64 } }
```