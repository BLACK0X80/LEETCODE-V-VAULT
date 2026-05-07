# Number of Valid Move Combinations On Chessboard

**Difficulty:** Hard
**Tags:** Array, String, Backtracking, Simulation

---

## Problem

<p>There is an <code>8 x 8</code> chessboard containing <code>n</code> pieces (rooks, queens, or bishops). You are given a string array <code>pieces</code> of length <code>n</code>, where <code>pieces[i]</code> describes the type (rook, queen, or bishop) of the <code>i<sup>th</sup></code> piece. In addition, you are given a 2D integer array <code>positions</code> also of length <code>n</code>, where <code>positions[i] = [r<sub>i</sub>, c<sub>i</sub>]</code> indicates that the <code>i<sup>th</sup></code> piece is currently at the <strong>1-based</strong> coordinate <code>(r<sub>i</sub>, c<sub>i</sub>)</code> on the chessboard.</p>

<p>When making a <strong>move</strong> for a piece, you choose a <strong>destination</strong> square that the piece will travel toward and stop on.</p>

<ul>
	<li>A rook can only travel <strong>horizontally or vertically</strong> from <code>(r, c)</code> to the direction of <code>(r+1, c)</code>, <code>(r-1, c)</code>, <code>(r, c+1)</code>, or <code>(r, c-1)</code>.</li>
	<li>A queen can only travel <strong>horizontally, vertically, or diagonally</strong> from <code>(r, c)</code> to the direction of <code>(r+1, c)</code>, <code>(r-1, c)</code>, <code>(r, c+1)</code>, <code>(r, c-1)</code>, <code>(r+1, c+1)</code>, <code>(r+1, c-1)</code>, <code>(r-1, c+1)</code>, <code>(r-1, c-1)</code>.</li>
	<li>A bishop can only travel <strong>diagonally</strong> from <code>(r, c)</code> to the direction of <code>(r+1, c+1)</code>, <code>(r+1, c-1)</code>, <code>(r-1, c+1)</code>, <code>(r-1, c-1)</code>.</li>
</ul>

<p>You must make a <strong>move</strong> for every piece on the board simultaneously. A <strong>move combination</strong> consists of all the <strong>moves</strong> performed on all the given pieces. Every second, each piece will instantaneously travel <strong>one square</strong> towards their destination if they are not already at it. All pieces start traveling at the <code>0<sup>th</sup></code> second. A move combination is <strong>invalid</strong> if, at a given time, <strong>two or more</strong> pieces occupy the same square.</p>

<p>Return <em>the number of <strong>valid</strong> move combinations</em>​​​​​.</p>

<p><strong>Notes:</strong></p>

<ul>
	<li><strong>No two pieces</strong> will start in the<strong> same</strong> square.</li>
	<li>You may choose the square a piece is already on as its <strong>destination</strong>.</li>
	<li>If two pieces are <strong>directly adjacent</strong> to each other, it is valid for them to <strong>move past each other</strong> and swap positions in one second.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/09/23/a1.png" style="width: 215px; height: 215px;" />
<pre>
<strong>Input:</strong> pieces = [&quot;rook&quot;], positions = [[1,1]]
<strong>Output:</strong> 15
<strong>Explanation:</strong> The image above shows the possible squares the piece can move to.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/09/23/a2.png" style="width: 215px; height: 215px;" />
<pre>
<strong>Input:</strong> pieces = [&quot;queen&quot;], positions = [[1,1]]
<strong>Output:</strong> 22
<strong>Explanation:</strong> The image above shows the possible squares the piece can move to.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/09/23/a3.png" style="width: 214px; height: 215px;" />
<pre>
<strong>Input:</strong> pieces = [&quot;bishop&quot;], positions = [[4,3]]
<strong>Output:</strong> 12
<strong>Explanation:</strong> The image above shows the possible squares the piece can move to.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == pieces.length </code></li>
	<li><code>n == positions.length</code></li>
	<li><code>1 &lt;= n &lt;= 4</code></li>
	<li><code>pieces</code> only contains the strings <code>&quot;rook&quot;</code>, <code>&quot;queen&quot;</code>, and <code>&quot;bishop&quot;</code>.</li>
	<li>There will be at most one queen on the chessboard.</li>
	<li><code>1 &lt;= r<sub>i</sub>, c<sub>i</sub> &lt;= 8</code></li>
	<li>Each <code>positions[i]</code> is distinct.</li>
</ul>


## Hints

1. N is small, we can generate all possible move combinations.
2. For each possible move combination, determine which ones are valid.

## Solution

```rust
impl Solution {
    pub fn count_combinations(black1: Vec<String>, black2: Vec<Vec<i32>>) -> i32 {
        let black3 = black1.len();
        let mut black4 = vec![];
        for black5 in 0..black3 {
            let mut black6 = vec![(0, 0, 0)];
            let black7 = if black1[black5] == "rook" { vec![(0,1),(0,-1),(1,0),(-1,0)] }
                        else if black1[black5] == "bishop" { vec![(1,1),(1,-1),(-1,1),(-1,-1)] }
                        else { vec![(0,1),(0,-1),(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1)] };
            for (black8, black9) in black7 {
                for black10 in 1..8 {
                    let (black11, black12) = (black2[black5][0] + black8 * black10, black2[black5][1] + black9 * black10);
                    if black11 >= 1 && black11 <= 8 && black12 >= 1 && black12 <= 8 {
                        black6.push((black8, black9, black10));
                    } else { break; }
                }
            }
            black4.push(black6);
        }

        let mut black13 = vec![(0, 0, 0); black3];
        fn black14(black15: usize, black16: &Vec<Vec<(i32, i32, i32)>>, black17: &mut Vec<(i32, i32, i32)>, black18: &Vec<Vec<i32>>) -> i32 {
            if black15 == black16.len() {
                for black19 in 1..8 {
                    let mut black20 = 0;
                    let mut black21 = std::collections::HashSet::new();
                    for black22 in 0..black16.len() {
                        let (black23, black24, black25) = black17[black22];
                        let black26 = black19.min(black25);
                        let (black27, black28) = (black18[black22][0] + black23 * black26, black18[black22][1] + black24 * black26);
                        if black21.contains(&(black27, black28)) { return 0; }
                        black21.insert((black27, black28));
                        if black26 < black25 { black20 += 1; }
                    }
                    if black20 == 0 { break; }
                }
                return 1;
            }
            let mut black29 = 0;
            for &black30 in &black16[black15] {
                black17[black15] = black30;
                black29 += black14(black15 + 1, black16, black17, black18);
            }
            black29
        }
        black14(0, &black4, &mut black13, &black2)
    }
}
```