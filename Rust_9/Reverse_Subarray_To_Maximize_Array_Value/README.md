# Reverse Subarray To Maximize Array Value

**Difficulty:** Hard
**Tags:** Array, Math, Greedy

---

## Problem

<p>You are given an integer array <code>nums</code>. The <em>value</em> of this array is defined as the sum of <code>|nums[i] - nums[i + 1]|</code> for all <code>0 &lt;= i &lt; nums.length - 1</code>.</p>

<p>You are allowed to select any subarray of the given array and reverse it. You can perform this operation <strong>only once</strong>.</p>

<p>Find maximum possible value of the final array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,3,1,5,4]
<strong>Output:</strong> 10
<b>Explanation: </b>By reversing the subarray [3,1,5] the array becomes [2,5,1,3,4] whose value is 10.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,4,9,24,2,1,10]
<strong>Output:</strong> 68
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>-10<sup>5</sup> &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li>The answer is guaranteed to fit in a 32-bit integer.</li>
</ul>


## Hints

1. What's the score after reversing a sub-array [L, R] ?
2. It's the score without reversing it + abs(a[R] - a[L-1]) + abs(a[L] - a[R+1]) - abs(a[L] - a[L-1]) - abs(a[R] - a[R+1])
3. How to maximize that formula given that abs(x - y) = max(x - y, y - x) ?
4. This can be written as max(max(a[R] - a[L - 1], a[L - 1] - a[R]) + max(a[R + 1] - a[L], a[L] - a[R + 1]) - value(L) - value(R + 1)) over all L < R where value(i) = abs(a[i] - a[i-1])
5. This can be divided into 4 cases.

## Solution

```rust

```