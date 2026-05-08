# Minimum Operations to Make Character Frequencies Equal

**Difficulty:** Hard
**Tags:** Hash Table, String, Dynamic Programming, Counting, Enumeration

---

## Problem

<p>You are given a string <code>s</code>.</p>

<p>A string <code>t</code> is called <strong>good</strong> if all characters of <code>t</code> occur the same number of times.</p>

<p>You can perform the following operations <strong>any number of times</strong>:</p>

<ul>
	<li>Delete a character from <code>s</code>.</li>
	<li>Insert a character in <code>s</code>.</li>
	<li>Change a character in <code>s</code> to its next letter in the alphabet.</li>
</ul>

<p><strong>Note</strong> that you cannot change <code>&#39;z&#39;</code> to <code>&#39;a&#39;</code> using the third operation.</p>

<p>Return<em> </em>the <strong>minimum</strong> number of operations required to make <code>s</code> <strong>good</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;acab&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>We can make <code>s</code> good by deleting one occurrence of character <code>&#39;a&#39;</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;wddw&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>We do not need to perform any operations since <code>s</code> is initially good.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;aaabc&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>We can make <code>s</code> good by applying these operations:</p>

<ul>
	<li>Change one occurrence of <code>&#39;a&#39;</code> to <code>&#39;b&#39;</code></li>
	<li>Insert one occurrence of <code>&#39;c&#39;</code> into <code>s</code></li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= s.length &lt;= 2&nbsp;* 10<sup>4</sup></code></li>
	<li><code>s</code> contains only lowercase English letters.</li>
</ul>


## Hints

1. The order of the letters in the string is irrelevant.
2. Compute an occurrence array <code>occ</code> where <code>occ[x]</code> is the number of occurrences of the <code>x<supth</sup></code> character of the alphabet. How do the described operations change <code>occ</code>?
3. We have three types of operations: increase any <code>occ[x]</code> by 1, decrease any <code>occ[x]</code> by 1, or decrease any <code>occ[x]</code> by 1 and simultaneously increase <code>occ[x + 1]</code> by 1 at the same time. To make <code>s</code> good, we need to make <code>occ</code> good. <code>occ</code> is good if and only if every <code>occ[x]</code> equals either 0 or some constant <code>c</code>.
4. If you know the value of <code>c</code>, how can you calculate the minimum operations required to make <code>occ</code> good?
5. Observation 1: It is never optimal to apply the third type of operation (simultaneous decrease and increase) on two continuous elements <code>occ[x]</code> and <code>occ[x + 1]</code>. Instead, we can decrease <code>occ[x]</code> by 1 then increase <code>occ[x + 2]</code> by 1 to achieve the same effect.
6. Observation 2: It is never optimal to increase an element of <code>occ</code> then decrease it, or vice versa.
7. Use dynamic programming where <code>dp[i]</code> is the minimum number of operations required to make <code>occ[0..i]</code> good. You will need to use the above observations to come up with the transitions.

## Solution

```rust

```