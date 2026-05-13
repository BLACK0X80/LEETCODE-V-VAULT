# Sort the Students by Their Kth Score

**Difficulty:** Medium
**Tags:** Array, Sorting, Matrix

---

## Problem

<p>There is a class with <code>m</code> students and <code>n</code> exams. You are given a <strong>0-indexed</strong> <code>m x n</code> integer matrix <code>score</code>, where each row represents one student and <code>score[i][j]</code> denotes the score the <code>i<sup>th</sup></code> student got in the <code>j<sup>th</sup></code> exam. The matrix <code>score</code> contains <strong>distinct</strong> integers only.</p>

<p>You are also given an integer <code>k</code>. Sort the students (i.e., the rows of the matrix) by their scores in the <code>k<sup>th</sup></code>&nbsp;(<strong>0-indexed</strong>) exam from the highest to the lowest.</p>

<p>Return <em>the matrix after sorting it.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/11/30/example1.png" style="width: 600px; height: 136px;" />
<pre>
<strong>Input:</strong> score = [[10,6,9,1],[7,5,11,2],[4,8,3,15]], k = 2
<strong>Output:</strong> [[7,5,11,2],[10,6,9,1],[4,8,3,15]]
<strong>Explanation:</strong> In the above diagram, S denotes the student, while E denotes the exam.
- The student with index 1 scored 11 in exam 2, which is the highest score, so they got first place.
- The student with index 0 scored 9 in exam 2, which is the second highest score, so they got second place.
- The student with index 2 scored 3 in exam 2, which is the lowest score, so they got third place.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/11/30/example2.png" style="width: 486px; height: 121px;" />
<pre>
<strong>Input:</strong> score = [[3,4],[5,6]], k = 0
<strong>Output:</strong> [[5,6],[3,4]]
<strong>Explanation:</strong> In the above diagram, S denotes the student, while E denotes the exam.
- The student with index 1 scored 5 in exam 0, which is the highest score, so they got first place.
- The student with index 0 scored 3 in exam 0, which is the lowest score, so they got second place.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == score.length</code></li>
	<li><code>n == score[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 250</code></li>
	<li><code>1 &lt;= score[i][j] &lt;= 10<sup>5</sup></code></li>
	<li><code>score</code> consists of <strong>distinct</strong> integers.</li>
	<li><code>0 &lt;= k &lt; n</code></li>
</ul>


## Hints

1. Find the row with the highest score in the kth exam and swap it with the first row.
2. After fixing the first row, perform the same operation for the rest of the rows, and the matrix's rows will get sorted one by one.

## Solution

```rust
impl Solution { pub fn sort_the_students(mut black_score: Vec<Vec<i32>>, black_k: i32) -> Vec<Vec<i32>> { black_score.sort_unstable_by_key(|black_row| -black_row[black_k as usize]); black_score } }
```