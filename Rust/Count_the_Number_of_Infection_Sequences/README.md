# Count the Number of Infection Sequences

**Difficulty:** Hard
**Tags:** Array, Math, Combinatorics

---

## Problem

<p>You are given an integer <code>n</code> and an array <code>sick</code> sorted in increasing order, representing positions of infected people in a line of <code>n</code> people.</p>

<p>At each step, <strong>one </strong>uninfected person <strong>adjacent</strong> to an infected person gets infected. This process continues until everyone is infected.</p>

<p>An <strong>infection sequence</strong> is the order in which uninfected people become infected, excluding those initially infected.</p>

<p>Return the number of different infection sequences possible, modulo <code>10<sup>9</sup>+7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, sick = [0,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>There is a total of 6 different sequences overall.</p>

<ul>
	<li>Valid infection sequences are <code>[1,2,3]</code>, <code>[1,3,2]</code>, <code>[3,2,1]</code> and <code>[3,1,2]</code>.</li>
	<li><code>[2,3,1]</code> and <code>[2,1,3]</code> are not valid infection sequences because the person at index 2 cannot be infected at the first step.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4, sick = [1]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>There is a total of 6 different sequences overall.</p>

<ul>
	<li>Valid infection sequences are <code>[0,2,3]</code>, <code>[2,0,3]</code> and <code>[2,3,0]</code>.</li>
	<li><code>[3,2,0]</code>, <code>[3,0,2]</code>, and <code>[0,3,2]</code> are not valid infection sequences because the infection starts at the person at index 1, then the order of infection is 2, then 3, and hence 3 cannot be infected earlier than 2.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= sick.length &lt;= n - 1</code></li>
	<li><code>0 &lt;= sick[i] &lt;= n - 1</code></li>
	<li><code>sick</code> is sorted in increasing order.</li>
</ul>


## Hints

1. Consider infected children as <code>0</code> and non-infected as <code>1</code>, then divide the array into segments with the same value.
2. For each segment of non-infected children whose indices are <code>[i, j]</code> and indices <code>(i - 1)</code> and <code>(j + 1)</code>, if they exist, are already infected. Then if <code>i == 0</code> or <code>j == n - 1</code>, each second there is only one kid that can be infected (which is at the other endpoint).
3. If <code>i > 0</code> and <code>j < n - 1</code>, we have two choices per second since the children at the two endpoints can both be the infect candidates. So there are <code>2<sup>j - i</sup></code> orders to infect all children in the segment.
4. Each second we can select a segment and select one endpoint from it.
5. The answer is: 
<code>S! / (len[1]! * len[2]! * ... * len[m]! * len<sub>start</sub>! * len<sub>end</sub>!) * 2<sup>k</sup></code> 
where <code>len[1], len[2], ..., len[m]</code> are the lengths of each segment of non-infected children that have an infected child at both endpoints, <code>len<sub>start</sub></code> and <code>len<sub>end</sub></code> denote the number of non-infected children with infected child at one endpoint, <code>S</code> is the total length of all segments of non-infected children, and <code>k = (len[1] - 1) + (len[2] - 1) + ... + (len[m] - 1)</code>.

## Solution

```rust

```