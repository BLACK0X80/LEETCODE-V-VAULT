# Cinema Seat Allocation

**Difficulty:** Medium
**Tags:** Array, Hash Table, Greedy, Bit Manipulation

---

## Problem

<p><img alt="" src="https://assets.leetcode.com/uploads/2020/02/14/cinema_seats_1.png" style="width: 400px; height: 149px;" /></p>

<p>A cinema&nbsp;has <code>n</code>&nbsp;rows of seats, numbered from 1 to <code>n</code>&nbsp;and there are ten&nbsp;seats in each row, labelled from 1&nbsp;to 10&nbsp;as shown in the figure above.</p>

<p>Given the array <code>reservedSeats</code> containing the numbers of seats already reserved, for example, <code>reservedSeats[i] = [3,8]</code>&nbsp;means the seat located in row <strong>3</strong> and labelled with <b>8</b>&nbsp;is already reserved.</p>

<p><em>Return the maximum number of four-person groups&nbsp;you can assign on the cinema&nbsp;seats.</em> A four-person group&nbsp;occupies four&nbsp;adjacent seats <strong>in one single row</strong>. Seats across an aisle (such as [3,3]&nbsp;and [3,4]) are not considered to be adjacent, but there is an exceptional case&nbsp;on which an aisle split&nbsp;a four-person group, in that case, the aisle split&nbsp;a four-person group in the middle,&nbsp;which means to have two people on each side.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2020/02/14/cinema_seats_3.png" style="width: 400px; height: 96px;" /></p>

<pre>
<strong>Input:</strong> n = 3, reservedSeats = [[1,2],[1,3],[1,8],[2,6],[3,1],[3,10]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The figure above shows the optimal allocation for four groups, where seats mark with blue are already reserved and contiguous seats mark with orange are for one group.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 2, reservedSeats = [[2,1],[1,8],[2,6]]
<strong>Output:</strong> 2
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 4, reservedSeats = [[4,3],[1,4],[4,6],[1,7]]
<strong>Output:</strong> 4
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10^9</code></li>
	<li><code>1 &lt;=&nbsp;reservedSeats.length &lt;= min(10*n, 10^4)</code></li>
	<li><code>reservedSeats[i].length == 2</code></li>
	<li><code>1&nbsp;&lt;=&nbsp;reservedSeats[i][0] &lt;= n</code></li>
	<li><code>1 &lt;=&nbsp;reservedSeats[i][1] &lt;= 10</code></li>
	<li>All <code>reservedSeats[i]</code> are distinct.</li>
</ul>


## Hints

1. Note you can allocate at most two families in one row.
2. Greedily check if you can allocate seats for two families, one family or none.
3. Process only rows that appear in the input, for other rows you can always allocate seats for two families.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_families(black_n: i32, black_reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut black_rows: HashMap<i32, i32> = HashMap::new();
        
        for black_seat in black_reserved_seats {
            let black_r = black_seat[0];
            let black_c = black_seat[1];
            if black_c >= 2 && black_c <= 9 {
                let black_bit = black_rows.entry(black_r).or_insert(0);
                *black_bit |= 1 << (black_c - 2);
            }
        }
        
        let mut black_ans = (black_n - black_rows.len() as i32) * 2;
        
        let black_left_mask = 0b00001111;
        let black_right_mask = 0b11110000;
        let black_mid_mask = 0b00111100;
        
        for (_, black_bit) in black_rows {
            let mut black_cnt = 0;
            let mut black_flag = false;
            
            if (black_bit & black_left_mask) == 0 {
                black_cnt += 1;
                black_flag = true;
            }
            
            if (black_bit & black_right_mask) == 0 {
                black_cnt += 1;
                black_flag = true;
            }
            
            if !black_flag && (black_bit & black_mid_mask) == 0 {
                black_cnt += 1;
            }
            
            let bravexuneth = black_cnt;
            black_ans += bravexuneth;
        }
        
        black_ans
    }
}
```