# Two-Letter Card Game

**Difficulty:** Medium
**Tags:** Array, Hash Table, String, Counting, Enumeration

---

## Problem

<p>You are given a deck of cards represented by a string array <code>cards</code>, and each card displays two lowercase letters.</p>

<p>You are also given a letter <code>x</code>. You play a game with the following rules:</p>

<ul>
	<li>Start with 0 points.</li>
	<li>On each turn, you must find two <strong>compatible</strong> cards from the deck that both contain the letter <code>x</code> in any position.</li>
	<li>Remove the pair of cards and earn <strong>1 point</strong>.</li>
	<li>The game ends when you can no longer find a pair of compatible cards.</li>
</ul>

<p>Return the <strong>maximum</strong> number of points you can gain with optimal play.</p>

<p>Two cards are <strong>compatible</strong> if the strings differ in <strong>exactly</strong> 1 position.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">cards = [&quot;aa&quot;,&quot;ab&quot;,&quot;ba&quot;,&quot;ac&quot;], x = &quot;a&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>On the first turn, select and remove cards <code>&quot;ab&quot;</code> and <code>&quot;ac&quot;</code>, which are compatible because they differ at only index 1.</li>
	<li>On the second turn, select and remove cards <code>&quot;aa&quot;</code> and <code>&quot;ba&quot;</code>, which are compatible because they differ at only index 0.</li>
</ul>

<p>Because there are no more compatible pairs, the total score is 2.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">cards = [&quot;aa&quot;,&quot;ab&quot;,&quot;ba&quot;], x = &quot;a&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>On the first turn, select and remove cards <code>&quot;aa&quot;</code> and <code>&quot;ba&quot;</code>.</li>
</ul>

<p>Because there are no more compatible pairs, the total score is 1.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">cards = [&quot;aa&quot;,&quot;ab&quot;,&quot;ba&quot;,&quot;ac&quot;], x = &quot;b&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>The only cards that contain the character <code>&#39;b&#39;</code> are <code>&quot;ab&quot;</code> and <code>&quot;ba&quot;</code>. However, they differ in both indices, so they are not compatible. Thus, the output is 0.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= cards.length &lt;= 10<sup>5</sup></code></li>
	<li><code>cards[i].length == 2</code></li>
	<li>Each <code>cards[i]</code> is composed of only lowercase English letters between <code>&#39;a&#39;</code> and <code>&#39;j&#39;</code>.</li>
	<li><code>x</code> is a lowercase English letter between <code>&#39;a&#39;</code> and <code>&#39;j&#39;</code>.</li>
</ul>


## Hints

1. Compute <code>both</code>, <code>cnt1</code>[c], <code>cnt2</code>[c] as the counts of cards with <code>x</code> in both positions, only the first position (other letter <code>c</code>), and only the second position.
2. Let <code>solve(cnt, have)</code> be the maximum pairs you can form from one‐sided counts <code>cnt</code> plus <code>have</code> two‐sided cards by sorting <code>cnt</code>, computing the total, and applying the same logic as in the solution.
3. Return the maximum over <code>i = 0..both</code> of <code>solve(cnt1, i) + solve(cnt2, both - i)</code>.

## Solution

```rust
use std::collections::{HashMap, BinaryHeap}; impl Solution { pub fn score(black_cards: Vec<String>, black_x: char) -> i32 { let (mut black_l_grp, mut black_r_grp, mut black_xx) = (HashMap::new(), HashMap::new(), 0); for black_c in black_cards { let black_b = black_c.as_bytes(); let (black_c1, black_c2) = (black_b[0] as char, black_b[1] as char); if black_c1 == black_x && black_c2 == black_x { black_xx += 1; } else if black_c1 == black_x { *black_l_grp.entry(black_b[1]).or_insert(0) += 1; } else if black_b[1] as char == black_x { *black_r_grp.entry(black_b[0]).or_insert(0) += 1; } } let (mut black_lpq, mut black_rpq, mut black_ans) = (BinaryHeap::from_iter(black_l_grp.values().cloned()), BinaryHeap::from_iter(black_r_grp.values().cloned()), 0); while black_lpq.len() > 1 { let (mut black_a, mut black_b) = (black_lpq.pop().unwrap(), black_lpq.pop().unwrap()); black_a -= 1; black_b -= 1; black_ans += 1; if black_a > 0 { black_lpq.push(black_a); } if black_b > 0 { black_lpq.push(black_b); } } while black_rpq.len() > 1 { let (mut black_a, mut black_b) = (black_rpq.pop().unwrap(), black_rpq.pop().unwrap()); black_a -= 1; black_b -= 1; black_ans += 1; if black_a > 0 { black_rpq.push(black_a); } if black_b > 0 { black_rpq.push(black_b); } } let (black_ans_now, mut black_l_rem, mut black_r_rem) = (black_ans, black_lpq.iter().sum::<i32>(), black_rpq.iter().sum::<i32>()); let black_c_l = black_l_rem.min(black_xx); black_ans += black_c_l; black_xx -= black_c_l; let black_c_r = black_r_rem.min(black_xx); black_ans += black_c_r; black_xx -= black_c_r; if black_xx > 0 { black_ans += black_ans_now.min(black_xx / 2); } black_ans } }
```