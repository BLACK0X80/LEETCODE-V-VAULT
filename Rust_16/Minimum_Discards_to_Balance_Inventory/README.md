#  Minimum Discards to Balance Inventory

**Difficulty:** Medium
**Tags:** Array, Hash Table, Sliding Window, Simulation, Counting

---

## Problem

<p>You are given two integers <code>w</code> and <code>m</code>, and an integer array <code>arrivals</code>, where <code>arrivals[i]</code> is the type of item arriving on day <code>i</code> (days are <strong>1-indexed</strong>).</p>

<p>Items are managed according to the following rules:</p>

<ul>
	<li>Each arrival may be <strong>kept</strong> or <strong>discarded</strong>; an item may only be discarded on its arrival day.</li>
	<li>For each day <code>i</code>, consider the window of days <code>[max(1, i - w + 1), i]</code> (the <code>w</code> most recent days up to day <code>i</code>):
	<ul>
		<li>For <strong>any</strong> such window, each item type may appear <strong>at most</strong> <code>m</code> times among kept arrivals whose arrival day lies in that window.</li>
		<li>If keeping the arrival on day <code>i</code> would cause its type to appear <strong>more than</strong> <code>m</code> times in the window, that arrival <strong>must</strong> be discarded.</li>
	</ul>
	</li>
</ul>

<p>Return the <strong>minimum</strong> number of arrivals to be discarded so that every <code>w</code>-day window contains at most <code>m</code> occurrences of each type.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">arrivals = [1,2,1,3,1], w = 4, m = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>On day 1, Item 1 arrives; the window contains no more than <code>m</code> occurrences of this type, so we keep it.</li>
	<li>On day 2, Item 2 arrives; the window of days 1 - 2 is fine.</li>
	<li>On day 3, Item 1 arrives, window <code>[1, 2, 1]</code> has item 1 twice, within limit.</li>
	<li>On day 4, Item 3 arrives, window <code>[1, 2, 1, 3]</code> has item 1 twice, allowed.</li>
	<li>On day 5, Item 1 arrives, window <code>[2, 1, 3, 1]</code> has item 1 twice, still valid.</li>
</ul>

<p>There are no discarded items, so return 0.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">arrivals = [1,2,3,3,3,4], w = 3, m = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>On day 1, Item 1 arrives. We keep it.</li>
	<li>On day 2, Item 2 arrives, window <code>[1, 2]</code> is fine.</li>
	<li>On day 3, Item 3 arrives, window <code>[1, 2, 3]</code> has item 3 once.</li>
	<li>On day 4, Item 3 arrives, window <code>[2, 3, 3]</code> has item 3 twice, allowed.</li>
	<li>On day 5, Item 3 arrives, window <code>[3, 3, 3]</code> has item 3 three times, exceeds limit, so the arrival must be discarded.</li>
	<li>On day 6, Item 4 arrives, window <code>[3, 4]</code> is fine.</li>
</ul>

<p>Item 3 on day 5 is discarded, and this is the minimum number of arrivals to discard, so return 1.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arrivals.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= arrivals[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= w &lt;= arrivals.length</code></li>
	<li><code>1 &lt;= m &lt;= w</code></li>
</ul>


## Hints

1. Use a sliding window of up to <code>w</code> days with two pointers <code>left</code> and <code>right</code> to represent the current interval.
2. Maintain a hash map <code>cnt</code> from item type to its current count in the window. When you advance <code>right</code> to day <code>i</code>, do <code>cnt[arrivals[i]]++</code>.
3. If the window size exceeds <code>w</code> (i.e. <code>right - left + 1 > w</code>), shrink it by doing <code>cnt[arrivals[left]]--</code> and then <code>left++</code>.
4. After each increment, check if <code>cnt[arrivals[right]] > m</code>. If so, we must discard the current arrival.

## Solution

```rust
use std::collections::HashMap; impl Solution { pub fn min_arrivals_to_discard(black_a: Vec<i32>, black_w: i32, black_m: i32) -> i32 { let (mut black_ans, mut black_map, mut black_kept) = (0, HashMap::new(), vec![]); for (black_i, &black_type) in black_a.iter().enumerate() { let black_limit = black_i as i32 - black_w + 1; while let Some(&(black_t, black_val)) = black_kept.first() { if black_t < black_limit { *black_map.get_mut(&black_val).unwrap() -= 1; black_kept.remove(0); } else { break; } } let black_count = *black_map.get(&black_type).unwrap_or(&0); if black_count < black_m { black_kept.push((black_i as i32, black_type)); *black_map.entry(black_type).or_insert(0) += 1; } else { black_ans += 1; } } black_ans } }
```