# Number of Unique Good Subsequences

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>You are given a binary string <code>binary</code>. A <strong>subsequence</strong> of <code>binary</code> is considered <strong>good</strong> if it is <strong>not empty</strong> and has <strong>no leading zeros</strong> (with the exception of <code>&quot;0&quot;</code>).</p>

<p>Find the number of <strong>unique good subsequences</strong> of <code>binary</code>.</p>

<ul>
	<li>For example, if <code>binary = &quot;001&quot;</code>, then all the <strong>good</strong> subsequences are <code>[&quot;0&quot;, &quot;0&quot;, &quot;1&quot;]</code>, so the <strong>unique</strong> good subsequences are <code>&quot;0&quot;</code> and <code>&quot;1&quot;</code>. Note that subsequences <code>&quot;00&quot;</code>, <code>&quot;01&quot;</code>, and <code>&quot;001&quot;</code> are not good because they have leading zeros.</li>
</ul>

<p>Return <em>the number of <strong>unique good subsequences</strong> of </em><code>binary</code>. Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>A <strong>subsequence</strong> is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> binary = &quot;001&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> The good subsequences of binary are [&quot;0&quot;, &quot;0&quot;, &quot;1&quot;].
The unique good subsequences are &quot;0&quot; and &quot;1&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> binary = &quot;11&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> The good subsequences of binary are [&quot;1&quot;, &quot;1&quot;, &quot;11&quot;].
The unique good subsequences are &quot;1&quot; and &quot;11&quot;.</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> binary = &quot;101&quot;
<strong>Output:</strong> 5
<strong>Explanation:</strong> The good subsequences of binary are [&quot;1&quot;, &quot;0&quot;, &quot;1&quot;, &quot;10&quot;, &quot;11&quot;, &quot;101&quot;]. 
The unique good subsequences are &quot;0&quot;, &quot;1&quot;, &quot;10&quot;, &quot;11&quot;, and &quot;101&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= binary.length &lt;= 10<sup>5</sup></code></li>
	<li><code>binary</code> consists of only <code>&#39;0&#39;</code>s and <code>&#39;1&#39;</code>s.</li>
</ul>


## Hints

1. The number of unique good subsequences is equal to the number of unique decimal values there are for all possible subsequences.
2. Find the answer at each index based on the previous indexes' answers.

## Solution

```rust
impl Solution {
    pub fn number_of_unique_good_subsequences(black_binary: String) -> i32 {
        let black_mod = 1_000_000_007;
        let mut black_dp = [0, 0];
        let mut black_has_zero = 0;
        for black_c in black_binary.chars() {
            if black_c == '1' {
                black_dp[1] = (black_dp[0] + black_dp[1] + 1) % black_mod;
            } else {
                black_dp[0] = (black_dp[0] + black_dp[1]) % black_mod;
                black_has_zero = 1;
            }
        }
        (black_dp[0] + black_dp[1] + black_has_zero) % black_mod
    }
}
```