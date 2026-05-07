# Count Integers in Intervals

**Difficulty:** Hard
**Tags:** Design, Segment Tree, Ordered Set

---

## Problem

<p>Given an <strong>empty</strong> set of intervals, implement a data structure that can:</p>

<ul>
	<li><strong>Add</strong> an interval to the set of intervals.</li>
	<li><strong>Count</strong> the number of integers that are present in <strong>at least one</strong> interval.</li>
</ul>

<p>Implement the <code>CountIntervals</code> class:</p>

<ul>
	<li><code>CountIntervals()</code> Initializes the object with an empty set of intervals.</li>
	<li><code>void add(int left, int right)</code> Adds the interval <code>[left, right]</code> to the set of intervals.</li>
	<li><code>int count()</code> Returns the number of integers that are present in <strong>at least one</strong> interval.</li>
</ul>

<p><strong>Note</strong> that an interval <code>[left, right]</code> denotes all the integers <code>x</code> where <code>left &lt;= x &lt;= right</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;CountIntervals&quot;, &quot;add&quot;, &quot;add&quot;, &quot;count&quot;, &quot;add&quot;, &quot;count&quot;]
[[], [2, 3], [7, 10], [], [5, 8], []]
<strong>Output</strong>
[null, null, null, 6, null, 8]

<strong>Explanation</strong>
CountIntervals countIntervals = new CountIntervals(); // initialize the object with an empty set of intervals. 
countIntervals.add(2, 3);  // add [2, 3] to the set of intervals.
countIntervals.add(7, 10); // add [7, 10] to the set of intervals.
countIntervals.count();    // return 6
                           // the integers 2 and 3 are present in the interval [2, 3].
                           // the integers 7, 8, 9, and 10 are present in the interval [7, 10].
countIntervals.add(5, 8);  // add [5, 8] to the set of intervals.
countIntervals.count();    // return 8
                           // the integers 2 and 3 are present in the interval [2, 3].
                           // the integers 5 and 6 are present in the interval [5, 8].
                           // the integers 7 and 8 are present in the intervals [5, 8] and [7, 10].
                           // the integers 9 and 10 are present in the interval [7, 10].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= left &lt;= right &lt;= 10<sup>9</sup></code></li>
	<li>At most <code>10<sup>5</sup></code> calls <strong>in total</strong> will be made to <code>add</code> and <code>count</code>.</li>
	<li>At least <strong>one</strong> call will be made to <code>count</code>.</li>
</ul>


## Hints

1. How can you efficiently add intervals to the set of intervals? Can a data structure like a Binary Search Tree help?
2. How can you ensure that the intervals present in the set are non-overlapping? Try merging the overlapping intervals whenever a new interval is added.
3. How can you update the count of integers present in at least one interval when a new interval is added to the set?

## Solution

```rust

```