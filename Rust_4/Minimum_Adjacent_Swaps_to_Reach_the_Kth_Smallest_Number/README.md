# Minimum Adjacent Swaps to Reach the Kth Smallest Number

**Difficulty:** Medium
**Tags:** Two Pointers, String, Greedy

---

## Problem

<p>You are given a string <code>num</code>, representing a large integer, and an integer <code>k</code>.</p>

<p>We call some integer <strong>wonderful</strong> if it is a <strong>permutation</strong> of the digits in <code>num</code> and is <strong>greater in value</strong> than <code>num</code>. There can be many wonderful integers. However, we only care about the <strong>smallest-valued</strong> ones.</p>

<ul>
	<li>For example, when <code>num = &quot;5489355142&quot;</code>:

	<ul>
		<li>The 1<sup>st</sup> smallest wonderful integer is <code>&quot;5489355214&quot;</code>.</li>
		<li>The 2<sup>nd</sup> smallest wonderful integer is <code>&quot;5489355241&quot;</code>.</li>
		<li>The 3<sup>rd</sup> smallest wonderful integer is <code>&quot;5489355412&quot;</code>.</li>
		<li>The 4<sup>th</sup> smallest wonderful integer is <code>&quot;5489355421&quot;</code>.</li>
	</ul>
	</li>
</ul>

<p>Return <em>the <strong>minimum number of adjacent digit swaps</strong> that needs to be applied to </em><code>num</code><em> to reach the </em><code>k<sup>th</sup></code><em><strong> smallest wonderful</strong> integer</em>.</p>

<p>The tests are generated in such a way that <code>k<sup>th</sup></code>&nbsp;smallest wonderful integer exists.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;5489355142&quot;, k = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong> The 4<sup>th</sup> smallest wonderful number is &quot;5489355421&quot;. To get this number:
- Swap index 7 with index 8: &quot;5489355<u>14</u>2&quot; -&gt; &quot;5489355<u>41</u>2&quot;
- Swap index 8 with index 9: &quot;54893554<u>12</u>&quot; -&gt; &quot;54893554<u>21</u>&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;11112&quot;, k = 4
<strong>Output:</strong> 4
<strong>Explanation:</strong> The 4<sup>th</sup> smallest wonderful number is &quot;21111&quot;. To get this number:
- Swap index 3 with index 4: &quot;111<u>12</u>&quot; -&gt; &quot;111<u>21</u>&quot;
- Swap index 2 with index 3: &quot;11<u>12</u>1&quot; -&gt; &quot;11<u>21</u>1&quot;
- Swap index 1 with index 2: &quot;1<u>12</u>11&quot; -&gt; &quot;1<u>21</u>11&quot;
- Swap index 0 with index 1: &quot;<u>12</u>111&quot; -&gt; &quot;<u>21</u>111&quot;
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;00123&quot;, k = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> The 1<sup>st</sup> smallest wonderful number is &quot;00132&quot;. To get this number:
- Swap index 3 with index 4: &quot;001<u>23</u>&quot; -&gt; &quot;001<u>32</u>&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= num.length &lt;= 1000</code></li>
	<li><code>1 &lt;= k &lt;= 1000</code></li>
	<li><code>num</code> only consists of digits.</li>
</ul>


## Hints

1. Find the next permutation of the given string k times.
2. Try to move each element to its correct position and calculate the number of steps.

## Solution

```rust
impl Solution { pub fn get_min_swaps(num: String, k: i32) -> i32 { let (mut black_target, mut black_res, black_s) = (num.as_bytes().to_vec(), 0, num.as_bytes().to_vec()); for _ in 0..k { let mut black_i = black_target.len() - 2; while black_target[black_i] >= black_target[black_i + 1] { black_i -= 1; } let mut black_j = black_target.len() - 1; while black_target[black_j] <= black_target[black_i] { black_j -= 1; } black_target.swap(black_i, black_j); black_target[black_i + 1..].reverse(); } let mut black_curr = black_s; for black_i in 0..black_curr.len() { if black_curr[black_i] != black_target[black_i] { let mut black_j = black_i + 1; while black_curr[black_j] != black_target[black_i] { black_j += 1; } while black_j > black_i { black_curr.swap(black_j, black_j - 1); black_res += 1; black_j -= 1; } } } black_res } }
```