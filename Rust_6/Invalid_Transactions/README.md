# Invalid Transactions

**Difficulty:** Medium
**Tags:** Array, Hash Table, String, Sorting

---

## Problem

<p>A transaction is possibly invalid if:</p>

<ul>
	<li>the amount exceeds <code>$1000</code>, or;</li>
	<li>if it occurs within (and including) <code>60</code> minutes of another transaction with the <strong>same name</strong> in a <strong>different city</strong>.</li>
</ul>

<p>You are given an array of strings <code>transaction</code> where <code>transactions[i]</code> consists of comma-separated values representing the name, time (in minutes), amount, and city of the transaction.</p>

<p>Return a list of <code>transactions</code> that are possibly invalid. You may return the answer in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> transactions = [&quot;alice,20,800,mtv&quot;,&quot;alice,50,100,beijing&quot;]
<strong>Output:</strong> [&quot;alice,20,800,mtv&quot;,&quot;alice,50,100,beijing&quot;]
<strong>Explanation:</strong> The first transaction is invalid because the second transaction occurs within a difference of 60 minutes, have the same name and is in a different city. Similarly the second one is invalid too.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> transactions = [&quot;alice,20,800,mtv&quot;,&quot;alice,50,1200,mtv&quot;]
<strong>Output:</strong> [&quot;alice,50,1200,mtv&quot;]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> transactions = [&quot;alice,20,800,mtv&quot;,&quot;bob,50,1200,mtv&quot;]
<strong>Output:</strong> [&quot;bob,50,1200,mtv&quot;]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>transactions.length &lt;= 1000</code></li>
	<li>Each <code>transactions[i]</code> takes the form <code>&quot;{name},{time},{amount},{city}&quot;</code></li>
	<li>Each <code>{name}</code> and <code>{city}</code> consist of lowercase English letters, and have lengths between <code>1</code> and <code>10</code>.</li>
	<li>Each <code>{time}</code> consist of digits, and represent an integer between <code>0</code> and <code>1000</code>.</li>
	<li>Each <code>{amount}</code> consist of digits, and represent an integer between <code>0</code> and <code>2000</code>.</li>
</ul>


## Hints

1. Split each string into four arrays.
2. For each transaction check if it's invalid, you can do this with just a loop with help of the four arrays generated on step 1.
3. At the end you perform O(N ^ 2) operations.

## Solution

```rust
impl Solution { pub fn invalid_transactions(black_t: Vec<String>) -> Vec<String> { struct BlackTx { black_s: String, black_n: String, black_t: i32, black_a: i32, black_c: String } let black_list: Vec<BlackTx> = black_t.iter().map(|black_s| { let black_v: Vec<&str> = black_s.split(',').collect(); BlackTx { black_s: black_s.clone(), black_n: black_v[0].to_string(), black_t: black_v[1].parse().unwrap(), black_a: black_v[2].parse().unwrap(), black_c: black_v[3].to_string() } }).collect(); let mut black_inv = vec![false; black_list.len()]; for black_i in 0..black_list.len() { if black_list[black_i].black_a > 1000 { black_inv[black_i] = true; } for black_j in 0..black_list.len() { if black_list[black_i].black_n == black_list[black_j].black_n && black_list[black_i].black_c != black_list[black_j].black_c && (black_list[black_i].black_t - black_list[black_j].black_t).abs() <= 60 { black_inv[black_i] = true; black_inv[black_j] = true; } } } black_list.into_iter().enumerate().filter(|(black_i, _)| black_inv[*black_i]).map(|(_, black_tx)| black_tx.black_s).collect() } }
```