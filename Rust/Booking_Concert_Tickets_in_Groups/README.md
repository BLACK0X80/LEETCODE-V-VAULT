# Booking Concert Tickets in Groups

**Difficulty:** Hard
**Tags:** Binary Search, Design, Binary Indexed Tree, Segment Tree

---

## Problem

<p>A concert hall has <code>n</code> rows numbered from <code>0</code> to <code>n - 1</code>, each with <code>m</code> seats, numbered from <code>0</code> to <code>m - 1</code>. You need to design a ticketing system that can allocate seats in the following cases:</p>

<ul>
	<li>If a group of <code>k</code> spectators can sit <strong>together</strong> in a row.</li>
	<li>If <strong>every</strong> member of a group of <code>k</code> spectators can get a seat. They may or <strong>may not</strong> sit together.</li>
</ul>

<p>Note that the spectators are very picky. Hence:</p>

<ul>
	<li>They will book seats only if each member of their group can get a seat with row number <strong>less than or equal</strong> to <code>maxRow</code>. <code>maxRow</code> can <strong>vary</strong> from group to group.</li>
	<li>In case there are multiple rows to choose from, the row with the <strong>smallest</strong> number is chosen. If there are multiple seats to choose in the same row, the seat with the <strong>smallest</strong> number is chosen.</li>
</ul>

<p>Implement the <code>BookMyShow</code> class:</p>

<ul>
	<li><code>BookMyShow(int n, int m)</code> Initializes the object with <code>n</code> as number of rows and <code>m</code> as number of seats per row.</li>
	<li><code>int[] gather(int k, int maxRow)</code> Returns an array of length <code>2</code> denoting the row and seat number (respectively) of the <strong>first seat</strong> being allocated to the <code>k</code> members of the group, who must sit <strong>together</strong>. In other words, it returns the smallest possible <code>r</code> and <code>c</code> such that all <code>[c, c + k - 1]</code> seats are valid and empty in row <code>r</code>, and <code>r &lt;= maxRow</code>. Returns <code>[]</code> in case it is <strong>not possible</strong> to allocate seats to the group.</li>
	<li><code>boolean scatter(int k, int maxRow)</code> Returns <code>true</code> if all <code>k</code> members of the group can be allocated seats in rows <code>0</code> to <code>maxRow</code>, who may or <strong>may not</strong> sit together. If the seats can be allocated, it allocates <code>k</code> seats to the group with the <strong>smallest</strong> row numbers, and the smallest possible seat numbers in each row. Otherwise, returns <code>false</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;BookMyShow&quot;, &quot;gather&quot;, &quot;gather&quot;, &quot;scatter&quot;, &quot;scatter&quot;]
[[2, 5], [4, 0], [2, 0], [5, 1], [5, 1]]
<strong>Output</strong>
[null, [0, 0], [], true, false]

<strong>Explanation</strong>
BookMyShow bms = new BookMyShow(2, 5); // There are 2 rows with 5 seats each 
bms.gather(4, 0); // return [0, 0]
                  // The group books seats [0, 3] of row 0. 
bms.gather(2, 0); // return []
                  // There is only 1 seat left in row 0,
                  // so it is not possible to book 2 consecutive seats. 
bms.scatter(5, 1); // return True
                   // The group books seat 4 of row 0 and seats [0, 3] of row 1. 
bms.scatter(5, 1); // return False
                   // There is only one seat left in the hall.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= m, k &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= maxRow &lt;= n - 1</code></li>
	<li>At most <code>5 * 10<sup>4</sup></code> calls <strong>in total</strong> will be made to <code>gather</code> and <code>scatter</code>.</li>
</ul>


## Hints

1. Since seats are allocated by smallest row and then by smallest seat numbers, how can we keep a record of the smallest seat number vacant in each row?
2. How can range max query help us to check if contiguous seats can be allocated in a range?
3. Similarly, can range sum query help us to check if enough seats are available in a range?
4. Which data structure can be used to implement the above?

## Solution

```rust
struct BookMyShow {
    black_n: usize,
    black_m: i64,
    black_min_tree: Vec<i64>,
    black_sum_tree: Vec<i64>,
}

impl BookMyShow {
    fn new(n: i32, m: i32) -> Self {
        let mut black_pow2 = 1;
        while black_pow2 < n as usize { black_pow2 *= 2; }
        Self {
            black_n: black_pow2,
            black_m: m as i64,
            black_min_tree: vec![0; 2 * black_pow2],
            black_sum_tree: vec![0; 2 * black_pow2],
        }
    }

    fn black_update(&mut self, mut black_idx: usize, black_val: i64) {
        black_idx += self.black_n;
        self.black_min_tree[black_idx] += black_val;
        self.black_sum_tree[black_idx] += black_val;
        while black_idx > 1 {
            black_idx /= 2;
            self.black_min_tree[black_idx] = self.black_min_tree[2 * black_idx].min(self.black_min_tree[2 * black_idx + 1]);
            self.black_sum_tree[black_idx] = self.black_sum_tree[2 * black_idx] + self.black_sum_tree[2 * black_idx + 1];
        }
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        let black_k = k as i64;
        let mut black_curr = 1;
        if self.black_min_tree[black_curr] > self.black_m - black_k { return vec![]; }
        while black_curr < self.black_n {
            if self.black_min_tree[2 * black_curr] <= self.black_m - black_k { black_curr *= 2; }
            else { black_curr = 2 * black_curr + 1; }
        }
        let black_row = black_curr - self.black_n;
        if black_row > max_row as usize { return vec![]; }
        let black_seat = self.black_min_tree[black_curr];
        self.black_update(black_row, black_k);
        vec![black_row as i32, black_seat as i32]
    }

    fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        let mut black_k = k as i64;
        if self.black_query_sum(0, max_row as usize) < black_k { return false; }
        for black_r in 0..=max_row as usize {
            let black_taken = (self.black_m - self.black_min_tree[black_r + self.black_n]).min(black_k);
            if black_taken > 0 {
                self.black_update(black_r, black_taken);
                black_k -= black_taken;
            }
            if black_k == 0 { break; }
        }
        true
    }

    fn black_query_sum(&self, mut black_l: usize, mut black_r: usize) -> i64 {
        black_l += self.black_n; black_r += self.black_n;
        let mut black_s = 0;
        let black_total = (black_r - black_l + 1) as i64 * self.black_m;
        while black_l <= black_r {
            if black_l % 2 == 1 { black_s += self.black_sum_tree[black_l]; black_l += 1; }
            if black_r % 2 == 0 { black_s += self.black_sum_tree[black_r]; black_r -= 1; }
            black_l /= 2; black_r /= 2;
        }
        black_total - black_s
    }
}
```