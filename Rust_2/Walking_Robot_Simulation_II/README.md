# Walking Robot Simulation II

**Difficulty:** Medium
**Tags:** Design, Simulation

---

## Problem

<p>A <code>width x height</code> grid is on an XY-plane with the <strong>bottom-left</strong> cell at <code>(0, 0)</code> and the <strong>top-right</strong> cell at <code>(width - 1, height - 1)</code>. The grid is aligned with the four cardinal directions (<code>&quot;North&quot;</code>, <code>&quot;East&quot;</code>, <code>&quot;South&quot;</code>, and <code>&quot;West&quot;</code>). A robot is <strong>initially</strong> at cell <code>(0, 0)</code> facing direction <code>&quot;East&quot;</code>.</p>

<p>The robot can be instructed to move for a specific number of <strong>steps</strong>. For each step, it does the following.</p>

<ol>
	<li>Attempts to move <strong>forward one</strong> cell in the direction it is facing.</li>
	<li>If the cell the robot is <strong>moving to</strong> is <strong>out of bounds</strong>, the robot instead <strong>turns</strong> 90 degrees <strong>counterclockwise</strong> and retries the step.</li>
</ol>

<p>After the robot finishes moving the number of steps required, it stops and awaits the next instruction.</p>

<p>Implement the <code>Robot</code> class:</p>

<ul>
	<li><code>Robot(int width, int height)</code> Initializes the <code>width x height</code> grid with the robot at <code>(0, 0)</code> facing <code>&quot;East&quot;</code>.</li>
	<li><code>void step(int num)</code> Instructs the robot to move forward <code>num</code> steps.</li>
	<li><code>int[] getPos()</code> Returns the current cell the robot is at, as an array of length 2, <code>[x, y]</code>.</li>
	<li><code>String getDir()</code> Returns the current direction of the robot, <code>&quot;North&quot;</code>, <code>&quot;East&quot;</code>, <code>&quot;South&quot;</code>, or <code>&quot;West&quot;</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="example-1" src="https://assets.leetcode.com/uploads/2021/10/09/example-1.png" style="width: 498px; height: 268px;" />
<pre>
<strong>Input</strong>
[&quot;Robot&quot;, &quot;step&quot;, &quot;step&quot;, &quot;getPos&quot;, &quot;getDir&quot;, &quot;step&quot;, &quot;step&quot;, &quot;step&quot;, &quot;getPos&quot;, &quot;getDir&quot;]
[[6, 3], [2], [2], [], [], [2], [1], [4], [], []]
<strong>Output</strong>
[null, null, null, [4, 0], &quot;East&quot;, null, null, null, [1, 2], &quot;West&quot;]

<strong>Explanation</strong>
Robot robot = new Robot(6, 3); // Initialize the grid and the robot at (0, 0) facing East.
robot.step(2);  // It moves two steps East to (2, 0), and faces East.
robot.step(2);  // It moves two steps East to (4, 0), and faces East.
robot.getPos(); // return [4, 0]
robot.getDir(); // return &quot;East&quot;
robot.step(2);  // It moves one step East to (5, 0), and faces East.
                // Moving the next step East would be out of bounds, so it turns and faces North.
                // Then, it moves one step North to (5, 1), and faces North.
robot.step(1);  // It moves one step North to (5, 2), and faces <strong>North</strong> (not West).
robot.step(4);  // Moving the next step North would be out of bounds, so it turns and faces West.
                // Then, it moves four steps West to (1, 2), and faces West.
robot.getPos(); // return [1, 2]
robot.getDir(); // return &quot;West&quot;

</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= width, height &lt;= 100</code></li>
	<li><code>1 &lt;= num &lt;= 10<sup>5</sup></code></li>
	<li>At most <code>10<sup>4</sup></code> calls <strong>in total</strong> will be made to <code>step</code>, <code>getPos</code>, and <code>getDir</code>.</li>
</ul>


## Hints

1. The robot only moves along the perimeter of the grid. Can you think if modulus can help you quickly compute which cell it stops at?
2. After the robot moves one time, whenever the robot stops at some cell, it will always face a specific direction. i.e., The direction it faces is determined by the cell it stops at.
3. Can you precompute what direction it faces when it stops at each cell along the perimeter, and reuse the results?

## Solution

```rust
struct Robot {
    black1: i32,
    black2: i32,
    black3: i32,
    black4: i32,
    black5: bool,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Self {
            black1: width,
            black2: height,
            black3: 0,
            black4: (width + height - 2) * 2,
            black5: false,
        }
    }

    fn step(&mut self, num: i32) {
        self.black5 = true;
        self.black3 = (self.black3 + num) % self.black4;
    }

    fn get_pos(&self) -> Vec<i32> {
        let (w, h) = (self.black1, self.black2);
        let s = self.black3;
        if s < w {
            vec![s, 0]
        } else if s < w + h - 1 {
            vec![w - 1, s - (w - 1)]
        } else if s < 2 * w + h - 2 {
            vec![w - 1 - (s - (w + h - 2)), h - 1]
        } else {
            vec![0, h - 1 - (s - (2 * w + h - 3))]
        }
    }

    fn get_dir(&self) -> String {
        let (w, h) = (self.black1, self.black2);
        let s = self.black3;
        if !self.black5 { return "East".to_string(); }
        if s > 0 && s < w {
            "East".to_string()
        } else if s >= w && s < w + h - 1 {
            "North".to_string()
        } else if s >= w + h - 1 && s < 2 * w + h - 2 {
            "West".to_string()
        } else if s >= 2 * w + h - 2 && s < self.black4 {
            "South".to_string()
        } else {
            "South".to_string()
        }
    }
}
```