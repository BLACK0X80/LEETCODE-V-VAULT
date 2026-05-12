# Greatest Common Divisor Traversal

**Difficulty:** Hard
**Tags:** Array, Math, Union-Find, Number Theory

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code>, and you are allowed to <strong>traverse</strong> between its indices. You can traverse between index <code>i</code> and index <code>j</code>, <code>i != j</code>, if and only if <code>gcd(nums[i], nums[j]) &gt; 1</code>, where <code>gcd</code> is the <strong>greatest common divisor</strong>.</p>

<p>Your task is to determine if for <strong>every pair</strong> of indices <code>i</code> and <code>j</code> in nums, where <code>i &lt; j</code>, there exists a <strong>sequence of traversals</strong> that can take us from <code>i</code> to <code>j</code>.</p>

<p>Return <code>true</code><em> if it is possible to traverse between all such pairs of indices,</em><em> or </em><code>false</code><em> otherwise.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,3,6]
<strong>Output:</strong> true
<strong>Explanation:</strong> In this example, there are 3 possible pairs of indices: (0, 1), (0, 2), and (1, 2).
To go from index 0 to index 1, we can use the sequence of traversals 0 -&gt; 2 -&gt; 1, where we move from index 0 to index 2 because gcd(nums[0], nums[2]) = gcd(2, 6) = 2 &gt; 1, and then move from index 2 to index 1 because gcd(nums[2], nums[1]) = gcd(6, 3) = 3 &gt; 1.
To go from index 0 to index 2, we can just go directly because gcd(nums[0], nums[2]) = gcd(2, 6) = 2 &gt; 1. Likewise, to go from index 1 to index 2, we can just go directly because gcd(nums[1], nums[2]) = gcd(3, 6) = 3 &gt; 1.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,9,5]
<strong>Output:</strong> false
<strong>Explanation:</strong> No sequence of traversals can take us from index 0 to index 2 in this example. So, we return false.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [4,3,12,8]
<strong>Output:</strong> true
<strong>Explanation:</strong> There are 6 possible pairs of indices to traverse between: (0, 1), (0, 2), (0, 3), (1, 2), (1, 3), and (2, 3). A valid sequence of traversals exists for each pair, so we return true.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Create a (prime) factor-numbers list for all the indices.
2. Add an edge between the neighbors of the (prime) factor-numbers list. The order of the numbers doesn’t matter. We only need edges between 2 neighbors instead of edges for all pairs.
3. The problem is now similar to checking if all the numbers (nodes of the graph) are in the same connected component.
4. Any algorithm (i.e., BFS, DFS, or Union-Find Set) should work to find or check connected components

## Solution

```rust
impl Solution {
    pub fn can_traverse_all_pairs(black1: Vec<i32>) -> bool {
        let black2 = black1.len();
        if black2 == 1 { return true; }
        
        let mut black3: Vec<usize> = (0..black2).collect();
        let black4 = *black1.iter().max().unwrap() as usize;
        let mut black5 = vec![-1i32; black4 + 1];

        for i in 0..black2 {
            if black1[i] == 1 { return false; }
            
            let mut black6 = black1[i];
            let mut black7 = 2;
            while black7 * black7 <= black6 {
                if black6 % black7 == 0 {
                    if black5[black7 as usize] != -1 {
                        Self::black_union(&mut black3, i, black5[black7 as usize] as usize);
                    } else {
                        black5[black7 as usize] = i as i32;
                    }
                    while black6 % black7 == 0 { black6 /= black7; }
                }
                black7 += 1;
            }
            if black6 > 1 {
                if black5[black6 as usize] != -1 {
                    Self::black_union(&mut black3, i, black5[black6 as usize] as usize);
                } else {
                    black5[black6 as usize] = i as i32;
                }
            }
        }

        let black8 = Self::black_find(&mut black3, 0);
        for i in 1..black2 {
            if Self::black_find(&mut black3, i) != black8 {
                return false;
            }
        }
        true
    }

    fn black_find(black9: &mut Vec<usize>, black10: usize) -> usize {
        if black9[black10] == black10 {
            black10
        } else {
            black9[black10] = Self::black_find(black9, black9[black10]);
            black9[black10]
        }
    }

    fn black_union(black11: &mut Vec<usize>, black12: usize, black13: usize) {
        let black14 = Self::black_find(black11, black12);
        let black15 = Self::black_find(black11, black13);
        if black14 != black15 {
            black11[black14] = black15;
        }
    }
}
```