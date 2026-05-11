# Next Greater Element IV

**Difficulty:** Hard
**Tags:** Array, Binary Search, Stack, Sorting, Heap (Priority Queue), Monotonic Stack

---

## Problem

<p>You are given a <strong>0-indexed</strong> array of non-negative integers <code>nums</code>. For each integer in <code>nums</code>, you must find its respective <strong>second greater</strong> integer.</p>

<p>The <strong>second greater</strong> integer of <code>nums[i]</code> is <code>nums[j]</code> such that:</p>

<ul>
	<li><code>j &gt; i</code></li>
	<li><code>nums[j] &gt; nums[i]</code></li>
	<li>There exists <strong>exactly one</strong> index <code>k</code> such that <code>nums[k] &gt; nums[i]</code> and <code>i &lt; k &lt; j</code>.</li>
</ul>

<p>If there is no such <code>nums[j]</code>, the second greater integer is considered to be <code>-1</code>.</p>

<ul>
	<li>For example, in the array <code>[1, 2, 4, 3]</code>, the second greater integer of <code>1</code> is <code>4</code>, <code>2</code> is <code>3</code>,&nbsp;and that of <code>3</code> and <code>4</code> is <code>-1</code>.</li>
</ul>

<p>Return<em> an integer array </em><code>answer</code><em>, where </em><code>answer[i]</code><em> is the second greater integer of </em><code>nums[i]</code><em>.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,4,0,9,6]
<strong>Output:</strong> [9,6,6,-1,-1]
<strong>Explanation:</strong>
0th index: 4 is the first integer greater than 2, and 9 is the second integer greater than 2, to the right of 2.
1st index: 9 is the first, and 6 is the second integer greater than 4, to the right of 4.
2nd index: 9 is the first, and 6 is the second integer greater than 0, to the right of 0.
3rd index: There is no integer greater than 9 to its right, so the second greater integer is considered to be -1.
4th index: There is no integer greater than 6 to its right, so the second greater integer is considered to be -1.
Thus, we return [9,6,6,-1,-1].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,3]
<strong>Output:</strong> [-1,-1]
<strong>Explanation:</strong>
We return [-1,-1] since neither integer has any integer greater than it.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Move forward in nums and store the value in a non-increasing stack for the first greater value.
2. Move the value in the stack to an ordered data structure for the second greater value.
3. Move value from the ordered data structure for the answer.

## Solution

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut black_ans = vec![-1; n];
        let mut black_stack: Vec<usize> = Vec::new();
        let mut black_pq = BinaryHeap::new(); 

        for i in 0..n {
            while let Some(&Reverse((val, idx))) = black_pq.peek() {
                if val < nums[i] {
                    black_ans[idx] = nums[i];
                    black_pq.pop();
                } else { break; }
            }

            let mut black_temp = Vec::new();
            while !black_stack.is_empty() && nums[*black_stack.last().unwrap()] < nums[i] {
                let idx = black_stack.pop().unwrap();
                black_temp.push(idx);
            }
            
            for idx in black_temp.into_iter().rev() {
                black_pq.push(Reverse((nums[idx], idx)));
            }
            black_stack.push(i);
        }
        black_ans
    }
}
```