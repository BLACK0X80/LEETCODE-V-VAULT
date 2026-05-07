# Sum of Largest Prime Substrings

**Difficulty:** Medium
**Tags:** Hash Table, Math, String, Sorting, Number Theory

---

## Problem

<p data-end="157" data-start="30">Given a string <code>s</code>, find the sum of the <strong>3 largest unique <span data-keyword="prime-number">prime numbers</span></strong> that can be formed using any of its<strong> <span data-keyword="substring">substrings</span></strong>.</p>

<p data-end="269" data-start="166">Return the <strong>sum</strong> of the three largest unique prime numbers that can be formed. If fewer than three exist, return the sum of <strong>all</strong> available primes. If no prime numbers can be formed, return 0.</p>

<p data-end="370" data-is-last-node="" data-is-only-node="" data-start="271"><strong data-end="280" data-start="271">Note:</strong> Each prime number should be counted only <strong>once</strong>, even if it appears in <strong>multiple</strong> substrings. Additionally, when converting a substring to an integer, any leading zeros are ignored.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;12234&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">1469</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li data-end="136" data-start="16">The unique prime numbers formed from the substrings of <code>&quot;12234&quot;</code> are 2, 3, 23, 223, and 1223.</li>
	<li data-end="226" data-start="137">The 3 largest primes are 1223, 223, and 23. Their sum is 1469.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;111&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">11</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li data-end="339" data-start="244">The unique prime number formed from the substrings of <code>&quot;111&quot;</code> is 11.</li>
	<li data-end="412" data-is-last-node="" data-start="340">Since there is only one prime number, the sum is 11.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li data-end="39" data-start="18"><code>1 &lt;= s.length &lt;= 10</code></li>
	<li data-end="68" data-is-last-node="" data-start="40"><code>s</code> consists of only digits.</li>
</ul>


## Hints

1. Iterate over all substrings of <code>s</code> to generate candidate numbers.
2. Check each candidate for primality in <code>O(sqrt(n))</code> time.
3. Store unique primes, then sum the three largest (or all if fewer than three).

## Solution

```rust
impl Solution { pub fn sum_of_largest_primes(s: String) -> i64 { let mut black_p = std::collections::HashSet::new(); let is_prime = |n: i64| n > 1 && (2..=((n as f64).sqrt() as i64)).all(|i| n % i != 0); for i in 0..s.len() { for j in i + 1..=s.len() { if let Ok(n) = s[i..j].parse::<i64>() { if is_prime(n) { black_p.insert(n); } } } } let mut black_v: Vec<_> = black_p.into_iter().collect(); black_v.sort_unstable_by(|a, b| b.cmp(a)); black_v.iter().take(3).sum() } }
```