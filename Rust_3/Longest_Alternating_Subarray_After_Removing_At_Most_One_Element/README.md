# Longest Alternating Subarray After Removing At Most One Element

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Enumeration

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>A <span data-keyword="subarray">subarray</span> <code>nums[l..r]</code> is <strong>alternating</strong> if one of the following holds:</p>

<ul>
	<li><code>nums[l] &lt; nums[l + 1] &gt; nums[l + 2] &lt; nums[l + 3] &gt; ...</code></li>
	<li><code>nums[l] &gt; nums[l + 1] &lt; nums[l + 2] &gt; nums[l + 3] &lt; ...</code></li>
</ul>

<p>In other words, if we compare adjacent elements in the subarray, then the comparisons alternate between <strong>strictly</strong> greater and <strong>strictly</strong> smaller.</p>

<p>You can remove <strong>at most one</strong> element from <code>nums</code>. Then, you select an alternating subarray from <code>nums</code>.</p>

<p>Return an integer denoting the <strong>maximum</strong> <strong>length</strong> of the alternating subarray you can select.</p>

<p>A subarray of length 1 is considered alternating.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,1,3,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Choose not to remove elements.</li>
	<li>Select the entire array <code>[<u><strong>2, 1, 3, 2</strong></u>]</code>, which is alternating because <code>2 &gt; 1 &lt; 3 &gt; 2</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,2,1,2,3,2,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Choose to remove <code>nums[3]</code> i.e., <code>[3, 2, 1, <u><strong>2</strong></u>, 3, 2, 1]</code>. The array becomes <code>[3, 2, 1, 3, 2, 1]</code>.</li>
	<li>Select the subarray <code>[3, <strong><u>2, 1, 3, 2</u></strong>, 1]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [100000,100000]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Choose not to remove elements.</li>
	<li>Select the subarray <code>[100000, <u><strong>100000</strong></u>]</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Define <code>left[i][d]</code> as the maximum length of an alternating subarray ending at index <code>i</code>, where <code>d = 0</code> means the last comparison is <code><</code> and <code>d = 1</code> means the last comparison is <code>></code>. Define <code>right[i][d]</code> similarly for subarrays starting at <code>i</code>.
2. Fill <code>left</code> from left to right and <code>right</code> from right to left; if adjacent values are equal, the alternating chain must restart since <code>==</code> is invalid.
3. Try removing each index <code>r</code>: if <code>nums[r - 1] < nums[r + 1]</code>, the two sides can connect with pattern <code>< ></code>, giving length <code>left[r - 1][0] + right[r + 1][1]</code>; if <code>nums[r - 1] > nums[r + 1]</code>, connect with pattern <code>> <</code>, giving <code>left[r - 1][1] + right[r + 1][0]</code>.
4. Also consider not removing any element by taking the maximum value over all <code>left[i][d]</code>.

## Solution

```rust

```