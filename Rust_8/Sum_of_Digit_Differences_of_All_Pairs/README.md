# Sum of Digit Differences of All Pairs

**Difficulty:** Medium
**Tags:** Array, Hash Table, Math, Counting

---

## Problem

<p>You are given an array <code>nums</code> consisting of <strong>positive</strong> integers where all integers have the <strong>same</strong> number of digits.</p>

<p>The <strong>digit difference</strong> between two integers is the <em>count</em> of different digits that are in the <strong>same</strong> position in the two integers.</p>

<p>Return the <strong>sum</strong> of the <strong>digit differences</strong> between <strong>all</strong> pairs of integers in <code>nums</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [13,23,12]</span></p>

<p><strong>Output:</strong> 4</p>

<p><strong>Explanation:</strong><br />
We have the following:<br />
- The digit difference between <strong>1</strong>3 and <strong>2</strong>3 is 1.<br />
- The digit difference between 1<strong>3</strong> and 1<strong>2</strong> is 1.<br />
- The digit difference between <strong>23</strong> and <strong>12</strong> is 2.<br />
So the total sum of digit differences between all pairs of integers is <code>1 + 1 + 2 = 4</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [10,10,10,10]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong><br />
All the integers in the array are the same. So the total sum of digit differences between all pairs of integers will be 0.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt; 10<sup>9</sup></code></li>
	<li>All integers in <code>nums</code> have the same number of digits.</li>
</ul>


## Hints

1. You can solve the problem for digits that are on the same position separately, and then sum up all the answers.
2. For each position, count the number of occurences of each digit from 0 to 9 that appear on that position.
3. Let <code>c</code> be the number of occurences of a digit on a position, that will contribute with <code>c * (n - c)</code> to the final answer, where <code>n</code> is the number of integers in <code>nums</code>.

## Solution

```rust

```