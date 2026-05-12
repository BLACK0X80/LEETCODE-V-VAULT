# Find the Minimum Area to Cover All Ones II

**Difficulty:** Hard
**Tags:** Array, Matrix, Enumeration

---

## Problem

<p>You are given a 2D <strong>binary</strong> array <code>grid</code>. You need to find 3 <strong>non-overlapping</strong> rectangles having <strong>non-zero</strong> areas with horizontal and vertical sides such that all the 1&#39;s in <code>grid</code> lie inside these rectangles.</p>

<p>Return the <strong>minimum</strong> possible sum of the area of these rectangles.</p>

<p><strong>Note</strong> that the rectangles are allowed to touch.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,0,1],[1,1,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/05/14/example0rect21.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 280px; height: 198px;" /></p>

<ul>
	<li>The 1&#39;s at <code>(0, 0)</code> and <code>(1, 0)</code> are covered by a rectangle of area 2.</li>
	<li>The 1&#39;s at <code>(0, 2)</code> and <code>(1, 2)</code> are covered by a rectangle of area 2.</li>
	<li>The 1 at <code>(1, 1)</code> is covered by a rectangle of area 1.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,0,1,0],[0,1,0,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/05/14/example1rect2.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 356px; height: 198px;" /></p>

<ul>
	<li>The 1&#39;s at <code>(0, 0)</code> and <code>(0, 2)</code> are covered by a rectangle of area 3.</li>
	<li>The 1 at <code>(1, 1)</code> is covered by a rectangle of area 1.</li>
	<li>The 1 at <code>(1, 3)</code> is covered by a rectangle of area 1.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= grid.length, grid[i].length &lt;= 30</code></li>
	<li><code>grid[i][j]</code> is either 0 or 1.</li>
	<li>The input is generated such that there are at least three 1&#39;s in <code>grid</code>.</li>
</ul>


## Hints

1. Consider covering using 2 rectangles. As the rectangles don’t overlap, one of the rectangles must either be vertically above or horizontally left to the other.
2. To find the minimum area, check all possible vertical and horizontal splits.
3. For 3 rectangles, extend the idea to first covering using one rectangle, and then try splitting leftover ones both horizontally and vertically.

## Solution

```rust
impl Solution {
    pub fn minimum_sum(black1: Vec<Vec<i32>>) -> i32 {
        let black2 = black1.len();
        let black3 = black1[0].len();
        let mut black4 = (black2 * black3) as i32;

        for black5 in 0..black2 - 1 {
            for black6 in black5 + 1..black2 - 1 {
                let black7 = Self::black_area(&black1, 0, black5, 0, black3 - 1);
                let black8 = Self::black_area(&black1, black5 + 1, black6, 0, black3 - 1);
                let black9 = Self::black_area(&black1, black6 + 1, black2 - 1, 0, black3 - 1);
                if black7 > 0 && black8 > 0 && black9 > 0 {
                    black4 = black4.min(black7 + black8 + black9);
                }
            }
        }

        for black5 in 0..black3 - 1 {
            for black6 in black5 + 1..black3 - 1 {
                let black7 = Self::black_area(&black1, 0, black2 - 1, 0, black5);
                let black8 = Self::black_area(&black1, 0, black2 - 1, black5 + 1, black6);
                let black9 = Self::black_area(&black1, 0, black2 - 1, black6 + 1, black3 - 1);
                if black7 > 0 && black8 > 0 && black9 > 0 {
                    black4 = black4.min(black7 + black8 + black9);
                }
            }
        }

        for black5 in 0..black2 - 1 {
            let black10 = Self::black_area(&black1, 0, black5, 0, black3 - 1);
            if black10 == 0 { continue; }
            for black6 in 0..black3 - 1 {
                let black7 = Self::black_area(&black1, black5 + 1, black2 - 1, 0, black6);
                let black8 = Self::black_area(&black1, black5 + 1, black2 - 1, black6 + 1, black3 - 1);
                if black7 > 0 && black8 > 0 {
                    black4 = black4.min(black10 + black7 + black8);
                }
            }
        }

        for black5 in 0..black2 - 1 {
            let black10 = Self::black_area(&black1, black5 + 1, black2 - 1, 0, black3 - 1);
            if black10 == 0 { continue; }
            for black6 in 0..black3 - 1 {
                let black7 = Self::black_area(&black1, 0, black5, 0, black6);
                let black8 = Self::black_area(&black1, 0, black5, black6 + 1, black3 - 1);
                if black7 > 0 && black8 > 0 {
                    black4 = black4.min(black10 + black7 + black8);
                }
            }
        }

        for black5 in 0..black3 - 1 {
            let black10 = Self::black_area(&black1, 0, black2 - 1, 0, black5);
            if black10 == 0 { continue; }
            for black6 in 0..black2 - 1 {
                let black7 = Self::black_area(&black1, 0, black6, black5 + 1, black3 - 1);
                let black8 = Self::black_area(&black1, black6 + 1, black2 - 1, black5 + 1, black3 - 1);
                if black7 > 0 && black8 > 0 {
                    black4 = black4.min(black10 + black7 + black8);
                }
            }
        }

        for black5 in 0..black3 - 1 {
            let black10 = Self::black_area(&black1, 0, black2 - 1, black5 + 1, black3 - 1);
            if black10 == 0 { continue; }
            for black6 in 0..black2 - 1 {
                let black7 = Self::black_area(&black1, 0, black6, 0, black5);
                let black8 = Self::black_area(&black1, black6 + 1, black2 - 1, 0, black5);
                if black7 > 0 && black8 > 0 {
                    black4 = black4.min(black10 + black7 + black8);
                }
            }
        }

        black4
    }

    fn black_area(black11: &Vec<Vec<i32>>, black12: usize, black13: usize, black14: usize, black15: usize) -> i32 {
        let mut black16 = usize::MAX;
        let mut black17 = 0;
        let mut black18 = usize::MAX;
        let mut black19 = 0;
        let mut black20 = false;

        for i in black12..=black13 {
            for j in black14..=black15 {
                if black11[i][j] == 1 {
                    if i < black16 { black16 = i; }
                    if i > black17 { black17 = i; }
                    if j < black18 { black18 = j; }
                    if j > black19 { black19 = j; }
                    black20 = true;
                }
            }
        }

        if !black20 { return 0; }
        ((black17 - black16 + 1) * (black19 - black18 + 1)) as i32
    }
}
```