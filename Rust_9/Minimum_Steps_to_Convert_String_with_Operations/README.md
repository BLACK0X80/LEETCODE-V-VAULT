# Minimum Steps to Convert String with Operations

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>You are given two strings, <code>word1</code> and <code>word2</code>, of equal length. You need to transform <code>word1</code> into <code>word2</code>.</p>

<p>For this, divide <code>word1</code> into one or more <strong>contiguous <span data-keyword="substring-nonempty">substrings</span></strong>. For each substring <code>substr</code> you can perform the following operations:</p>

<ol>
	<li>
	<p><strong>Replace:</strong> Replace the character at any one index of <code>substr</code> with another lowercase English letter.</p>
	</li>
	<li>
	<p><strong>Swap:</strong> Swap any two characters in <code>substr</code>.</p>
	</li>
	<li>
	<p><strong>Reverse Substring:</strong> Reverse <code>substr</code>.</p>
	</li>
</ol>

<p>Each of these counts as <strong>one</strong> operation and each character of each substring can be used in each type of operation at most once (i.e. no single index may be involved in more than one replace, one swap, or one reverse).</p>

<p>Return the <strong>minimum number of operations</strong> required to transform <code>word1</code> into <code>word2</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word1 = &quot;abcdf&quot;, word2 = &quot;dacbe&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>Divide <code>word1</code> into <code>&quot;ab&quot;</code>, <code>&quot;c&quot;</code>, and <code>&quot;df&quot;</code>. The operations are:</p>

<ul>
	<li>For the substring <code>&quot;ab&quot;</code>,

	<ul>
		<li>Perform operation of type 3 on <code>&quot;ab&quot; -&gt; &quot;ba&quot;</code>.</li>
		<li>Perform operation of type 1 on <code>&quot;ba&quot; -&gt; &quot;da&quot;</code>.</li>
	</ul>
	</li>
	<li>For the substring <code>&quot;c&quot;</code> do no operations.</li>
	<li>For the substring <code>&quot;df&quot;</code>,
	<ul>
		<li>Perform operation of type 1 on <code>&quot;df&quot; -&gt; &quot;bf&quot;</code>.</li>
		<li>Perform operation of type 1 on <code>&quot;bf&quot; -&gt; &quot;be&quot;</code>.</li>
	</ul>
	</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word1 = &quot;abceded&quot;, word2 = &quot;baecfef&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>Divide <code>word1</code> into <code>&quot;ab&quot;</code>, <code>&quot;ce&quot;</code>, and <code>&quot;ded&quot;</code>. The operations are:</p>

<ul>
	<li>For the substring <code>&quot;ab&quot;</code>,

	<ul>
		<li>Perform operation of type 2 on <code>&quot;ab&quot; -&gt; &quot;ba&quot;</code>.</li>
	</ul>
	</li>
	<li>For the substring <code>&quot;ce&quot;</code>,
	<ul>
		<li>Perform operation of type 2 on <code>&quot;ce&quot; -&gt; &quot;ec&quot;</code>.</li>
	</ul>
	</li>
	<li>For the substring <code>&quot;ded&quot;</code>,
	<ul>
		<li>Perform operation of type 1 on <code>&quot;ded&quot; -&gt; &quot;fed&quot;</code>.</li>
		<li>Perform operation of type 1 on <code>&quot;fed&quot; -&gt; &quot;fef&quot;</code>.</li>
	</ul>
	</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word1 = &quot;abcdef&quot;, word2 = &quot;fedabc&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>Divide <code>word1</code> into <code>&quot;abcdef&quot;</code>. The operations are:</p>

<ul>
	<li>For the substring <code>&quot;abcdef&quot;</code>,

	<ul>
		<li>Perform operation of type 3 on <code>&quot;abcdef&quot; -&gt; &quot;fedcba&quot;</code>.</li>
		<li>Perform operation of type 2 on <code>&quot;fedcba&quot; -&gt; &quot;fedabc&quot;</code>.</li>
	</ul>
	</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= word1.length == word2.length &lt;= 100</code></li>
	<li><code>word1</code> and <code>word2</code> consist only of lowercase English letters.</li>
</ul>


## Hints

1. Use dynamic programming
2. Do DP on disjoint substrings of <code>word1</code>. For the DP, we try both the substring and its reversed version (just add one extra operation)
3. First we swap pairs like (<code>word1[i]</code>, <code>word2[i]</code>) and (<code>word1[j]</code>, <code>word2[j]</code>) where <code>word1[i] == word2[j]</code> and <code>word2[i] == word1[j]</code>
4. For the remaining characters, we use replace operations

## Solution

```rust
impl Solution {
    pub fn min_operations(word1: String, word2: String) -> i32 {
        let black_n = word1.len();
        let black_arr1 = word1.as_bytes();
        let black_arr2 = word2.as_bytes();
        let mut black_dp = vec![vec![None; black_n]; black_n];

        Self::solve(0, 0, black_arr1, black_arr2, black_n, &mut black_dp)
    }

    fn solve(black_i: usize, black_j: usize, black_arr1: &[u8], black_arr2: &[u8], black_n: usize, black_dp: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if black_i >= black_n { return 0; }
        if black_j >= black_n { return 100000; }

        if let Some(black_val) = black_dp[black_i][black_j] {
            return black_val;
        }

        let black_dont_start = Self::solve(black_i, black_j + 1, black_arr1, black_arr2, black_n, black_dp);
        
        let black_cost = Self::min_opr(black_arr1, black_arr2, black_i, black_j, false)
            .min(Self::min_opr(black_arr1, black_arr2, black_i, black_j, true));
            
        let black_start = black_cost + Self::solve(black_j + 1, black_j + 1, black_arr1, black_arr2, black_n, black_dp);
        
        let black_res = black_dont_start.min(black_start);
        black_dp[black_i][black_j] = Some(black_res);
        black_res
    }

    fn min_opr(black_arr1: &[u8], black_arr2: &[u8], black_i: usize, black_j: usize, black_reversed: bool) -> i32 {
        let mut black_operations = if black_reversed { 1 } else { 0 };
        let mut black_freq = [[0i32; 26]; 26];
        let black_len = black_j - black_i + 1;

        for black_k in 0..black_len {
            let black_idx1 = black_i + black_k;
            let black_idx2 = if black_reversed { black_j - black_k } else { black_i + black_k };

            if black_arr1[black_idx1] != black_arr2[black_idx2] {
                let black_wanted = (black_arr1[black_idx1] - b'a') as usize;
                let black_got = (black_arr2[black_idx2] - b'a') as usize;

                if black_freq[black_got][black_wanted] > 0 {
                    black_freq[black_got][black_wanted] -= 1;
                } else {
                    black_freq[black_wanted][black_got] += 1;
                    black_operations += 1;
                }
            }
        }
        black_operations
    }
}
```