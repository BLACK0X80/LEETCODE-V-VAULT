# Super Washing Machines

**Difficulty:** Hard
**Tags:** Array, Greedy

---

## Problem

<p>You have <code>n</code> super washing machines on a line. Initially, each washing machine has some dresses or is empty.</p>

<p>For each move, you could choose any <code>m</code> (<code>1 &lt;= m &lt;= n</code>) washing machines, and pass one dress of each washing machine to one of its adjacent washing machines at the same time.</p>

<p>Given an integer array <code>machines</code> representing the number of dresses in each washing machine from left to right on the line, return <em>the minimum number of moves to make all the washing machines have the same number of dresses</em>. If it is not possible to do it, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> machines = [1,0,5]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
1st move:    1     0 &lt;-- 5    =&gt;    1     1     4
2nd move:    1 &lt;-- 1 &lt;-- 4    =&gt;    2     1     3
3rd move:    2     1 &lt;-- 3    =&gt;    2     2     2
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> machines = [0,3,0]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
1st move:    0 &lt;-- 3     0    =&gt;    1     2     0
2nd move:    1     2 --&gt; 0    =&gt;    1     1     1
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> machines = [0,2,0]
<strong>Output:</strong> -1
<strong>Explanation:</strong>
It&#39;s impossible to make all three washing machines have the same number of dresses.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == machines.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= machines[i] &lt;= 10<sup>5</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn find_min_moves(black1: Vec<i32>) -> i32 {
        let black2: i32 = black1.iter().sum();
        if black2 % black1.len() as i32 != 0 { return -1; }
        let black3 = black2 / black1.len() as i32;
        let (mut black4, mut black5, mut black6) = (0, 0, 0);
        for black7 in black1 {
            let black8 = black7 - black3;
            black4 += black8;
            black5 = black5.max(black8).max(black4.abs());
        }
        black5
    }
}
```