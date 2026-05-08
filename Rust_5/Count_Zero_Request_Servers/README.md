# Count Zero Request Servers

**Difficulty:** Medium
**Tags:** Array, Hash Table, Sliding Window, Sorting

---

## Problem

<p>You are given an integer <code>n</code> denoting the total number of servers and a <strong>2D</strong> <strong>0-indexed </strong>integer array <code>logs</code>, where <code>logs[i] = [server_id, time]</code> denotes that the server with id <code>server_id</code> received a request at time <code>time</code>.</p>

<p>You are also given an integer <code>x</code> and a <strong>0-indexed</strong> integer array <code>queries</code>.</p>

<p>Return <em>a <strong>0-indexed</strong> integer array</em> <code>arr</code> <em>of length</em> <code>queries.length</code> <em>where</em> <code>arr[i]</code> <em>represents the number of servers that <strong>did not receive</strong> any requests during the time interval</em> <code>[queries[i] - x, queries[i]]</code>.</p>

<p>Note that the time intervals are inclusive.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 3, logs = [[1,3],[2,6],[1,5]], x = 5, queries = [10,11]
<strong>Output:</strong> [1,2]
<strong>Explanation:</strong> 
For queries[0]: The servers with ids 1 and 2 get requests in the duration of [5, 10]. Hence, only server 3 gets zero requests.
For queries[1]: Only the server with id 2 gets a request in duration of [6,11]. Hence, the servers with ids 1 and 3 are the only servers that do not receive any requests during that time period.

</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 3, logs = [[2,4],[2,1],[1,2],[3,1]], x = 2, queries = [3,4]
<strong>Output:</strong> [0,1]
<strong>Explanation:</strong> 
For queries[0]: All servers get at least one request in the duration of [1, 3].
For queries[1]: Only server with id 3 gets no request in the duration [2,4].

</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= logs.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code><font face="monospace">logs[i].length == 2</font></code></li>
	<li><code>1 &lt;= logs[i][0] &lt;= n</code></li>
	<li><code>1 &lt;= logs[i][1] &lt;= 10<sup>6</sup></code></li>
	<li><code>1 &lt;= x &lt;= 10<sup>5</sup></code></li>
	<li><code>x &lt;&nbsp;queries[i]&nbsp;&lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Can we use sorting and two-pointer approach here?
2. Sort the queries array and logs array based on time in increasing order.
3. For every window of size x, use sliding window and two-pointer approach to find the answer to the queries.

## Solution

```rust
impl Solution { pub fn count_servers(black_n: i32, mut black_l: Vec<Vec<i32>>, black_x: i32, black_q: Vec<i32>) -> Vec<i32> { let mut black_res = vec![0; black_q.len()]; let mut black_qs: Vec<_> = black_q.into_iter().enumerate().collect(); black_qs.sort_unstable_by_key(|x| x.1); black_l.sort_unstable_by_key(|x| x[1]); let (mut black_cnt, mut black_m, mut black_i, mut black_j) = (0, std::collections::HashMap::new(), 0, 0); for (black_idx, black_time) in black_qs { while black_j < black_l.len() && black_l[black_j][1] <= black_time { *black_m.entry(black_l[black_j][0]).or_insert(0) += 1; if black_m[&black_l[black_j][0]] == 1 { black_cnt += 1; } black_j += 1; } while black_i < black_l.len() && black_l[black_i][1] < black_time - black_x { *black_m.get_mut(&black_l[black_i][0]).unwrap() -= 1; if black_m[&black_l[black_i][0]] == 0 { black_cnt -= 1; } black_i += 1; } black_res[black_idx] = black_n - black_cnt; } black_res } }
```