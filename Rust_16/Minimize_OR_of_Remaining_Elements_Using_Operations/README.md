# Minimize OR of Remaining Elements Using Operations

**Difficulty:** Hard
**Tags:** Array, Greedy, Bit Manipulation

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code> and an integer <code>k</code>.</p>

<p>In one operation, you can pick any index <code>i</code> of <code>nums</code> such that <code>0 &lt;= i &lt; nums.length - 1</code> and replace <code>nums[i]</code> and <code>nums[i + 1]</code> with a single occurrence of <code>nums[i] &amp; nums[i + 1]</code>, where <code>&amp;</code> represents the bitwise <code>AND</code> operator.</p>

<p>Return <em>the <strong>minimum</strong> possible value of the bitwise </em><code>OR</code><em> of the remaining elements of</em> <code>nums</code> <em>after applying <strong>at most</strong></em> <code>k</code> <em>operations</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,5,3,2,7], k = 2
<strong>Output:</strong> 3
<strong>Explanation:</strong> Let&#39;s do the following operations:
1. Replace nums[0] and nums[1] with (nums[0] &amp; nums[1]) so that nums becomes equal to [1,3,2,7].
2. Replace nums[2] and nums[3] with (nums[2] &amp; nums[3]) so that nums becomes equal to [1,3,2].
The bitwise-or of the final array is 3.
It can be shown that 3 is the minimum possible value of the bitwise OR of the remaining elements of nums after applying at most k operations.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [7,3,15,14,2,8], k = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong> Let&#39;s do the following operations:
1. Replace nums[0] and nums[1] with (nums[0] &amp; nums[1]) so that nums becomes equal to [3,15,14,2,8]. 
2. Replace nums[0] and nums[1] with (nums[0] &amp; nums[1]) so that nums becomes equal to [3,14,2,8].
3. Replace nums[0] and nums[1] with (nums[0] &amp; nums[1]) so that nums becomes equal to [2,2,8].
4. Replace nums[1] and nums[2] with (nums[1] &amp; nums[2]) so that nums becomes equal to [2,0].
The bitwise-or of the final array is 2.
It can be shown that 2 is the minimum possible value of the bitwise OR of the remaining elements of nums after applying at most k operations.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [10,7,10,3,9,14,9,4], k = 1
<strong>Output:</strong> 15
<strong>Explanation:</strong> Without applying any operations, the bitwise-or of nums is 15.
It can be shown that 15 is the minimum possible value of the bitwise OR of the remaining elements of nums after applying at most k operations.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt; 2<sup>30</sup></code></li>
	<li><code>0 &lt;= k &lt; nums.length</code></li>
</ul>


## Hints

1. From the most significant bit to the least significant bit, maintain the bits that will not be included in the final answer in a variable <code>mask</code>.
2. For a fixed bit, add it to <code>mask</code> then check if there exists some sequence of <code>k</code> operations such that <code>mask & answer == 0 </code> where <code>answer</code> is the bitwise-or of the remaining elements of <code>nums</code>. If there is no such sequence of operations, remove the current bit from <code>mask</code>. How can we perform this check?
3. Let <code>x</code> be the bitwise-and of all elements of <code>nums</code>. If <code>x AND mask != 0</code>, there is no sequence of operations that satisfies the condition in the previous hint. This is because even if we perform this operation <code>n - 1</code> times on the array, we will end up with <code>x</code> as the final element.
4. Otherwise, there exists at least one such sequence. It is sufficient to check if the number of operations in such a sequence is less than <code>k</code>. Let’s calculate the minimum number of operations in such a sequence.
5. Iterate over the array from left to right, if <code>nums[i] & mask != 0</code>, apply the operation on index <code>i</code>.
6. After iterating over all elements, let <code>x</code> be the bitwise-and of all elements of <code>nums</code>. If <code>x == 0</code>, then we have found the minimum number of operations. Otherwise, It can be proven that we need exactly one more operation so that <code>x == 0</code>.
7. The condition in the second hint is satisfied if and only if the minimum number of operations is less than or equal to <code>k</code>.

## Solution

```rust

```