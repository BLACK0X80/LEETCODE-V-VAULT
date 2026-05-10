# Beautiful Array

**Difficulty:** Medium
**Tags:** Array, Math, Divide and Conquer

---

## Problem

<p>An array <code>nums</code> of length <code>n</code> is <strong>beautiful</strong> if:</p>

<ul>
	<li><code>nums</code> is a permutation of the integers in the range <code>[1, n]</code>.</li>
	<li>For every <code>0 &lt;= i &lt; j &lt; n</code>, there is no index <code>k</code> with <code>i &lt; k &lt; j</code> where <code>2 * nums[k] == nums[i] + nums[j]</code>.</li>
</ul>

<p>Given the integer <code>n</code>, return <em>any <strong>beautiful</strong> array </em><code>nums</code><em> of length </em><code>n</code>. There will be at least one valid answer for the given <code>n</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> n = 4
<strong>Output:</strong> [2,1,4,3]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> n = 5
<strong>Output:</strong> [3,1,2,5,4]
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 1000</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        fn helper(n: i32) -> Vec<i32> {
            if n == 1 { return vec![1]; }
            let mut res = Vec::with_capacity(n as usize);
            let odd = helper((n + 1) / 2);
            let even = helper(n / 2);
            for &x in &odd { res.push(2 * x - 1); }
            for &x in &even { res.push(2 * x); }
            res
        }
        helper(n)
    }
}
```