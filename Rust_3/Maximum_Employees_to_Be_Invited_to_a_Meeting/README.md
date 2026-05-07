# Maximum Employees to Be Invited to a Meeting

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Depth-First Search, Graph Theory, Topological Sort

---

## Problem

<p>A company is organizing a meeting and has a list of <code>n</code> employees, waiting to be invited. They have arranged for a large <strong>circular</strong> table, capable of seating <strong>any number</strong> of employees.</p>

<p>The employees are numbered from <code>0</code> to <code>n - 1</code>. Each employee has a <strong>favorite</strong> person and they will attend the meeting <strong>only if</strong> they can sit next to their favorite person at the table. The favorite person of an employee is <strong>not</strong> themself.</p>

<p>Given a <strong>0-indexed</strong> integer array <code>favorite</code>, where <code>favorite[i]</code> denotes the favorite person of the <code>i<sup>th</sup></code> employee, return <em>the <strong>maximum number of employees</strong> that can be invited to the meeting</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/14/ex1.png" style="width: 236px; height: 195px;" />
<pre>
<strong>Input:</strong> favorite = [2,2,1,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
The above figure shows how the company can invite employees 0, 1, and 2, and seat them at the round table.
All employees cannot be invited because employee 2 cannot sit beside employees 0, 1, and 3, simultaneously.
Note that the company can also invite employees 1, 2, and 3, and give them their desired seats.
The maximum number of employees that can be invited to the meeting is 3. 
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> favorite = [1,2,0]
<strong>Output:</strong> 3
<strong>Explanation:</strong> 
Each employee is the favorite person of at least one other employee, and the only way the company can invite them is if they invite every employee.
The seating arrangement will be the same as that in the figure given in example 1:
- Employee 0 will sit between employees 2 and 1.
- Employee 1 will sit between employees 0 and 2.
- Employee 2 will sit between employees 1 and 0.
The maximum number of employees that can be invited to the meeting is 3.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/14/ex2.png" style="width: 219px; height: 220px;" />
<pre>
<strong>Input:</strong> favorite = [3,0,1,4,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong>
The above figure shows how the company will invite employees 0, 1, 3, and 4, and seat them at the round table.
Employee 2 cannot be invited because the two spots next to their favorite employee 1 are taken.
So the company leaves them out of the meeting.
The maximum number of employees that can be invited to the meeting is 4.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == favorite.length</code></li>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= favorite[i] &lt;=&nbsp;n - 1</code></li>
	<li><code>favorite[i] != i</code></li>
</ul>


## Hints

1. From the given array favorite, create a graph where for every index i, there is a directed edge from favorite[i] to i. The graph will be a combination of cycles and chains of acyclic edges. Now, what are the ways in which we can choose employees to sit at the table?
2. The first way by which we can choose employees is by selecting a cycle of the graph. It can be proven that in this case, the employees that do not lie in the cycle can never be seated at the table (unless the cycle has a length of 2).
3. The second way is by combining acyclic chains. At most two chains can be combined by a cycle of length 2, where each chain ends on one of the employees in the cycle.

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn maximum_invitations(black1: Vec<i32>) -> i32 {
        let black2 = black1.len();
        let mut black3 = vec![0; black2];
        for &black4 in &black1 {
            black3[black4 as usize] += 1;
        }

        let mut black5 = VecDeque::new();
        for black6 in 0..black2 {
            if black3[black6] == 0 {
                black5.push_back(black6);
            }
        }

        let mut black7 = vec![1; black2];
        let mut black8 = vec![false; black2];
        while let Some(black9) = black5.pop_front() {
            black8[black9] = true;
            let black10 = black1[black9] as usize;
            black7[black10] = black7[black10].max(black7[black9] + 1);
            black3[black10] -= 1;
            if black3[black10] == 0 {
                black5.push_back(black10);
            }
        }

        let mut black11 = 0;
        let mut black12 = 0;

        for black13 in 0..black2 {
            if !black8[black13] {
                let mut black14 = 0;
                let mut black15 = black13;
                while !black8[black15] {
                    black8[black15] = true;
                    black15 = black1[black15] as usize;
                    black14 += 1;
                }

                if black14 == 2 {
                    black12 += black7[black13] + black7[black1[black13] as usize];
                } else {
                    black11 = black11.max(black14);
                }
            }
        }

        black11.max(black12)
    }
}
```