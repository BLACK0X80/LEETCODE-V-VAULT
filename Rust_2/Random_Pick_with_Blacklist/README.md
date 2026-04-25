# Random Pick with Blacklist

**Difficulty:** Hard
**Tags:** Array, Hash Table, Math, Binary Search, Sorting, Randomized

---

## Problem

<p>You are given an integer <code>n</code> and an array of <strong>unique</strong> integers <code>blacklist</code>. Design an algorithm to pick a random integer in the range <code>[0, n - 1]</code> that is <strong>not</strong> in <code>blacklist</code>. Any integer that is in the mentioned range and not in <code>blacklist</code> should be <strong>equally likely</strong> to be returned.</p>

<p>Optimize your algorithm such that it minimizes the number of calls to the <strong>built-in</strong> random function of your language.</p>

<p>Implement the <code>Solution</code> class:</p>

<ul>
	<li><code>Solution(int n, int[] blacklist)</code> Initializes the object with the integer <code>n</code> and the blacklisted integers <code>blacklist</code>.</li>
	<li><code>int pick()</code> Returns a random integer in the range <code>[0, n - 1]</code> and not in <code>blacklist</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;Solution&quot;, &quot;pick&quot;, &quot;pick&quot;, &quot;pick&quot;, &quot;pick&quot;, &quot;pick&quot;, &quot;pick&quot;, &quot;pick&quot;]
[[7, [2, 3, 5]], [], [], [], [], [], [], []]
<strong>Output</strong>
[null, 0, 4, 1, 6, 1, 0, 4]

<strong>Explanation</strong>
Solution solution = new Solution(7, [2, 3, 5]);
solution.pick(); // return 0, any integer from [0,1,4,6] should be ok. Note that for every call of pick,
                 // 0, 1, 4, and 6 must be equally likely to be returned (i.e., with probability 1/4).
solution.pick(); // return 4
solution.pick(); // return 1
solution.pick(); // return 6
solution.pick(); // return 1
solution.pick(); // return 0
solution.pick(); // return 4
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= blacklist.length &lt;= min(10<sup>5</sup>, n - 1)</code></li>
	<li><code>0 &lt;= blacklist[i] &lt; n</code></li>
	<li>All the values of <code>blacklist</code> are <strong>unique</strong>.</li>
	<li>At most <code>2 * 10<sup>4</sup></code> calls will be made to <code>pick</code>.</li>
</ul>



## Solution

```rust
use std::collections::HashMap;

struct Solution {
    black_map: HashMap<i32, i32>,
    black_sz: i32,
}

impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let black_sz = n - blacklist.len() as i32;
        let black_set: std::collections::HashSet<i32> = blacklist.iter().cloned().collect();
        let mut black_map = HashMap::new();
        let mut black_ptr = black_sz;
        for &b in &blacklist {
            if b < black_sz {
                while black_set.contains(&black_ptr) { black_ptr += 1; }
                black_map.insert(b, black_ptr);
                black_ptr += 1;
            }
        }
        Solution { black_map, black_sz }
    }

    fn pick(&self) -> i32 {
        let black_r = (rand::random::<u32>() % self.black_sz as u32) as i32;
        *self.black_map.get(&black_r).unwrap_or(&black_r)
    }
}
```