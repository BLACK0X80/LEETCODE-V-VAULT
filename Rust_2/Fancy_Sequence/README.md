# Fancy Sequence

**Difficulty:** Hard
**Tags:** Math, Design, Segment Tree, Number Theory

---

## Problem

<p>Write an API that generates fancy sequences using the <code>append</code>, <code>addAll</code>, and <code>multAll</code> operations.</p>

<p>Implement the <code>Fancy</code> class:</p>

<ul>
	<li><code>Fancy()</code> Initializes the object with an empty sequence.</li>
	<li><code>void append(val)</code> Appends an integer <code>val</code> to the end of the sequence.</li>
	<li><code>void addAll(inc)</code> Increments all existing values in the sequence by an integer <code>inc</code>.</li>
	<li><code>void multAll(m)</code> Multiplies all existing values in the sequence by an integer <code>m</code>.</li>
	<li><code>int getIndex(idx)</code> Gets the current value at index <code>idx</code> (0-indexed) of the sequence <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>. If the index is greater or equal than the length of the sequence, return <code>-1</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;Fancy&quot;, &quot;append&quot;, &quot;addAll&quot;, &quot;append&quot;, &quot;multAll&quot;, &quot;getIndex&quot;, &quot;addAll&quot;, &quot;append&quot;, &quot;multAll&quot;, &quot;getIndex&quot;, &quot;getIndex&quot;, &quot;getIndex&quot;]
[[], [2], [3], [7], [2], [0], [3], [10], [2], [0], [1], [2]]
<strong>Output</strong>
[null, null, null, null, null, 10, null, null, null, 26, 34, 20]

<strong>Explanation</strong>
Fancy fancy = new Fancy();
fancy.append(2);   // fancy sequence: [2]
fancy.addAll(3);   // fancy sequence: [2+3] -&gt; [5]
fancy.append(7);   // fancy sequence: [5, 7]
fancy.multAll(2);  // fancy sequence: [5*2, 7*2] -&gt; [10, 14]
fancy.getIndex(0); // return 10
fancy.addAll(3);   // fancy sequence: [10+3, 14+3] -&gt; [13, 17]
fancy.append(10);  // fancy sequence: [13, 17, 10]
fancy.multAll(2);  // fancy sequence: [13*2, 17*2, 10*2] -&gt; [26, 34, 20]
fancy.getIndex(0); // return 26
fancy.getIndex(1); // return 34
fancy.getIndex(2); // return 20
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= val, inc, m &lt;= 100</code></li>
	<li><code>0 &lt;= idx &lt;= 10<sup>5</sup></code></li>
	<li>At most <code>10<sup>5</sup></code> calls total will be made to <code>append</code>, <code>addAll</code>, <code>multAll</code>, and <code>getIndex</code>.</li>
</ul>


## Hints

1. Use two arrays to save the cumulative multipliers at each time point and cumulative sums adjusted by the current multiplier.
2. The function getIndex(idx) ask to the current value modulo 10^9+7. Use modular inverse and both arrays to calculate this value.

## Solution

```rust
const MOD: i64 = 1_000_000_007;

struct Fancy {
    black: Vec<i64>,
    mul: i64,
    add: i64,
}

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    let mut res = 1;
    while e > 0 {
        if e & 1 == 1 { res = res * a % MOD; }
        a = a * a % MOD;
        e >>= 1;
    }
    res
}

impl Fancy {
    fn new() -> Self {
        Self { black: Vec::new(), mul: 1, add: 0 }
    }
    
    fn append(&mut self, val: i32) {
        let val = val as i64;
        let inv = mod_pow(self.mul, MOD - 2);
        let stored = (val - self.add + MOD) % MOD * inv % MOD;
        self.black.push(stored);
    }
    
    fn add_all(&mut self, inc: i32) {
        self.add = (self.add + inc as i64) % MOD;
    }
    
    fn mult_all(&mut self, m: i32) {
        self.mul = self.mul * m as i64 % MOD;
        self.add = self.add * m as i64 % MOD;
    }
    
    fn get_index(&self, idx: i32) -> i32 {
        let idx = idx as usize;
        if idx >= self.black.len() { return -1; }
        ((self.black[idx] * self.mul % MOD + self.add) % MOD) as i32
    }
}
```