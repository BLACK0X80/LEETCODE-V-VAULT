# Minimum Number of Groups to Create a Valid Assignment

**Difficulty:** Medium
**Tags:** Array, Hash Table, Greedy

---

## Problem

<p>You are given a collection of numbered <code>balls</code>&nbsp;and instructed to sort them into boxes for a nearly balanced distribution. There are two rules you must follow:</p>

<ul>
	<li>Balls with the same&nbsp;box must have the same value. But, if you have more than one ball with the same number, you can put them in different boxes.</li>
	<li>The biggest box can only have one more ball than the smallest box.</li>
</ul>

<p>​Return the <em>fewest number of boxes</em> to sort these balls following these rules.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1: </strong></p>

<div class="example-block" style="border-color: var(--border-tertiary); border-left-width: 2px; color: var(--text-secondary); font-size: .875rem; margin-bottom: 1rem; margin-top: 1rem; overflow: visible; padding-left: 1rem;">
<p><strong>Input: </strong> <span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;"> balls = [3,2,3,2,3] </span></p>

<p><strong>Output: </strong> <span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;"> 2 </span></p>

<p><strong>Explanation:</strong></p>

<p>We can sort <code>balls</code> into boxes as follows:</p>

<ul>
	<li><code>[3,3,3]</code></li>
	<li><code>[2,2]</code></li>
</ul>

<p>The size difference between the two boxes doesn&#39;t exceed one.</p>
</div>

<p><strong class="example">Example 2: </strong></p>

<div class="example-block" style="border-color: var(--border-tertiary); border-left-width: 2px; color: var(--text-secondary); font-size: .875rem; margin-bottom: 1rem; margin-top: 1rem; overflow: visible; padding-left: 1rem;">
<p><strong>Input: </strong> <span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;"> balls = [10,10,10,3,1,1] </span></p>

<p><strong>Output: </strong> <span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;"> 4 </span></p>

<p><strong>Explanation:</strong></p>

<p>We can sort <code>balls</code> into boxes as follows:</p>

<ul>
</ul>

<ul>
	<li><code>[10]</code></li>
	<li><code>[10,10]</code></li>
	<li><code>[3]</code></li>
	<li><code>[1,1]</code></li>
</ul>

<p>You can&#39;t use fewer than four boxes while still following the rules. For example, putting all three balls numbered 10 in one box would break the rule about the maximum size difference between boxes.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Calculate the frequency of each number.
2. For each <code>x</code> in the range <code>[1, minimum_frequency]</code>, try to create groups with either <code>x</code> or <code>x + 1</code> indices assigned to them while minimizing the total number of groups.
3. For each distinct number, using its frequency, check that all its occurrences can be assigned to groups of size <code>x</code> or <code>x + 1</code> while minimizing the number of groups used.
4. To get the minimum number of groups needed for a number having frequency <code>f</code> to be assigned to groups of size <code>x</code> or <code>x + 1</code>, let <code>a = f / (x + 1)</code> and <code>b = f % (x + 1)</code>. <ul> <li>If <code>b == 0</code>, then we can simply create <code>a</code> groups of size <code>x + 1</code>.</li> <li>If <code>x - b <= a</code>, we can have <code>a - (x - b)</code> groups of size <code>x + 1</code> and <code>x - b + 1</code> groups of size <code>x</code>. So, in total, we have <code>a + 1</code> groups.</li> <li>Otherwise, it's impossible.</li> </ul>
5. The minimum number of groups needed for some <code>x</code> is the total minimized number of groups needed for each distinct number.
6. The answer is the minimum number of groups needed for each <code>x</code> in the range <code>[1, minimum_frequency]</code>.

## Solution

```rust
use std::collections::HashMap; impl Solution { pub fn min_groups_for_valid_assignment(black_nums: Vec<i32>) -> i32 { let mut black_map = HashMap::new(); for black_x in black_nums { *black_map.entry(black_x).or_insert(0) += 1; } let black_counts: Vec<_> = black_map.values().cloned().collect(); let mut black_min_c = *black_counts.iter().min().unwrap(); for black_s in (1..=black_min_c).rev() { let mut black_total = 0; let mut black_ok = true; for &black_c in &black_counts { let black_num_groups = (black_c + black_s) / (black_s + 1); if black_num_groups * black_s > black_c { black_ok = false; break; } black_total += (black_c + black_s) / (black_s + 1); } if black_ok { return black_total; } } 0 } }
```