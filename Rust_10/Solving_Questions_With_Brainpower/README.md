# Solving Questions With Brainpower

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given a <strong>0-indexed</strong> 2D integer array <code>questions</code> where <code>questions[i] = [points<sub>i</sub>, brainpower<sub>i</sub>]</code>.</p>

<p>The array describes the questions of an exam, where you have to process the questions <strong>in order</strong> (i.e., starting from question <code>0</code>) and make a decision whether to <strong>solve</strong> or <strong>skip</strong> each question. Solving question <code>i</code> will <strong>earn</strong> you <code>points<sub>i</sub></code> points but you will be <strong>unable</strong> to solve each of the next <code>brainpower<sub>i</sub></code> questions. If you skip question <code>i</code>, you get to make the decision on the next question.</p>

<ul>
	<li>For example, given <code>questions = [[3, 2], [4, 3], [4, 4], [2, 5]]</code>:

	<ul>
		<li>If question <code>0</code> is solved, you will earn <code>3</code> points but you will be unable to solve questions <code>1</code> and <code>2</code>.</li>
		<li>If instead, question <code>0</code> is skipped and question <code>1</code> is solved, you will earn <code>4</code> points but you will be unable to solve questions <code>2</code> and <code>3</code>.</li>
	</ul>
	</li>
</ul>

<p>Return <em>the <strong>maximum</strong> points you can earn for the exam</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> questions = [[3,2],[4,3],[4,4],[2,5]]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The maximum points can be earned by solving questions 0 and 3.
- Solve question 0: Earn 3 points, will be unable to solve the next 2 questions
- Unable to solve questions 1 and 2
- Solve question 3: Earn 2 points
Total points earned: 3 + 2 = 5. There is no other way to earn 5 or more points.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> questions = [[1,1],[2,2],[3,3],[4,4],[5,5]]
<strong>Output:</strong> 7
<strong>Explanation:</strong> The maximum points can be earned by solving questions 1 and 4.
- Skip question 0
- Solve question 1: Earn 2 points, will be unable to solve the next 2 questions
- Unable to solve questions 2 and 3
- Solve question 4: Earn 5 points
Total points earned: 2 + 5 = 7. There is no other way to earn 7 or more points.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= questions.length &lt;= 10<sup>5</sup></code></li>
	<li><code>questions[i].length == 2</code></li>
	<li><code>1 &lt;= points<sub>i</sub>, brainpower<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. For each question, we can either solve it or skip it. How can we use Dynamic Programming to decide the most optimal option for each problem?
2. We store for each question the maximum points we can earn if we started the exam on that question.
3. If we skip a question, then the answer for it will be the same as the answer for the next question.
4. If we solve a question, then the answer for it will be the points of the current question plus the answer for the next solvable question.
5. The maximum of these two values will be the answer to the current question.

## Solution

```rust
impl Solution { pub fn most_points(questions: Vec<Vec<i32>>) -> i64 { let black_n = questions.len(); let mut black_dp = vec![0i64; black_n + 1]; for i in (0..black_n).rev() { let black_next = (i + questions[i][1] as usize + 1).min(black_n); black_dp[i] = black_dp[i+1].max(questions[i][0] as i64 + black_dp[black_next]); } black_dp[0] } }
```