# Avoid Flood in The City

**Difficulty:** Medium
**Tags:** Array, Hash Table, Binary Search, Greedy, Heap (Priority Queue)

---

## Problem

<p>Your country has 10<sup>9</sup> lakes. Initially, all the lakes are empty, but when it rains over the <code>n<sup>th</sup></code> lake, the <code>n<sup>th</sup></code> lake becomes full of water. If it rains over a lake that is <strong>full of water</strong>, there will be a <strong>flood</strong>. Your goal is to avoid floods in any lake.</p>

<p>Given an integer array <code>rains</code> where:</p>

<ul>
	<li><code>rains[i] &gt; 0</code> means there will be rains over the <code>rains[i]</code> lake.</li>
	<li><code>rains[i] == 0</code> means there are no rains this day and you&nbsp;<strong>must</strong> choose&nbsp;<strong>one lake</strong> this day and <strong>dry it</strong>.</li>
</ul>

<p>Return <em>an array <code>ans</code></em> where:</p>

<ul>
	<li><code>ans.length == rains.length</code></li>
	<li><code>ans[i] == -1</code> if <code>rains[i] &gt; 0</code>.</li>
	<li><code>ans[i]</code> is the lake you choose to dry in the <code>ith</code> day if <code>rains[i] == 0</code>.</li>
</ul>

<p>If there are multiple valid answers return <strong>any</strong> of them. If it is impossible to avoid flood return <strong>an empty array</strong>.</p>

<p>Notice that if you chose to dry a full lake, it becomes empty, but if you chose to dry an empty lake, nothing changes.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> rains = [1,2,3,4]
<strong>Output:</strong> [-1,-1,-1,-1]
<strong>Explanation:</strong> After the first day full lakes are [1]
After the second day full lakes are [1,2]
After the third day full lakes are [1,2,3]
After the fourth day full lakes are [1,2,3,4]
There&#39;s no day to dry any lake and there is no flood in any lake.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> rains = [1,2,0,0,2,1]
<strong>Output:</strong> [-1,-1,2,1,-1,-1]
<strong>Explanation:</strong> After the first day full lakes are [1]
After the second day full lakes are [1,2]
After the third day, we dry lake 2. Full lakes are [1]
After the fourth day, we dry lake 1. There is no full lakes.
After the fifth day, full lakes are [2].
After the sixth day, full lakes are [1,2].
It is easy that this scenario is flood-free. [-1,-1,1,2,-1,-1] is another acceptable scenario.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> rains = [1,2,0,1,2]
<strong>Output:</strong> []
<strong>Explanation:</strong> After the second day, full lakes are  [1,2]. We have to dry one lake in the third day.
After that, it will rain over lakes [1,2]. It&#39;s easy to prove that no matter which lake you choose to dry in the 3rd day, the other one will flood.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= rains.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= rains[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Keep An array of the last day there was rains over each city.
2. Keep an array of the days you can dry a lake when you face one.
3. When it rains over a lake, check the first possible day you can dry this lake and assign this day to this lake.

## Solution

```rust
use std::collections::{HashMap, BTreeSet};
impl Solution { pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> { let (mut black_res, mut black_dry, mut black_full) = (vec![1; rains.len()], BTreeSet::new(), HashMap::new()); for (black_i, &black_r) in rains.iter().enumerate() { if black_r > 0 { black_res[black_i] = -1; if let Some(&black_prev) = black_full.get(&black_r) { if let Some(&black_day) = black_dry.range(black_prev..).next() { black_res[black_day] = black_r; black_dry.remove(&black_day); } else { return vec![]; } } black_full.insert(black_r, black_i); } else { black_dry.insert(black_i); } } for black_i in black_dry { black_res[black_i] = 1; } black_res } }
```