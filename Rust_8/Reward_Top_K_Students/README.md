# Reward Top K Students

**Difficulty:** Medium
**Tags:** Array, Hash Table, String, Sorting, Heap (Priority Queue)

---

## Problem

<p>You are given two string arrays <code>positive_feedback</code> and <code>negative_feedback</code>, containing the words denoting positive and negative feedback, respectively. Note that <strong>no</strong> word is both positive and negative.</p>

<p>Initially every student has <code>0</code> points. Each positive word in a feedback report <strong>increases</strong> the points of a student by <code>3</code>, whereas each negative word <strong>decreases</strong> the points by <code>1</code>.</p>

<p>You are given <code>n</code> feedback reports, represented by a <strong>0-indexed</strong> string array <code>report</code>&nbsp;and a <strong>0-indexed</strong> integer array <code>student_id</code>, where <code>student_id[i]</code> represents the ID of the student who has received the feedback report <code>report[i]</code>. The ID of each student is <strong>unique</strong>.</p>

<p>Given an integer <code>k</code>, return <em>the top </em><code>k</code><em> students after ranking them in <strong>non-increasing</strong> order by their points</em>. In case more than one student has the same points, the one with the lower ID ranks higher.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> positive_feedback = [&quot;smart&quot;,&quot;brilliant&quot;,&quot;studious&quot;], negative_feedback = [&quot;not&quot;], report = [&quot;this student is studious&quot;,&quot;the student is smart&quot;], student_id = [1,2], k = 2
<strong>Output:</strong> [1,2]
<strong>Explanation:</strong> 
Both the students have 1 positive feedback and 3 points but since student 1 has a lower ID he ranks higher.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> positive_feedback = [&quot;smart&quot;,&quot;brilliant&quot;,&quot;studious&quot;], negative_feedback = [&quot;not&quot;], report = [&quot;this student is not studious&quot;,&quot;the student is smart&quot;], student_id = [1,2], k = 2
<strong>Output:</strong> [2,1]
<strong>Explanation:</strong> 
- The student with ID 1 has 1 positive feedback and 1 negative feedback, so he has 3-1=2 points. 
- The student with ID 2 has 1 positive feedback, so he has 3 points. 
Since student 2 has more points, [2,1] is returned.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= positive_feedback.length, negative_feedback.length &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= positive_feedback[i].length, negative_feedback[j].length &lt;= 100</code></li>
	<li>Both <code>positive_feedback[i]</code> and <code>negative_feedback[j]</code> consists of lowercase English letters.</li>
	<li>No word is present in both <code>positive_feedback</code> and <code>negative_feedback</code>.</li>
	<li><code>n == report.length == student_id.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>4</sup></code></li>
	<li><code>report[i]</code> consists of lowercase English letters and spaces <code>&#39; &#39;</code>.</li>
	<li>There is a single space between consecutive words of <code>report[i]</code>.</li>
	<li><code>1 &lt;= report[i].length &lt;= 100</code></li>
	<li><code>1 &lt;= student_id[i] &lt;= 10<sup>9</sup></code></li>
	<li>All the values of <code>student_id[i]</code> are <strong>unique</strong>.</li>
	<li><code>1 &lt;= k &lt;= n</code></li>
</ul>


## Hints

1. Hash the positive and negative feedback words separately.
2. Calculate the points for each student’s feedback.
3. Sort the students accordingly to find the top <em>k</em> among them.

## Solution

```rust
impl Solution { pub fn top_students(black_pos: Vec<String>, black_neg: Vec<String>, black_rep: Vec<String>, black_ids: Vec<i32>, black_k: i32) -> Vec<i32> { let black_p_set: std::collections::HashSet<_> = black_pos.into_iter().collect(); let black_n_set: std::collections::HashSet<_> = black_neg.into_iter().collect(); let mut black_scores: Vec<(i32, i32)> = black_rep.iter().zip(black_ids.iter()).map(|(black_r, &black_id)| { let mut black_s = 0; for black_w in black_r.split_whitespace() { if black_p_set.contains(black_w) { black_s += 3; } else if black_n_set.contains(black_w) { black_s -= 1; } } (-black_s, black_id) }).collect(); black_scores.sort_unstable(); black_scores.into_iter().take(black_k as usize).map(|black_pair| black_pair.1).collect() } }
```