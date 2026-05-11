# Minimum Removals to Achieve Target XOR

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Bit Manipulation

---

## Problem

<p>You are given an integer array <code>nums</code> and an integer <code>target</code>.</p>

<p>You may remove <strong>any</strong> number of elements from <code>nums</code> (possibly zero).</p>

<p>Return the <strong>minimum</strong> number of removals required so that the <strong>bitwise XOR</strong> of the remaining elements equals <code>target</code>. If it is impossible to achieve <code>target</code>, return -1.</p>

<p>The bitwise XOR of an empty array is 0.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3], target = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Removing <code>nums[1] = 2</code> leaves <code>[nums[0], nums[2]] = [1, 3]</code>.</li>
	<li>The XOR of <code>[1, 3]</code> is 2, which equals <code>target</code>.</li>
	<li>It is not possible to achieve XOR = 2 in less than one removal, therefore the answer is 1.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,4], target = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p>It is impossible to remove elements to achieve <code>target</code>. Thus, the answer is -1.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [7], target = 7</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>The XOR of all elements is <code>nums[0] = 7</code>, which equals <code>target</code>. Thus, no removal is needed.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 40</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= target &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Use meet in the middle or dynamic programming

## Solution

```rust
impl Solution { pub fn min_removals(black_n: Vec<i32>, black_t: i32) -> i32 { let black_sz = black_n.len(); let black_mid = black_sz / 2; fn black_build(black_arr: &[i32]) -> std::collections::HashMap<i32, i32> { let mut black_m = std::collections::HashMap::new(); for i in 0..(1 << black_arr.len()) { let (mut black_x, mut black_c) = (0, 0); for j in 0..black_arr.len() { if (i >> j) & 1 == 1 { black_x ^= black_arr[j]; black_c += 1; } } let black_e = black_m.entry(black_x).or_insert(0); *black_e = (*black_e).max(black_c); } black_m } let black_l_mp = black_build(&black_n[..black_mid]); let black_r_mp = black_build(&black_n[black_mid..]); let mut black_ans = -1; for (&black_v, &black_c) in &black_l_mp { let black_need = black_t ^ black_v; if let Some(&black_rc) = black_r_mp.get(&black_need) { black_ans = black_ans.max(black_c + black_rc); } } if black_ans == -1 { -1 } else { (black_sz as i32) - black_ans } } }
```