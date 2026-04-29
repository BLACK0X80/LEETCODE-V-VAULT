# Find Number of Ways to Reach the K-th Stair

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming, Bit Manipulation, Memoization, Combinatorics

---

## Problem

<p>You are given a <strong>non-negative</strong> integer <code>k</code>. There exists a staircase with an infinite number of stairs, with the <strong>lowest</strong> stair numbered 0.</p>

<p>Alice has an integer <code>jump</code>, with an initial value of 0. She starts on stair 1 and wants to reach stair <code>k</code> using <strong>any</strong> number of <strong>operations</strong>. If she is on stair <code>i</code>, in one <strong>operation</strong> she can:</p>

<ul>
	<li>Go down to stair <code>i - 1</code>. This operation <strong>cannot</strong> be used consecutively or on stair 0.</li>
	<li>Go up to stair <code>i + 2<sup>jump</sup></code>. And then, <code>jump</code> becomes <code>jump + 1</code>.</li>
</ul>

<p>Return the <em>total</em> number of ways Alice can reach stair <code>k</code>.</p>

<p><strong>Note</strong> that it is possible that Alice reaches the stair <code>k</code>, and performs some operations to reach the stair <code>k</code> again.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">k = 0</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The 2 possible ways of reaching stair 0 are:</p>

<ul>
	<li>Alice starts at stair 1.
	<ul>
		<li>Using an operation of the first type, she goes down 1 stair to reach stair 0.</li>
	</ul>
	</li>
	<li>Alice starts at stair 1.
	<ul>
		<li>Using an operation of the first type, she goes down 1 stair to reach stair 0.</li>
		<li>Using an operation of the second type, she goes up 2<sup>0</sup> stairs to reach stair 1.</li>
		<li>Using an operation of the first type, she goes down 1 stair to reach stair 0.</li>
	</ul>
	</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The 4 possible ways of reaching stair 1 are:</p>

<ul>
	<li>Alice starts at stair 1. Alice is at stair 1.</li>
	<li>Alice starts at stair 1.
	<ul>
		<li>Using an operation of the first type, she goes down 1 stair to reach stair 0.</li>
		<li>Using an operation of the second type, she goes up 2<sup>0</sup> stairs to reach stair 1.</li>
	</ul>
	</li>
	<li>Alice starts at stair 1.
	<ul>
		<li>Using an operation of the second type, she goes up 2<sup>0</sup> stairs to reach stair 2.</li>
		<li>Using an operation of the first type, she goes down 1 stair to reach stair 1.</li>
	</ul>
	</li>
	<li>Alice starts at stair 1.
	<ul>
		<li>Using an operation of the first type, she goes down 1 stair to reach stair 0.</li>
		<li>Using an operation of the second type, she goes up 2<sup>0</sup> stairs to reach stair 1.</li>
		<li>Using an operation of the first type, she goes down 1 stair to reach stair 0.</li>
		<li>Using an operation of the second type, she goes up 2<sup>1</sup> stairs to reach stair 2.</li>
		<li>Using an operation of the first type, she goes down 1 stair to reach stair 1.</li>
	</ul>
	</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= k &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. On using <code>x</code> operations of the second type and <code>y</code> operations of the first type, the stair <code>2<sup>x</sup> - y</code> is reached.
2. Since first operations cannot be consecutive, there are exactly <code>x + 1</code> positions (before and after each power of 2) to perform the second operation.
3. Using combinatorics, we have <sup>x + 1</sup>C<sub>y</sub> number of ways to select the positions of second operations.

## Solution

```rust
use std::collections::HashMap;
impl Solution {
    pub fn ways_to_reach_stair(black1: i32) -> i32 {
        fn black2(black3: i32, black4: i32, black5: bool, black6: i32, black7: &mut HashMap<(i32, i32, bool), i32>) -> i32 {
            if black3 > black6 + 1 { return 0; }
            if let Some(&black8) = black7.get(&(black3, black4, black5)) { return black8; }
            let mut black9 = if black3 == black6 { 1 } else { 0 };
            if black5 && black3 > 0 { black9 += black2(black3 - 1, black4, false, black6, black7); }
            black9 += black2(black3 + (1 << black4), black4 + 1, true, black6, black7);
            black7.insert((black3, black4, black5), black9);
            black9
        }
        black2(1, 0, true, black1, &mut HashMap::new())
    }
}
```