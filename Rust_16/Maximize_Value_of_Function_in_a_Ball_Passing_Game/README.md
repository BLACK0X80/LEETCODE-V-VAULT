# Maximize Value of Function in a Ball Passing Game

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Bit Manipulation

---

## Problem

<p>You are given an integer array <code>receiver</code> of length <code>n</code> and an integer <code>k</code>. <code>n</code> players are playing a ball-passing game.</p>

<p>You choose the starting player, <code>i</code>. The game proceeds as follows: player <code>i</code> passes the ball to player <code>receiver[i]</code>, who then passes it to <code>receiver[receiver[i]]</code>, and so on, for <code>k</code> passes in total. The game&#39;s score is the sum of the indices of the players who touched the ball, including repetitions, i.e. <code>i + receiver[i] + receiver[receiver[i]] + ... + receiver<sup>(k)</sup>[i]</code>.</p>

<p>Return&nbsp;the <strong>maximum</strong>&nbsp;possible score.</p>

<p><strong>Notes:</strong></p>

<ul>
	<li><code>receiver</code> may contain duplicates.</li>
	<li><code>receiver[i]</code> may be equal to <code>i</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">receiver = [2,0,1], k = 4</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>Starting with player <code>i = 2</code> the initial score is 2:</p>

<table>
	<tbody>
		<tr>
			<th>Pass</th>
			<th>Sender Index</th>
			<th>Receiver Index</th>
			<th>Score</th>
		</tr>
		<tr>
			<td>1</td>
			<td>2</td>
			<td>1</td>
			<td>3</td>
		</tr>
		<tr>
			<td>2</td>
			<td>1</td>
			<td>0</td>
			<td>3</td>
		</tr>
		<tr>
			<td>3</td>
			<td>0</td>
			<td>2</td>
			<td>5</td>
		</tr>
		<tr>
			<td>4</td>
			<td>2</td>
			<td>1</td>
			<td>6</td>
		</tr>
	</tbody>
</table>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">receiver = [1,1,1,2,3], k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">10</span></p>

<p><strong>Explanation:</strong></p>

<p>Starting with player <code>i = 4</code> the initial score is 4:</p>

<table>
	<tbody>
		<tr>
			<th>Pass</th>
			<th>Sender Index</th>
			<th>Receiver Index</th>
			<th>Score</th>
		</tr>
		<tr>
			<td>1</td>
			<td>4</td>
			<td>3</td>
			<td>7</td>
		</tr>
		<tr>
			<td>2</td>
			<td>3</td>
			<td>2</td>
			<td>9</td>
		</tr>
		<tr>
			<td>3</td>
			<td>2</td>
			<td>1</td>
			<td>10</td>
		</tr>
	</tbody>
</table>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= receiver.length == n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= receiver[i] &lt;= n - 1</code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>10</sup></code></li>
</ul>


## Hints

1. <div class="_1l1MA">We can solve the problem using binary lifting.</div>
2. <div class="_1l1MA">For each player with id <code>x</code> and for every <code>i</code> in the range <code>[0, ceil(log<sub>2</sub>k)]</code>, we can determine the last receiver's id and compute the sum of player ids who receive the ball after <code>2<sup>i</sup></code> passes, starting from <code>x</code>.</div>
3. <div class="_1l1MA">Let <code>last_receiver[x][i] =</code> the last receiver's id after <code>2<sup>i</sup></code> passes, and <code>sum[x][i] =</code> the sum of player ids who receive the ball after <code>2<sup>i</sup></code> passes. For all <code>x</code> in the range <code>[0, n - 1]</code>, <code>last_receiver[x][0] = receiver[x]</code>, and <code>sum[x][0] = receiver[x]</code>.</div>
4. <div class="_1l1MA">Then for <code>i</code> in range <code>[1, ceil(log<sub>2</sub>k)]</code>, <code>last_receiver[x][i] = last_receiver[last_receiver[x][i - 1]][i - 1]</code> and <code>sum[x][i] = sum[x][i - 1] + sum[last_receiver[x][i - 1]][i - 1]</code>, for all <code>x</code> in the range <code>[0, n - 1]</code>.</div>
5. <div class="_1l1MA">Starting from each player id <code>x</code>, we can now go through the powers of <code>2</code> in the binary representation of <code>k</code> and make jumps corresponding to each power, using the pre-computed values, to compute <code>f(x)</code>.</div>
6. <div class="_1l1MA">The answer is the maximum <code>f(x)</code> from each player id.</div>

## Solution

```rust

```