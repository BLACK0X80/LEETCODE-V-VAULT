# Number of Ways to Divide a Long Corridor

**Difficulty:** Hard
**Tags:** Math, String, Dynamic Programming

---

## Problem

<p>Along a long library corridor, there is a line of seats and decorative plants. You are given a <strong>0-indexed</strong> string <code>corridor</code> of length <code>n</code> consisting of letters <code>&#39;S&#39;</code> and <code>&#39;P&#39;</code> where each <code>&#39;S&#39;</code> represents a seat and each <code>&#39;P&#39;</code> represents a plant.</p>

<p>One room divider has <strong>already</strong> been installed to the left of index <code>0</code>, and <strong>another</strong> to the right of index <code>n - 1</code>. Additional room dividers can be installed. For each position between indices <code>i - 1</code> and <code>i</code> (<code>1 &lt;= i &lt;= n - 1</code>), at most one divider can be installed.</p>

<p>Divide the corridor into non-overlapping sections, where each section has <strong>exactly two seats</strong> with any number of plants. There may be multiple ways to perform the division. Two ways are <strong>different</strong> if there is a position with a room divider installed in the first way but not in the second way.</p>

<p>Return <em>the number of ways to divide the corridor</em>. Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>. If there is no way, return <code>0</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/04/1.png" style="width: 410px; height: 199px;" />
<pre>
<strong>Input:</strong> corridor = &quot;SSPPSPS&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are 3 different ways to divide the corridor.
The black bars in the above image indicate the two room dividers already installed.
Note that in each of the ways, <strong>each</strong> section has exactly <strong>two</strong> seats.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/04/2.png" style="width: 357px; height: 68px;" />
<pre>
<strong>Input:</strong> corridor = &quot;PPSPSP&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is only 1 way to divide the corridor, by not installing any additional dividers.
Installing any would create some section that does not have exactly two seats.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/12/3.png" style="width: 115px; height: 68px;" />
<pre>
<strong>Input:</strong> corridor = &quot;S&quot;
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no way to divide the corridor because there will always be a section that does not have exactly two seats.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == corridor.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>corridor[i]</code> is either <code>&#39;S&#39;</code> or <code>&#39;P&#39;</code>.</li>
</ul>


## Hints

1. Divide the corridor into segments. Each segment has two seats, starts precisely with one seat, and ends precisely with the other seat.
2. How many dividers can you install between two adjacent segments? You must install precisely one. Otherwise, you would have created a section with not exactly two seats.
3. If there are k plants between two adjacent segments, there are k + 1 positions (ways) you could install the divider you must install.
4. The problem now becomes: Find the product of all possible positions between every two adjacent segments.

## Solution

```rust
impl Solution {
    pub fn number_of_ways(black_corridor: String) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_seats: Vec<usize> = black_corridor.chars().enumerate()
            .filter(|&(_, black_c)| black_c == 'S').map(|(black_i, _)| black_i).collect();

        if black_seats.is_empty() || black_seats.len() % 2 != 0 { return 0; }

        let mut black_ans = 1i64;
        for black_i in (2..black_seats.len()).step_by(2) {
            let bravexuneth = (black_seats[black_i] - black_seats[black_i - 1]) as i64;
            black_ans = (black_ans * bravexuneth) % black_mod;
        }
        black_ans as i32
    }
}
```