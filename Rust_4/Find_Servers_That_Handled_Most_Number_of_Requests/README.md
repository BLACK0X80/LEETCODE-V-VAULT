# Find Servers That Handled Most Number of Requests

**Difficulty:** Hard
**Tags:** Array, Heap (Priority Queue), Simulation, Ordered Set

---

## Problem

<p>You have <code>k</code> servers numbered from <code>0</code> to <code>k-1</code> that are being used to handle multiple requests simultaneously. Each server has infinite computational capacity but <strong>cannot handle more than one request at a time</strong>. The requests are assigned to servers according to a specific algorithm:</p>

<ul>
	<li>The <code>i<sup>th</sup></code> (0-indexed) request arrives.</li>
	<li>If all servers are busy, the request is dropped (not handled at all).</li>
	<li>If the <code>(i % k)<sup>th</sup></code> server is available, assign the request to that server.</li>
	<li>Otherwise, assign the request to the next available server (wrapping around the list of servers and starting from 0 if necessary). For example, if the <code>i<sup>th</sup></code> server is busy, try to assign the request to the <code>(i+1)<sup>th</sup></code> server, then the <code>(i+2)<sup>th</sup></code> server, and so on.</li>
</ul>

<p>You are given a <strong>strictly increasing</strong> array <code>arrival</code> of positive integers, where <code>arrival[i]</code> represents the arrival time of the <code>i<sup>th</sup></code> request, and another array <code>load</code>, where <code>load[i]</code> represents the load of the <code>i<sup>th</sup></code> request (the time it takes to complete). Your goal is to find the <strong>busiest server(s)</strong>. A server is considered <strong>busiest</strong> if it handled the most number of requests successfully among all the servers.</p>

<p>Return <em>a list containing the IDs (0-indexed) of the <strong>busiest server(s)</strong></em>. You may return the IDs in any order.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/08/load-1.png" style="width: 389px; height: 221px;" />
<pre>
<strong>Input:</strong> k = 3, arrival = [1,2,3,4,5], load = [5,2,3,3,3] 
<strong>Output:</strong> [1] 
<strong>Explanation:</strong> 
All of the servers start out available.
The first 3 requests are handled by the first 3 servers in order.
Request 3 comes in. Server 0 is busy, so it&#39;s assigned to the next available server, which is 1.
Request 4 comes in. It cannot be handled since all servers are busy, so it is dropped.
Servers 0 and 2 handled one request each, while server 1 handled two requests. Hence server 1 is the busiest server.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> k = 3, arrival = [1,2,3,4], load = [1,2,1,2]
<strong>Output:</strong> [0]
<strong>Explanation:</strong> 
The first 3 requests are handled by first 3 servers.
Request 3 comes in. It is handled by server 0 since the server is available.
Server 0 handled two requests, while servers 1 and 2 handled one request each. Hence server 0 is the busiest server.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> k = 3, arrival = [1,2,3], load = [10,12,11]
<strong>Output:</strong> [0,1,2]
<strong>Explanation:</strong> Each server handles a single request, so they are all considered the busiest.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= arrival.length, load.length &lt;= 10<sup>5</sup></code></li>
	<li><code>arrival.length == load.length</code></li>
	<li><code>1 &lt;= arrival[i], load[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>arrival</code> is <strong>strictly increasing</strong>.</li>
</ul>


## Hints

1. To speed up the next available server search, keep track of the available servers in a sorted structure such as an ordered set.
2. To determine if a server is available, keep track of the end times for each task in a heap and add the server to the available set once the soonest task ending time is less than or equal to the next task to add.

## Solution

```rust
use std::collections::{BTreeSet, BinaryHeap};
use std::cmp::Reverse;

impl Solution {
    pub fn busiest_servers(black1: i32, black2: Vec<i32>, black3: Vec<i32>) -> Vec<i32> {
        let mut black4 = vec![0; black1 as usize];
        let mut black5: BTreeSet<i32> = (0..black1).collect();
        let mut black6 = BinaryHeap::new();
        for black7 in 0..black2.len() {
            let black8 = black2[black7];
            while let Some(Reverse((black9, black10))) = black6.peek() {
                if *black9 <= black8 {
                    black5.insert(*black10);
                    black6.pop();
                } else { break; }
            }
            if black5.is_empty() { continue; }
            let mut black11 = black5.range((black7 as i32 % black1)..).next();
            if black11.is_none() { black11 = black5.iter().next(); }
            let &black12 = black11.unwrap();
            black4[black12 as usize] += 1;
            black6.push(Reverse((black8 + black3[black7], black12)));
            black5.remove(&black12);
        }
        let black13 = *black4.iter().max().unwrap();
        black4.iter().enumerate().filter(|&(_, &black14)| black14 == black13).map(|(black15, _)| black15 as i32).collect()
    }
}
```