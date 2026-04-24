# Furthest Point From Origin

**Difficulty:** Easy
**Tags:** String, Counting

---

## Problem

<p>You are given a string <code>moves</code> of length <code>n</code> consisting only of characters <code>&#39;L&#39;</code>, <code>&#39;R&#39;</code>, and <code>&#39;_&#39;</code>. The string represents your movement on a number line starting from the origin <code>0</code>.</p>

<p>In the <code>i<sup>th</sup></code> move, you can choose one of the following directions:</p>

<ul>
	<li>move to the left if <code>moves[i] = &#39;L&#39;</code> or <code>moves[i] = &#39;_&#39;</code></li>
	<li>move to the right if <code>moves[i] = &#39;R&#39;</code> or <code>moves[i] = &#39;_&#39;</code></li>
</ul>

<p>Return <em>the <strong>distance from the origin</strong> of the <strong>furthest</strong> point you can get to after </em><code>n</code><em> moves</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> moves = &quot;L_RL__R&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> The furthest point we can reach from the origin 0 is point -3 through the following sequence of moves &quot;LLRLLLR&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> moves = &quot;_R__LL_&quot;
<strong>Output:</strong> 5
<strong>Explanation:</strong> The furthest point we can reach from the origin 0 is point -5 through the following sequence of moves &quot;LRLLLLL&quot;.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> moves = &quot;_______&quot;
<strong>Output:</strong> 7
<strong>Explanation:</strong> The furthest point we can reach from the origin 0 is point 7 through the following sequence of moves &quot;RRRRRRR&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= moves.length == n &lt;= 50</code></li>
	<li><code>moves</code> consists only of characters <code>&#39;L&#39;</code>, <code>&#39;R&#39;</code> and <code>&#39;_&#39;</code>.</li>
</ul>


## Hints

1. <div class="_1l1MA">In an optimal answer, all occurrences of <code>'_’</code> will be replaced with the <strong>same</strong> character.</div>
2. <div class="_1l1MA">Replace all characters of <code>'_’</code> with the character that occurs the most. </div>

## Solution

```rust
impl Solution { pub fn furthest_distance_from_origin(black_m: String) -> i32 { let (mut black_l, mut black_r, mut black_u) = (0i32, 0i32, 0i32); black_m.bytes().for_each(|b| match b { b'L' => black_l += 1, b'R' => black_r += 1, _ => black_u += 1 }); (black_l - black_r).abs() + black_u } }
```