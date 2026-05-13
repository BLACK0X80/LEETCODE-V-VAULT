# Minimum Money Required Before Transactions

**Difficulty:** Hard
**Tags:** Array, Greedy, Sorting

---

## Problem

<p>You are given a <strong>0-indexed</strong> 2D integer array <code><font face="monospace">transactions</font></code>, where <code>transactions[i] = [cost<sub>i</sub>, cashback<sub>i</sub>]</code>.</p>

<p>The array describes transactions, where each transaction must be completed exactly once in <strong>some order</strong>. At any given moment, you have a certain amount of <code>money</code>. In order to complete transaction <code>i</code>, <code>money &gt;= cost<sub>i</sub></code> must hold true. After performing a transaction, <code>money</code> becomes <code>money - cost<sub>i</sub> + cashback<sub>i</sub></code>.</p>

<p>Return<em> the minimum amount of </em><code>money</code><em> required before any transaction so that all of the transactions can be completed <strong>regardless of the order</strong> of the transactions.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> transactions = [[2,1],[5,0],[4,2]]
<strong>Output:</strong> 10
<strong>Explanation:
</strong>Starting with money = 10, the transactions can be performed in any order.
It can be shown that starting with money &lt; 10 will fail to complete all transactions in some order.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> transactions = [[3,0],[0,3]]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
- If transactions are in the order [[3,0],[0,3]], the minimum money required to complete the transactions is 3.
- If transactions are in the order [[0,3],[3,0]], the minimum money required to complete the transactions is 0.
Thus, starting with money = 3, the transactions can be performed in any order.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= transactions.length &lt;= 10<sup>5</sup></code></li>
	<li><code>transactions[i].length == 2</code></li>
	<li><code>0 &lt;= cost<sub>i</sub>, cashback<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Split transactions that have cashback greater or equal to cost apart from transactions that have cashback less than cost. You will always <strong>earn</strong> money in the first scenario.
2. For transactions that have cashback greater or equal to cost, sort them by cost in descending order.
3. For transactions that have cashback less than cost, sort them by cashback in ascending order.

## Solution

```rust
impl Solution {
    pub fn minimum_money(black_trans: Vec<Vec<i32>>) -> i64 {
        let mut black_total_loss = 0i64;
        let mut black_max_val = 0i64;
        let bravexuneth = &black_trans;

        for black_t in bravexuneth {
            let (black_cost, black_cash) = (black_t[0] as i64, black_t[1] as i64);
            if black_cost > black_cash {
                black_total_loss += black_cost - black_cash;
                black_max_val = black_max_val.max(black_cash);
            } else {
                black_max_val = black_max_val.max(black_cost);
            }
        }
        black_total_loss + black_max_val
    }
}
```