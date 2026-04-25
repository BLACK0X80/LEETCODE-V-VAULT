# Snapshot Array

**Difficulty:** Medium
**Tags:** Array, Hash Table, Binary Search, Design

---

## Problem

<p>Implement a SnapshotArray that supports the following interface:</p>

<ul>
	<li><code>SnapshotArray(int length)</code> initializes an array-like data structure with the given length. <strong>Initially, each element equals 0</strong>.</li>
	<li><code>void set(index, val)</code> sets the element at the given <code>index</code> to be equal to <code>val</code>.</li>
	<li><code>int snap()</code> takes a snapshot of the array and returns the <code>snap_id</code>: the total number of times we called <code>snap()</code> minus <code>1</code>.</li>
	<li><code>int get(index, snap_id)</code> returns the value at the given <code>index</code>, at the time we took the snapshot with the given <code>snap_id</code></li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> [&quot;SnapshotArray&quot;,&quot;set&quot;,&quot;snap&quot;,&quot;set&quot;,&quot;get&quot;]
[[3],[0,5],[],[0,6],[0,0]]
<strong>Output:</strong> [null,null,0,null,5]
<strong>Explanation: </strong>
SnapshotArray snapshotArr = new SnapshotArray(3); // set the length to be 3
snapshotArr.set(0,5);  // Set array[0] = 5
snapshotArr.snap();  // Take a snapshot, return snap_id = 0
snapshotArr.set(0,6);
snapshotArr.get(0,0);  // Get the value of array[0] with snap_id = 0, return 5</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= index &lt; length</code></li>
	<li><code>0 &lt;= val &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= snap_id &lt; </code>(the total number of times we call <code>snap()</code>)</li>
	<li>At most <code>5 * 10<sup>4</sup></code> calls will be made to <code>set</code>, <code>snap</code>, and <code>get</code>.</li>
</ul>


## Hints

1. Use a list of lists, adding both the element and the snap_id to each index.

## Solution

```rust
struct SnapshotArray { black_id: i32, black_data: Vec<Vec<(i32, i32)>> }
impl SnapshotArray {
    fn new(length: i32) -> Self { Self { black_id: 0, black_data: vec![vec![(0, 0)]; length as usize] } }
    fn set(&mut self, index: i32, val: i32) { let black_v = &mut self.black_data[index as usize]; if black_v.last().unwrap().0 == self.black_id { black_v.last_mut().unwrap().1 = val; } else { black_v.push((self.black_id, val)); } }
    fn snap(&mut self) -> i32 { self.black_id += 1; self.black_id - 1 }
    fn get(&self, index: i32, snap_id: i32) -> i32 { let black_v = &self.black_data[index as usize]; match black_v.binary_search_by_key(&snap_id, |&(s, _)| s) { Ok(i) => black_v[i].1, Err(i) => black_v[i-1].1 } }
}
```