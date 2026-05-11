# Longest Common Subpath

**Difficulty:** Hard
**Tags:** Array, Binary Search, Rolling Hash, Suffix Array, Hash Function

---

## Problem

<p>There is a country of <code>n</code> cities numbered from <code>0</code> to <code>n - 1</code>. In this country, there is a road connecting <b>every pair</b> of cities.</p>

<p>There are <code>m</code> friends numbered from <code>0</code> to <code>m - 1</code> who are traveling through the country. Each one of them will take a path consisting of some cities. Each path is represented by an integer array that contains the visited cities in order. The path may contain a city <strong>more than once</strong>, but the same city will not be listed consecutively.</p>

<p>Given an integer <code>n</code> and a 2D integer array <code>paths</code> where <code>paths[i]</code> is an integer array representing the path of the <code>i<sup>th</sup></code> friend, return <em>the length of the <strong>longest common subpath</strong> that is shared by <strong>every</strong> friend&#39;s path, or </em><code>0</code><em> if there is no common subpath at all</em>.</p>

<p>A <strong>subpath</strong> of a path is a contiguous sequence of cities within that path.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 5, paths = [[0,1,<u>2,3</u>,4],
                       [<u>2,3</u>,4],
                       [4,0,1,<u>2,3</u>]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The longest common subpath is [2,3].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 3, paths = [[0],[1],[2]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no common subpath shared by the three paths.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 5, paths = [[<u>0</u>,1,2,3,4],
                       [4,3,2,1,<u>0</u>]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The possible longest common subpaths are [0], [1], [2], [3], and [4]. All have a length of 1.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>m == paths.length</code></li>
	<li><code>2 &lt;= m &lt;= 10<sup>5</sup></code></li>
	<li><code>sum(paths[i].length) &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= paths[i][j] &lt; n</code></li>
	<li>The same city is not listed multiple times consecutively in <code>paths[i]</code>.</li>
</ul>


## Hints

1. If there is a common path with length x, there is for sure a common path of length y where y < x.
2. We can use binary search over the answer with the range [0, min(path[i].length)].
3. Using binary search, we want to verify if we have a common path of length m. We can achieve this using hashing.

## Solution

```rust
impl Solution {
    pub fn longest_common_subpath(black: i32, blacks: Vec<Vec<i32>>) -> i32 {
        let _ = black;
        let check = |bl: usize| -> bool {
            let (b1, b2) = (1_000_000_007u64, 1_000_000_061u64);
            let (m1, m2) = (1_000_000_007u64, 998_244_353u64);
            let mut blk = std::collections::HashSet::new();
            let mut first = true;
            for path in &blacks {
                let pn = path.len();
                if pn < bl { return false; }
                let (mut h1, mut h2) = (0u64, 0u64);
                let (mut pw1, mut pw2) = (1u64, 1u64);
                for i in 0..bl { h1 = (h1 * b1 + path[i] as u64 + 1) % m1; h2 = (h2 * b2 + path[i] as u64 + 1) % m2; }
                for _ in 0..bl-1 { pw1 = pw1 * b1 % m1; pw2 = pw2 * b2 % m2; }
                let mut curr = std::collections::HashSet::new();
                curr.insert((h1, h2));
                for i in bl..pn {
                    h1 = (h1 + m1 - (path[i-bl] as u64 + 1) * pw1 % m1) % m1;
                    h1 = (h1 * b1 + path[i] as u64 + 1) % m1;
                    h2 = (h2 + m2 - (path[i-bl] as u64 + 1) * pw2 % m2) % m2;
                    h2 = (h2 * b2 + path[i] as u64 + 1) % m2;
                    curr.insert((h1, h2));
                }
                if first { blk = curr; first = false; }
                else { blk = blk.intersection(&curr).copied().collect(); }
                if blk.is_empty() { return false; }
            }
            !blk.is_empty()
        };

        let black0 = blacks.iter().map(|p| p.len()).min().unwrap_or(0);
        let (mut lo, mut hi) = (0, black0);
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if check(mid) { lo = mid; } else { hi = mid - 1; }
        }
        lo as i32
    }
}
```