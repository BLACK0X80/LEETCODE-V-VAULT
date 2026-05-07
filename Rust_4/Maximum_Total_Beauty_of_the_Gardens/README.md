# Maximum Total Beauty of the Gardens

**Difficulty:** Hard
**Tags:** Array, Two Pointers, Binary Search, Greedy, Sorting, Enumeration, Prefix Sum

---

## Problem

<p>Alice is a caretaker of <code>n</code> gardens and she wants to plant flowers to maximize the total beauty of all her gardens.</p>

<p>You are given a <strong>0-indexed</strong> integer array <code>flowers</code> of size <code>n</code>, where <code>flowers[i]</code> is the number of flowers already planted in the <code>i<sup>th</sup></code> garden. Flowers that are already planted <strong>cannot</strong> be removed. You are then given another integer <code>newFlowers</code>, which is the <strong>maximum</strong> number of flowers that Alice can additionally plant. You are also given the integers <code>target</code>, <code>full</code>, and <code>partial</code>.</p>

<p>A garden is considered <strong>complete</strong> if it has <strong>at least</strong> <code>target</code> flowers. The <strong>total beauty</strong> of the gardens is then determined as the <strong>sum</strong> of the following:</p>

<ul>
	<li>The number of <strong>complete</strong> gardens multiplied by <code>full</code>.</li>
	<li>The <strong>minimum</strong> number of flowers in any of the <strong>incomplete</strong> gardens multiplied by <code>partial</code>. If there are no incomplete gardens, then this value will be <code>0</code>.</li>
</ul>

<p>Return <em>the <strong>maximum</strong> total beauty that Alice can obtain after planting at most </em><code>newFlowers</code><em> flowers.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> flowers = [1,3,1,1], newFlowers = 7, target = 6, full = 12, partial = 1
<strong>Output:</strong> 14
<strong>Explanation:</strong> Alice can plant
- 2 flowers in the 0<sup>th</sup> garden
- 3 flowers in the 1<sup>st</sup> garden
- 1 flower in the 2<sup>nd</sup> garden
- 1 flower in the 3<sup>rd</sup> garden
The gardens will then be [3,6,2,2]. She planted a total of 2 + 3 + 1 + 1 = 7 flowers.
There is 1 garden that is complete.
The minimum number of flowers in the incomplete gardens is 2.
Thus, the total beauty is 1 * 12 + 2 * 1 = 12 + 2 = 14.
No other way of planting flowers can obtain a total beauty higher than 14.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> flowers = [2,4,5,3], newFlowers = 10, target = 5, full = 2, partial = 6
<strong>Output:</strong> 30
<strong>Explanation:</strong> Alice can plant
- 3 flowers in the 0<sup>th</sup> garden
- 0 flowers in the 1<sup>st</sup> garden
- 0 flowers in the 2<sup>nd</sup> garden
- 2 flowers in the 3<sup>rd</sup> garden
The gardens will then be [5,4,5,5]. She planted a total of 3 + 0 + 0 + 2 = 5 flowers.
There are 3 gardens that are complete.
The minimum number of flowers in the incomplete gardens is 4.
Thus, the total beauty is 3 * 2 + 4 * 6 = 6 + 24 = 30.
No other way of planting flowers can obtain a total beauty higher than 30.
Note that Alice could make all the gardens complete but in this case, she would obtain a lower total beauty.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= flowers.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= flowers[i], target &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= newFlowers &lt;= 10<sup>10</sup></code></li>
	<li><code>1 &lt;= full, partial &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Say we choose k gardens to be complete, is there an optimal way of choosing which gardens to plant more flowers to achieve this?
2. For a given k, we should greedily fill-up the k gardens with the most flowers planted already. This gives us the most remaining flowers to fill up the other gardens.
3. After sorting flowers, we can thus try every possible k and what is left is to find the highest minimum flowers we can obtain by planting the remaining flowers in the other gardens.
4. To find the highest minimum in the other gardens, we can use binary search to find the most optimal way of planting.

## Solution

```rust
impl Solution {
    pub fn maximum_beauty(mut black: Vec<i32>, mut bl: i64, black0: i32, black1: i32, black2: i32) -> i64 {
        let (t, f, p) = (black0 as i64, black1 as i64, black2 as i64);
        black.sort_by(|a, b| b.cmp(a));
        let (mut b1, mut blk, mut res) = (0i64, 0i64, 0i64);
        let sz = black.len() as i64;

        while b1 < sz {
            if t - black[b1 as usize] as i64 > bl { break; }
            bl -= (t - black[b1 as usize] as i64).max(0);
            b1 += 1;
        }

        if b1 == sz {
            let last = (black.last().copied().unwrap_or(0)) as i64;
            return (sz * f).max((sz-1)*f + if last < t { (t-1)*p } else { f });
        }

        let mut b2 = sz - 1;
        let mut min_f = black[b2 as usize] as i64;

        while min_f < t {
            while b2 >= b1 && (black[b2 as usize] as i64) <= min_f {
                blk += black[b2 as usize] as i64;
                b2 -= 1;
            }
            let needed = (sz - b2 - 1) * min_f - blk;
            if needed > bl {
                b1 -= 1;
                if b1 < 0 { break; }
                bl += (t - black[b1 as usize] as i64).max(0);
            } else {
                res = res.max(b1 * f + min_f * p);
                min_f += 1;
            }
        }
        res
    }
}
```