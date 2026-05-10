# Heaters

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Binary Search, Sorting

---

## Problem

<p>Winter is coming! During the contest, your first job is to design a standard heater with a fixed warm radius to warm all the houses.</p>

<p>Every house can be warmed, as long as the house is within the heater&#39;s warm radius range.&nbsp;</p>

<p>Given the positions of <code>houses</code> and <code>heaters</code> on a horizontal line, return <em>the minimum radius standard of heaters&nbsp;so that those heaters could cover all houses.</em></p>

<p><strong>Notice</strong> that&nbsp;all the <code>heaters</code> follow your radius standard, and the warm radius will be the same.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> houses = [1,2,3], heaters = [2]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only heater was placed in the position 2, and if we use the radius 1 standard, then all the houses can be warmed.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> houses = [1,2,3,4], heaters = [1,4]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The two heaters were placed at positions 1 and 4. We need to use a radius 1 standard, then all the houses can be warmed.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> houses = [1,5], heaters = [2]
<strong>Output:</strong> 3
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= houses.length, heaters.length &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= houses[i], heaters[i] &lt;= 10<sup>9</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn find_radius(mut black_h: Vec<i32>, mut black_ht: Vec<i32>) -> i32 { black_ht.sort_unstable(); black_h.iter().map(|&black_house| { match black_ht.binary_search(&black_house) { Ok(_) => 0, Err(idx) => { let black_d1 = if idx > 0 { black_house - black_ht[idx-1] } else { i32::MAX }; let black_d2 = if idx < black_ht.len() { black_ht[idx] - black_house } else { i32::MAX }; black_d1.min(black_d2) } } }).max().unwrap() } }
```