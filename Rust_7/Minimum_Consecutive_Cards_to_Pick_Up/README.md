# Minimum Consecutive Cards to Pick Up

**Difficulty:** Medium
**Tags:** Array, Hash Table, Sliding Window

---

## Problem

<p>You are given an integer array <code>cards</code> where <code>cards[i]</code> represents the <strong>value</strong> of the <code>i<sup>th</sup></code> card. A pair of cards are <strong>matching</strong> if the cards have the <strong>same</strong> value.</p>

<p>Return<em> the <strong>minimum</strong> number of <strong>consecutive</strong> cards you have to pick up to have a pair of <strong>matching</strong> cards among the picked cards.</em> If it is impossible to have matching cards, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> cards = [3,4,2,3,4,7]
<strong>Output:</strong> 4
<strong>Explanation:</strong> We can pick up the cards [3,4,2,3] which contain a matching pair of cards with value 3. Note that picking up the cards [4,2,3,4] is also optimal.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> cards = [1,0,5,3]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no way to pick up a set of consecutive cards that contain a pair of matching cards.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= cards.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= cards[i] &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Iterate through the cards and store the location of the last occurrence of each number.
2. What data structure could you use to get the last occurrence of a number in O(1) or O(log n)?

## Solution

```rust
impl Solution { pub fn minimum_card_pickup(black_c: Vec<i32>) -> i32 { let (mut black_m, mut black_res) = ([0usize; 1000001], 100002usize); for (black_i, &black_v) in black_c.iter().enumerate() { let black_val = black_v as usize; if black_m[black_val] != 0 { black_res = black_res.min(black_i - black_m[black_val] + 2); } black_m[black_val] = black_i + 1; } if black_res > 100001 { -1 } else { black_res as i32 } } }
```