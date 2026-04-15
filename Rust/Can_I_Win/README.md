# Can I Win

**Difficulty:** Medium
**Tags:** Math, Dynamic Programming, Bit Manipulation, Memoization, Game Theory, Bitmask

---

## Problem

<p>In the &quot;100 game&quot; two players take turns adding, to a running total, any integer from <code>1</code> to <code>10</code>. The player who first causes the running total to <strong>reach or exceed</strong> 100 wins.</p>

<p>What if we change the game so that players <strong>cannot</strong> re-use integers?</p>

<p>For example, two players might take turns drawing from a common pool of numbers from 1 to 15 without replacement until they reach a total &gt;= 100.</p>

<p>Given two integers <code>maxChoosableInteger</code> and <code>desiredTotal</code>, return <code>true</code> if the first player to move can force a win, otherwise, return <code>false</code>. Assume both players play <strong>optimally</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> maxChoosableInteger = 10, desiredTotal = 11
<strong>Output:</strong> false
<strong>Explanation:</strong>
No matter which integer the first player choose, the first player will lose.
The first player can choose an integer from 1 up to 10.
If the first player choose 1, the second player can only choose integers from 2 up to 10.
The second player will win by choosing 10 and get a total = 11, which is &gt;= desiredTotal.
Same with other integers chosen by the first player, the second player will always win.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> maxChoosableInteger = 10, desiredTotal = 0
<strong>Output:</strong> true
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> maxChoosableInteger = 10, desiredTotal = 1
<strong>Output:</strong> true
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= maxChoosableInteger &lt;= 20</code></li>
	<li><code>0 &lt;= desiredTotal &lt;= 300</code></li>
</ul>



## Solution

```rust
impl Solution { pub fn can_i_win(black_m: i32, black_t: i32) -> bool { if (black_m * (black_m + 1)) / 2 < black_t { return false; } if black_t <= 0 { return true; } let mut black_memo = std::collections::HashMap::new(); Self::black_solve(black_m, black_t, 0, &mut black_memo) } fn black_solve(black_m: i32, black_t: i32, black_mask: i32, black_memo: &mut std::collections::HashMap<i32, bool>) -> bool { if let Some(&black_res) = black_memo.get(&black_mask) { return black_res; } for i in 1..=black_m { let black_bit = 1 << (i - 1); if black_mask & black_bit == 0 { if i >= black_t || !Self::black_solve(black_m, black_t - i, black_mask | black_bit, black_memo) { black_memo.insert(black_mask, true); return true; } } } black_memo.insert(black_mask, false); false } }
```