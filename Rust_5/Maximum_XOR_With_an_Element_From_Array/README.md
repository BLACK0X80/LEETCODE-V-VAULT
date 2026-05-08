# Maximum XOR With an Element From Array

**Difficulty:** Hard
**Tags:** Array, Bit Manipulation, Trie

---

## Problem

<p>You are given an array <code>nums</code> consisting of non-negative integers. You are also given a <code>queries</code> array, where <code>queries[i] = [x<sub>i</sub>, m<sub>i</sub>]</code>.</p>

<p>The answer to the <code>i<sup>th</sup></code> query is the maximum bitwise <code>XOR</code> value of <code>x<sub>i</sub></code> and any element of <code>nums</code> that does not exceed <code>m<sub>i</sub></code>. In other words, the answer is <code>max(nums[j] XOR x<sub>i</sub>)</code> for all <code>j</code> such that <code>nums[j] &lt;= m<sub>i</sub></code>. If all elements in <code>nums</code> are larger than <code>m<sub>i</sub></code>, then the answer is <code>-1</code>.</p>

<p>Return <em>an integer array </em><code>answer</code><em> where </em><code>answer.length == queries.length</code><em> and </em><code>answer[i]</code><em> is the answer to the </em><code>i<sup>th</sup></code><em> query.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [0,1,2,3,4], queries = [[3,1],[1,3],[5,6]]
<strong>Output:</strong> [3,3,7]
<strong>Explanation:</strong>
1) 0 and 1 are the only two integers not greater than 1. 0 XOR 3 = 3 and 1 XOR 3 = 2. The larger of the two is 3.
2) 1 XOR 2 = 3.
3) 5 XOR 2 = 7.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [5,2,4,6,6,3], queries = [[12,4],[8,1],[6,3]]
<strong>Output:</strong> [15,-1,5]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length, queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>queries[i].length == 2</code></li>
	<li><code>0 &lt;= nums[j], x<sub>i</sub>, m<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. In problems involving bitwise operations, we often think on the bits level. In this problem, we can think that to maximize the result of an xor operation, we need to maximize the most significant bit, then the next one, and so on.
2. If there's some number in the array that is less than m and whose the most significant bit is different than that of x, then xoring with this number maximizes the most significant bit, so I know this bit in the answer is 1.
3. To check the existence of such numbers and narrow your scope for further bits based on your choice, you can use trie.
4. You can sort the array and the queries, and maintain the trie such that in each query the trie consists exactly of the valid elements.

## Solution

```rust
impl Solution {
    pub fn maximize_xor(mut black_nums: Vec<i32>, black_queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_nums.len();
        let black_q_len = black_queries.len();
        let mut black_ans = vec![-1; black_q_len];
        
        black_nums.sort_unstable();
        
        let mut black_q_idx: Vec<usize> = (0..black_q_len).collect();
        black_q_idx.sort_by_key(|&i| black_queries[i][1]);
        
        let mut black_trie = BlackTrie::new();
        let mut black_ptr = 0;
        
        for &black_idx in &black_q_idx {
            let black_x = black_queries[black_idx][0];
            let black_m = black_queries[black_idx][1];
            
            while black_ptr < black_n && black_nums[black_ptr] <= black_m {
                black_trie.black_insert(black_nums[black_ptr]);
                black_ptr += 1;
            }
            
            if black_ptr > 0 {
                black_ans[black_idx] = black_trie.black_get_max(black_x);
            }
        }
        
        black_ans
    }
}

struct BlackNode {
    black_children: [Option<Box<BlackNode>>; 2],
}

struct BlackTrie {
    black_root: BlackNode,
}

impl BlackTrie {
    fn new() -> Self {
        Self {
            black_root: BlackNode {
                black_children: [None, None],
            },
        }
    }
    
    fn black_insert(&mut self, black_val: i32) {
        let mut black_curr = &mut self.black_root;
        for black_i in (0..31).rev() {
            let black_bit = ((black_val >> black_i) & 1) as usize;
            if black_curr.black_children[black_bit].is_none() {
                black_curr.black_children[black_bit] = Some(Box::new(BlackNode {
                    black_children: [None, None],
                }));
            }
            black_curr = black_curr.black_children[black_bit].as_mut().unwrap();
        }
    }
    
    fn black_get_max(&self, black_val: i32) -> i32 {
        let mut black_curr = &self.black_root;
        let mut black_res = 0;
        for black_i in (0..31).rev() {
            let black_bit = ((black_val >> black_i) & 1) as usize;
            let black_desired = 1 - black_bit;
            if black_curr.black_children[black_desired].is_some() {
                black_res |= 1 << black_i;
                black_curr = black_curr.black_children[black_desired].as_ref().unwrap();
            } else {
                black_curr = black_curr.black_children[black_bit].as_ref().unwrap();
            }
        }
        black_res
    }
}


```