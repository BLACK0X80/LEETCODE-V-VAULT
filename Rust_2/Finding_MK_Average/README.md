# Finding MK Average

**Difficulty:** Hard
**Tags:** Design, Queue, Heap (Priority Queue), Data Stream, Ordered Set

---

## Problem

<p>You are given two integers, <code>m</code> and <code>k</code>, and a stream of integers. You are tasked to implement a data structure that calculates the <strong>MKAverage</strong> for the stream.</p>

<p>The <strong>MKAverage</strong> can be calculated using these steps:</p>

<ol>
	<li>If the number of the elements in the stream is less than <code>m</code> you should consider the <strong>MKAverage</strong> to be <code>-1</code>. Otherwise, copy the last <code>m</code> elements of the stream to a separate container.</li>
	<li>Remove the smallest <code>k</code> elements and the largest <code>k</code> elements from the container.</li>
	<li>Calculate the average value for the rest of the elements <strong>rounded down to the nearest integer</strong>.</li>
</ol>

<p>Implement the <code>MKAverage</code> class:</p>

<ul>
	<li><code>MKAverage(int m, int k)</code> Initializes the <strong>MKAverage</strong> object with an empty stream and the two integers <code>m</code> and <code>k</code>.</li>
	<li><code>void addElement(int num)</code> Inserts a new element <code>num</code> into the stream.</li>
	<li><code>int calculateMKAverage()</code> Calculates and returns the <strong>MKAverage</strong> for the current stream <strong>rounded down to the nearest integer</strong>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;MKAverage&quot;, &quot;addElement&quot;, &quot;addElement&quot;, &quot;calculateMKAverage&quot;, &quot;addElement&quot;, &quot;calculateMKAverage&quot;, &quot;addElement&quot;, &quot;addElement&quot;, &quot;addElement&quot;, &quot;calculateMKAverage&quot;]
[[3, 1], [3], [1], [], [10], [], [5], [5], [5], []]
<strong>Output</strong>
[null, null, null, -1, null, 3, null, null, null, 5]

<strong>Explanation</strong>
<code>MKAverage obj = new MKAverage(3, 1); 
obj.addElement(3);        // current elements are [3]
obj.addElement(1);        // current elements are [3,1]
obj.calculateMKAverage(); // return -1, because m = 3 and only 2 elements exist.
obj.addElement(10);       // current elements are [3,1,10]
obj.calculateMKAverage(); // The last 3 elements are [3,1,10].
                          // After removing smallest and largest 1 element the container will be [3].
                          // The average of [3] equals 3/1 = 3, return 3
obj.addElement(5);        // current elements are [3,1,10,5]
obj.addElement(5);        // current elements are [3,1,10,5,5]
obj.addElement(5);        // current elements are [3,1,10,5,5,5]
obj.calculateMKAverage(); // The last 3 elements are [5,5,5].
                          // After removing smallest and largest 1 element the container will be [5].
                          // The average of [5] equals 5/1 = 5, return 5
</code></pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= m &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt; k*2 &lt; m</code></li>
	<li><code>1 &lt;= num &lt;= 10<sup>5</sup></code></li>
	<li>At most <code>10<sup>5</sup></code> calls will be made to <code>addElement</code> and <code>calculateMKAverage</code>.</li>
</ul>


## Hints

1. At each query, try to save and update the sum of the elements needed to calculate MKAverage.
2. You can use BSTs for fast insertion and deletion of the elements.

## Solution

```rust
use std::collections::BTreeMap;

struct MKAverage {
    black_m: usize,
    black_k: usize,
    black_sz: usize,
    black_pos: usize,
    black_sum: i64,
    black_buf: Vec<i32>,
    black_lo: BTreeMap<i32, usize>,
    black_mid: BTreeMap<i32, usize>,
    black_hi: BTreeMap<i32, usize>,
}

impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        let (m, k) = (m as usize, k as usize);
        MKAverage {
            black_m: m, black_k: k, black_sz: m - 2 * k,
            black_pos: 0, black_sum: 0,
            black_buf: vec![0; m],
            black_lo: BTreeMap::new(),
            black_mid: BTreeMap::new(),
            black_hi: BTreeMap::new(),
        }
    }

    fn black_erase(map: &mut BTreeMap<i32, usize>, v: i32) {
        let c = map.get_mut(&v).unwrap();
        if *c == 1 { map.remove(&v); } else { *c -= 1; }
    }

    fn black_insert(map: &mut BTreeMap<i32, usize>, v: i32) {
        *map.entry(v).or_insert(0) += 1;
    }

    fn black_max(map: &BTreeMap<i32, usize>) -> i32 { *map.keys().next_back().unwrap() }
    fn black_min(map: &BTreeMap<i32, usize>) -> i32 { *map.keys().next().unwrap() }

    fn black_remove(&mut self, n: i32) {
        let lo_max = Self::black_max(&self.black_lo);
        let mid_max = if self.black_mid.is_empty() { i32::MIN } else { Self::black_max(&self.black_mid) };

        if n <= lo_max {
            Self::black_erase(&mut self.black_lo, n);
        } else if n <= mid_max {
            Self::black_erase(&mut self.black_mid, n);
            self.black_sum -= n as i64;
        } else {
            Self::black_erase(&mut self.black_hi, n);
        }

        if self.black_lo.values().sum::<usize>() < self.black_k && !self.black_mid.is_empty() {
            let v = Self::black_min(&self.black_mid);
            Self::black_erase(&mut self.black_mid, v);
            self.black_sum -= v as i64;
            Self::black_insert(&mut self.black_lo, v);
        }

        if self.black_mid.values().sum::<usize>() < self.black_sz && !self.black_hi.is_empty() {
            let v = Self::black_min(&self.black_hi);
            Self::black_erase(&mut self.black_hi, v);
            Self::black_insert(&mut self.black_mid, v);
            self.black_sum += v as i64;
        }
    }

    fn black_add(&mut self, n: i32) {
        Self::black_insert(&mut self.black_lo, n);

        if self.black_lo.values().sum::<usize>() > self.black_k {
            let v = Self::black_max(&self.black_lo);
            Self::black_erase(&mut self.black_lo, v);
            Self::black_insert(&mut self.black_mid, v);
            self.black_sum += v as i64;
        }

        if self.black_mid.values().sum::<usize>() > self.black_sz {
            let v = Self::black_max(&self.black_mid);
            Self::black_erase(&mut self.black_mid, v);
            self.black_sum -= v as i64;
            Self::black_insert(&mut self.black_hi, v);
        }
    }

    fn add_element(&mut self, num: i32) {
        if self.black_pos >= self.black_m {
            let old = self.black_buf[self.black_pos % self.black_m];
            self.black_remove(old);
        }
        self.black_add(num);
        self.black_buf[self.black_pos % self.black_m] = num;
        self.black_pos += 1;
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.black_pos < self.black_m { return -1; }
        (self.black_sum / self.black_sz as i64) as i32
    }
}
```