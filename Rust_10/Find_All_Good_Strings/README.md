# Find All Good Strings

**Difficulty:** Hard
**Tags:** String, Dynamic Programming, String Matching

---

## Problem

<p>Given the strings <code>s1</code> and <code>s2</code> of size <code>n</code> and the string <code>evil</code>, return <em>the number of <strong>good</strong> strings</em>.</p>

<p>A <strong>good</strong> string has size <code>n</code>, it is alphabetically greater than or equal to <code>s1</code>, it is alphabetically smaller than or equal to <code>s2</code>, and it does not contain the string <code>evil</code> as a substring. Since the answer can be a huge number, return this <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 2, s1 = &quot;aa&quot;, s2 = &quot;da&quot;, evil = &quot;b&quot;
<strong>Output:</strong> 51 
<strong>Explanation:</strong> There are 25 good strings starting with &#39;a&#39;: &quot;aa&quot;,&quot;ac&quot;,&quot;ad&quot;,...,&quot;az&quot;. Then there are 25 good strings starting with &#39;c&#39;: &quot;ca&quot;,&quot;cc&quot;,&quot;cd&quot;,...,&quot;cz&quot; and finally there is one good string starting with &#39;d&#39;: &quot;da&quot;.&nbsp;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 8, s1 = &quot;leetcode&quot;, s2 = &quot;leetgoes&quot;, evil = &quot;leet&quot;
<strong>Output:</strong> 0 
<strong>Explanation:</strong> All strings greater than or equal to s1 and smaller than or equal to s2 start with the prefix &quot;leet&quot;, therefore, there is not any good string.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 2, s1 = &quot;gx&quot;, s2 = &quot;gz&quot;, evil = &quot;x&quot;
<strong>Output:</strong> 2
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>s1.length == n</code></li>
	<li><code>s2.length == n</code></li>
	<li><code>s1 &lt;= s2</code></li>
	<li><code>1 &lt;= n &lt;= 500</code></li>
	<li><code>1 &lt;= evil.length &lt;= 50</code></li>
	<li>All strings consist of lowercase English letters.</li>
</ul>


## Hints

1. Use DP with 4 states (pos: Int, posEvil: Int, equalToS1: Bool, equalToS2: Bool) which compute the number of valid strings of size "pos" where the maximum common suffix with string "evil" has size "posEvil". When "equalToS1" is "true", the current valid string is equal to "S1" otherwise it is greater. In a similar way when equalToS2 is "true" the current valid string is equal to "S2" otherwise it is smaller.
2. To update the maximum common suffix with string "evil" use KMP preprocessing.

## Solution

```rust
impl Solution {
    pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let evil = evil.as_bytes();
        let m = evil.len();

        let mut fail = vec![0usize; m];
        let mut j = 0usize;
        for i in 1..m {
            while j > 0 && evil[i] != evil[j] { j = fail[j-1]; }
            if evil[i] == evil[j] { j += 1; }
            fail[i] = j;
        }

        fn kmp_next(fail: &[usize], state: usize, c: u8, evil: &[u8]) -> usize {
            let mut j = state;
            while j > 0 && c != evil[j] { j = fail[j-1]; }
            if c == evil[j] { j += 1; }
            j
        }

        fn count(
            s: &[u8], evil: &[u8], fail: &[usize],
            tight: bool, started: bool, pos: usize, evil_state: usize,
            memo: &mut Vec<Vec<Vec<Vec<u64>>>>,
            n: usize,
        ) -> u64 {
            const MOD: u64 = 1_000_000_007;
            if evil_state == evil.len() { return 0; }
            if pos == n { return 1; }
            let ti = tight as usize;
            let si = started as usize;
            if memo[pos][evil_state][ti][si] != u64::MAX {
                return memo[pos][evil_state][ti][si];
            }
            let limit = if tight { s[pos] } else { b'z' };
            let mut res = 0u64;
            for c in b'a'..=limit {
                let ns = kmp_next(fail, evil_state, c, evil);
                if ns == evil.len() { continue; }
                res = (res + count(s, evil, fail, tight && c == limit, true, pos+1, ns, memo, n)) % MOD;
            }
            memo[pos][evil_state][ti][si] = res;
            res
        }

        let n = n as usize;
        let m = evil.len();
        let mut memo1 = vec![vec![vec![vec![u64::MAX; 2]; 2]; m+1]; n];
        let mut memo2 = vec![vec![vec![vec![u64::MAX; 2]; 2]; m+1]; n];
        let a = count(s2, evil, &fail, true, false, 0, 0, &mut memo2, n);
        let b = count(s1, evil, &fail, true, false, 0, 0, &mut memo1, n);
        let _ = memo1;
        let evil_in_s1 = {
            let mut j = 0usize;
            let mut found = false;
            for &c in s1 {
                while j > 0 && c != evil[j] { j = fail[j-1]; }
                if c == evil[j] { j += 1; }
                if j == m { found = true; break; }
            }
            found
        };
        let res = (a + MOD - b + if evil_in_s1 { 0 } else { 1 }) % MOD;
        res as i32
    }
}
```