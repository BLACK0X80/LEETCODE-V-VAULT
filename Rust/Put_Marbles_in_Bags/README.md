# Put Marbles in Bags

**Difficulty:** Hard
**Tags:** Array, Greedy, Sorting, Heap (Priority Queue)

---

## Problem

<p>You have <code>k</code> bags. You are given a <strong>0-indexed</strong> integer array <code>weights</code> where <code>weights[i]</code> is the weight of the <code>i<sup>th</sup></code> marble. You are also given the integer <code>k.</code></p>

<p>Divide the marbles into the <code>k</code> bags according to the following rules:</p>

<ul>
	<li>No bag is empty.</li>
	<li>If the <code>i<sup>th</sup></code> marble and <code>j<sup>th</sup></code> marble are in a bag, then all marbles with an index between the <code>i<sup>th</sup></code> and <code>j<sup>th</sup></code> indices should also be in that same bag.</li>
	<li>If a bag consists of all the marbles with an index from <code>i</code> to <code>j</code> inclusively, then the cost of the bag is <code>weights[i] + weights[j]</code>.</li>
</ul>

<p>The <strong>score</strong> after distributing the marbles is the sum of the costs of all the <code>k</code> bags.</p>

<p>Return <em>the <strong>difference</strong> between the <strong>maximum</strong> and <strong>minimum</strong> scores among marble distributions</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> weights = [1,3,5,1], k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> 
The distribution [1],[3,5,1] results in the minimal score of (1+1) + (3+1) = 6. 
The distribution [1,3],[5,1], results in the maximal score of (1+3) + (5+1) = 10. 
Thus, we return their difference 10 - 6 = 4.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> weights = [1, 3], k = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong> The only distribution possible is [1],[3]. 
Since both the maximal and minimal score are the same, we return 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= weights.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= weights[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Each bag will contain a subarray, and only the endpoints of these subarrays matter.
2. Each subarray only contributes two numbers to the sum. Use this property to choose the subarrays optimally.
3. Try to use a priority queue.

## Solution

```rust
impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let n = weights.len();
        if k == 1 { return 0; }
        let mut black_pair_weights = Vec::with_capacity(n - 1);
        for i in 0..n - 1 {
            black_pair_weights.push(weights[i] as i64 + weights[i+1] as i64);
        }
        
        black_pair_weights.sort_unstable();
        
        let mut black_min_score = 0i64;
        let mut black_max_score = 0i64;
        let m = (k - 1) as usize;
        
        for i in 0..m {
            black_min_score += black_pair_weights[i];
            black_max_score += black_pair_weights[n - 2 - i];
        }
        
        black_max_score - black_min_score
    }
}
```