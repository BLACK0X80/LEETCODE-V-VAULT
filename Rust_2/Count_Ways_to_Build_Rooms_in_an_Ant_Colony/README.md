# Count Ways to Build Rooms in an Ant Colony

**Difficulty:** Hard
**Tags:** Array, Math, Dynamic Programming, Tree, Depth-First Search, Graph Theory, Topological Sort, Combinatorics

---

## Problem

<p>You are an ant tasked with adding <code>n</code> new rooms numbered <code>0</code> to <code>n-1</code> to your colony. You are given the expansion plan as a <strong>0-indexed</strong> integer array of length <code>n</code>, <code>prevRoom</code>, where <code>prevRoom[i]</code> indicates that you must build room <code>prevRoom[i]</code> before building room <code>i</code>, and these two rooms must be connected <strong>directly</strong>. Room <code>0</code> is already built, so <code>prevRoom[0] = -1</code>. The expansion&nbsp;plan is given such that once all the rooms are built, every room will be reachable from room <code>0</code>.</p>

<p>You can only build <strong>one room</strong> at a time, and you can travel freely between rooms you have <strong>already built</strong> only if they are <strong>connected</strong>.&nbsp;You can choose to build <strong>any room</strong> as long as its <strong>previous room</strong>&nbsp;is already built.</p>

<p>Return <em>the <strong>number of different orders</strong> you can build all the rooms in</em>. Since the answer may be large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/19/d1.JPG" style="width: 200px; height: 212px;" />
<pre>
<strong>Input:</strong> prevRoom = [-1,0,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong>&nbsp;There is only one way to build the additional rooms: 0 &rarr; 1 &rarr; 2
</pre>

<p><strong class="example">Example 2:</strong></p>
<strong><img alt="" src="https://assets.leetcode.com/uploads/2021/06/19/d2.JPG" style="width: 200px; height: 239px;" /></strong>

<pre>
<strong>Input:</strong> prevRoom = [-1,0,0,1,2]
<strong>Output:</strong> 6
<strong>Explanation:
</strong>The 6 ways are:
0 &rarr; 1 &rarr; 3 &rarr; 2 &rarr; 4
0 &rarr; 2 &rarr; 4 &rarr; 1 &rarr; 3
0 &rarr; 1 &rarr; 2 &rarr; 3 &rarr; 4
0 &rarr; 1 &rarr; 2 &rarr; 4 &rarr; 3
0 &rarr; 2 &rarr; 1 &rarr; 3 &rarr; 4
0 &rarr; 2 &rarr; 1 &rarr; 4 &rarr; 3
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == prevRoom.length</code></li>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>prevRoom[0] == -1</code></li>
	<li><code>0 &lt;= prevRoom[i] &lt; n</code> for all <code>1 &lt;= i &lt; n</code></li>
	<li>Every room is reachable from room <code>0</code> once all the rooms are built.</li>
</ul>

## Hints

1. Use dynamic programming.
2. Let dp[i] be the number of ways to solve the problem for the subtree of node i.
3. Imagine you are trying to fill an array with the order of traversal, dp[i] equals the multiplications of the number of ways to distribute the subtrees of the children of i on the array using combinatorics, multiplied bu their dp values.

## Solution

```rust
impl Solution {
    pub fn ways_to_build_rooms(black1: Vec<i32>) -> i32 {
        let black2 = black1.len();
        let black3 = 1_000_000_007u64;
        let mut black4 = vec![vec![]; black2];
        let mut black5 = vec![0; black2];
        
        for (black6, &black7) in black1.iter().enumerate().skip(1) {
            black4[black7 as usize].push(black6);
            black5[black7 as usize] += 1;
        }

        let mut black8 = vec![1u64; black2 + 1];
        let mut black9 = vec![1u64; black2 + 1];
        for black10 in 2..=black2 {
            black8[black10] = (black8[black10 - 1] * black10 as u64) % black3;
        }

        fn black_pow(mut black11: u64, mut black12: u64, black13: u64) -> u64 {
            let mut black14 = 1;
            black11 %= black13;
            while black12 > 0 {
                if black12 % 2 == 1 { black14 = (black14 * black11) % black13; }
                black11 = (black11 * black11) % black13;
                black12 /= 2;
            }
            black14
        }

        black9[black2] = black_pow(black8[black2], black3 - 2, black3);
        for black15 in (1..black2).rev() {
            black9[black15] = (black9[black15 + 1] * (black15 + 1) as u64) % black3;
        }

        let mut black16 = vec![0; black2];
        let mut black17 = vec![1u64; black2];
        let mut black18 = Vec::new();
        let mut black19 = vec![0; black2];
        for black20 in 0..black2 {
            black19[black20] = black4[black20].len();
            if black19[black20] == 0 { black18.push(black20); }
        }

        let mut black21 = 0;
        while black21 < black18.len() {
            let black22 = black18[black21];
            black21 += 1;
            black16[black22] += 1;
            
            let black23 = black8[black16[black22] - 1];
            black17[black22] = (black17[black22] * black23) % black3;

            if black22 != 0 {
                let black24 = black1[black22] as usize;
                black16[black24] += black16[black22];
                black17[black24] = (black17[black24] * black17[black22]) % black3;
                black17[black24] = (black17[black24] * black9[black16[black22]]) % black3;
                black19[black24] -= 1;
                if black19[black24] == 0 { black18.push(black24); }
            }
        }

        black17[0] as i32
    }
}
```