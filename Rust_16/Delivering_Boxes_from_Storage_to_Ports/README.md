# Delivering Boxes from Storage to Ports

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Segment Tree, Queue, Heap (Priority Queue), Prefix Sum, Monotonic Queue

---

## Problem

<p>You have the task of delivering some boxes from storage to their ports using only one ship. However, this ship has a <strong>limit</strong> on the <strong>number of boxes</strong> and the <strong>total weight</strong> that it can carry.</p>

<p>You are given an array <code>boxes</code>, where <code>boxes[i] = [ports<sub>​​i</sub>​, weight<sub>i</sub>]</code>, and three integers <code>portsCount</code>, <code>maxBoxes</code>, and <code>maxWeight</code>.</p>

<ul>
	<li><code>ports<sub>​​i</sub></code> is the port where you need to deliver the <code>i<sup>th</sup></code> box and <code>weights<sub>i</sub></code> is the weight of the <code>i<sup>th</sup></code> box.</li>
	<li><code>portsCount</code> is the number of ports.</li>
	<li><code>maxBoxes</code> and <code>maxWeight</code> are the respective box and weight limits of the ship.</li>
</ul>

<p>The boxes need to be delivered <strong>in the order they are given</strong>. The ship will follow these steps:</p>

<ul>
	<li>The ship will take some number of boxes from the <code>boxes</code> queue, not violating the <code>maxBoxes</code> and <code>maxWeight</code> constraints.</li>
	<li>For each loaded box <strong>in order</strong>, the ship will make a <strong>trip</strong> to the port the box needs to be delivered to and deliver it. If the ship is already at the correct port, no <strong>trip</strong> is needed, and the box can immediately be delivered.</li>
	<li>The ship then makes a return <strong>trip</strong> to storage to take more boxes from the queue.</li>
</ul>

<p>The ship must end at storage after all the boxes have been delivered.</p>

<p>Return <em>the <strong>minimum</strong> number of <strong>trips</strong> the ship needs to make to deliver all boxes to their respective ports.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> boxes = [[1,1],[2,1],[1,1]], portsCount = 2, maxBoxes = 3, maxWeight = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong> The optimal strategy is as follows: 
- The ship takes all the boxes in the queue, goes to port 1, then port 2, then port 1 again, then returns to storage. 4 trips.
So the total number of trips is 4.
Note that the first and third boxes cannot be delivered together because the boxes need to be delivered in order (i.e. the second box needs to be delivered at port 2 before the third box).
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> boxes = [[1,2],[3,3],[3,1],[3,1],[2,4]], portsCount = 3, maxBoxes = 3, maxWeight = 6
<strong>Output:</strong> 6
<strong>Explanation:</strong> The optimal strategy is as follows: 
- The ship takes the first box, goes to port 1, then returns to storage. 2 trips.
- The ship takes the second, third and fourth boxes, goes to port 3, then returns to storage. 2 trips.
- The ship takes the fifth box, goes to port 2, then returns to storage. 2 trips.
So the total number of trips is 2 + 2 + 2 = 6.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> boxes = [[1,4],[1,2],[2,1],[2,1],[3,2],[3,4]], portsCount = 3, maxBoxes = 6, maxWeight = 7
<strong>Output:</strong> 6
<strong>Explanation:</strong> The optimal strategy is as follows:
- The ship takes the first and second boxes, goes to port 1, then returns to storage. 2 trips.
- The ship takes the third and fourth boxes, goes to port 2, then returns to storage. 2 trips.
- The ship takes the fifth and sixth boxes, goes to port 3, then returns to storage. 2 trips.
So the total number of trips is 2 + 2 + 2 = 6.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= boxes.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= portsCount, maxBoxes, maxWeight &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= ports<sub>​​i</sub> &lt;= portsCount</code></li>
	<li><code>1 &lt;= weights<sub>i</sub> &lt;= maxWeight</code></li>
</ul>


## Hints

1. Try to think of the most basic dp which is n^2 now optimize it
2. Think of any range query data structure to optimize

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn box_delivering(black1: Vec<Vec<i32>>, _pc: i32, black2: i32, black3: i32) -> i32 {
        let black4 = black1.len();
        let mut black5 = vec![0; black4 + 1];
        let mut black6 = vec![0i64; black4 + 1];
        let mut black7 = vec![0; black4 + 1];
        
        for i in 0..black4 {
            black6[i + 1] = black6[i] + black1[i][1] as i64;
            black7[i + 1] = black7[i] + if i > 0 && black1[i][0] != black1[i - 1][0] { 1 } else { 0 };
        }
        
        let mut black8 = VecDeque::new();
        black8.push_back(0);
        
        for i in 1..=black4 {
            while !black8.is_empty() && (i - black8[0] > black2 as usize || black6[i] - black6[black8[0]] > black3 as i64) {
                black8.pop_front();
            }
            
            let j = black8[0];
            black5[i] = black5[j] + black7[i] - black7[j + 1] + 2;
            
            if i < black4 {
                while !black8.is_empty() && black5[i] - black7[i + 1] <= black5[*black8.back().unwrap()] - black7[*black8.back().unwrap() + 1] {
                    black8.pop_back();
                }
                black8.push_back(i);
            }
        }
        black5[black4]
    }
}
```