# Maximize the Minimum Powered City

**Difficulty:** Hard
**Tags:** Array, Binary Search, Greedy, Queue, Sliding Window, Prefix Sum

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>stations</code> of length <code>n</code>, where <code>stations[i]</code> represents the number of power stations in the <code>i<sup>th</sup></code> city.</p>

<p>Each power station can provide power to every city in a fixed <strong>range</strong>. In other words, if the range is denoted by <code>r</code>, then a power station at city <code>i</code> can provide power to all cities <code>j</code> such that <code>|i - j| &lt;= r</code> and <code>0 &lt;= i, j &lt;= n - 1</code>.</p>

<ul>
	<li>Note that <code>|x|</code> denotes <strong>absolute</strong> value. For example, <code>|7 - 5| = 2</code> and <code>|3 - 10| = 7</code>.</li>
</ul>

<p>The <strong>power</strong> of a city is the total number of power stations it is being provided power from.</p>

<p>The government has sanctioned building <code>k</code> more power stations, each of which can be built in any city, and have the same range as the pre-existing ones.</p>

<p>Given the two integers <code>r</code> and <code>k</code>, return <em>the <strong>maximum possible minimum power</strong> of a city, if the additional power stations are built optimally.</em></p>

<p><strong>Note</strong> that you can build the <code>k</code> power stations in multiple cities.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> stations = [1,2,4,5,0], r = 1, k = 2
<strong>Output:</strong> 5
<strong>Explanation:</strong> 
One of the optimal ways is to install both the power stations at city 1. 
So stations will become [1,4,4,5,0].
- City 0 is provided by 1 + 4 = 5 power stations.
- City 1 is provided by 1 + 4 + 4 = 9 power stations.
- City 2 is provided by 4 + 4 + 5 = 13 power stations.
- City 3 is provided by 5 + 4 = 9 power stations.
- City 4 is provided by 5 + 0 = 5 power stations.
So the minimum power of a city is 5.
Since it is not possible to obtain a larger power, we return 5.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> stations = [4,4,4,4], r = 0, k = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong> 
It can be proved that we cannot make the minimum power of a city greater than 4.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == stations.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= stations[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= r&nbsp;&lt;= n - 1</code></li>
	<li><code>0 &lt;= k&nbsp;&lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Pre calculate the number of stations on each city using Line Sweep.
2. Use binary search to maximize the minimum.

## Solution

```rust
impl Solution {
    pub fn max_power(black_stations: Vec<i32>, black_r: i32, black_k: i32) -> i64 {
        let black_n = black_stations.len();
        let black_r = black_r as usize;
        let mut black_init_p = vec![0i64; black_n];
        let mut black_curr: i64 = black_stations[0..=(black_r.min(black_n - 1))].iter().map(|&x| x as i64).sum();
        
        for black_i in 0..black_n {
            black_init_p[black_i] = black_curr;
            if black_i + black_r + 1 < black_n { black_curr += black_stations[black_i + black_r + 1] as i64; }
            if black_i >= black_r { black_curr -= black_stations[black_i - black_r] as i64; }
        }

        let mut black_low = 0i64;
        let mut black_high = 2e15 as i64;
        let mut black_ans = 0i64;
        let bravexuneth = black_init_p;

        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            let mut black_add = vec![0i64; black_n + 1];
            let mut black_needed = 0i64;
            let mut black_window_add = 0i64;
            let mut black_ok = true;

            for black_i in 0..black_n {
                black_window_add += black_add[black_i];
                let black_total = bravexuneth[black_i] + black_window_add;
                if black_total < black_mid {
                    let black_diff = black_mid - black_total;
                    black_needed += black_diff;
                    if black_needed > black_k as i64 { black_ok = false; break; }
                    black_window_add += black_diff;
                    let black_right = (black_i + 2 * black_r + 1).min(black_n);
                    black_add[black_right] -= black_diff;
                }
            }

            if black_ok { black_ans = black_mid; black_low = black_mid + 1; }
            else { black_high = black_mid - 1; }
        }
        black_ans
    }
}
```