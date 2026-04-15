# Analyze Organization Hierarchy

**Difficulty:** Hard
**Tags:** Database

---

## Problem

<p>Table: <code>Employees</code></p>

<pre>
+----------------+---------+
| Column Name    | Type    | 
+----------------+---------+
| employee_id    | int     |
| employee_name  | varchar |
| manager_id     | int     |
| salary         | int     |
| department     | varchar |
+----------------+----------+
employee_id is the unique key for this table.
Each row contains information about an employee, including their ID, name, their manager&#39;s ID, salary, and department.
manager_id is null for the top-level manager (CEO).
</pre>

<p>Write a solution to analyze the organizational hierarchy and answer the following:</p>

<ol>
	<li><strong>Hierarchy Levels:</strong> For each employee, determine their level in the organization (CEO is level <code>1</code>, employees reporting directly to the CEO are level <code>2</code>, and so on).</li>
	<li><strong>Team Size:</strong> For each employee who is a manager, count the total number of employees under them (direct and indirect reports).</li>
	<li><strong>Salary Budget:</strong> For each manager, calculate the total salary budget they control (sum of salaries of all employees under them, including indirect reports, plus their own salary).</li>
</ol>

<p>Return <em>the result table ordered by&nbsp;<em>the result ordered by <strong>level</strong> in <strong>ascending</strong> order, then by <strong>budget</strong> in <strong>descending</strong> order, and finally by <strong>employee_name</strong> in <strong>ascending</strong> order</em>.</em></p>

<p><em>The result format is in the following example.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example:</strong></p>

<div class="example-block">
<p><strong>Input:</strong></p>

<p>Employees table:</p>

<pre class="example-io">
+-------------+---------------+------------+--------+-------------+
| employee_id | employee_name | manager_id | salary | department  |
+-------------+---------------+------------+--------+-------------+
| 1           | Alice         | null       | 12000  | Executive   |
| 2           | Bob           | 1          | 10000  | Sales       |
| 3           | Charlie       | 1          | 10000  | Engineering |
| 4           | David         | 2          | 7500   | Sales       |
| 5           | Eva           | 2          | 7500   | Sales       |
| 6           | Frank         | 3          | 9000   | Engineering |
| 7           | Grace         | 3          | 8500   | Engineering |
| 8           | Hank          | 4          | 6000   | Sales       |
| 9           | Ivy           | 6          | 7000   | Engineering |
| 10          | Judy          | 6          | 7000   | Engineering |
+-------------+---------------+------------+--------+-------------+
</pre>

<p><strong>Output:</strong></p>

<pre class="example-io">
+-------------+---------------+-------+-----------+--------+
| employee_id | employee_name | level | team_size | budget |
+-------------+---------------+-------+-----------+--------+
| 1           | Alice         | 1     | 9         | 84500  |
| 3           | Charlie       | 2     | 4         | 41500  |
| 2           | Bob           | 2     | 3         | 31000  |
| 6           | Frank         | 3     | 2         | 23000  |
| 4           | David         | 3     | 1         | 13500  |
| 7           | Grace         | 3     | 0         | 8500   |
| 5           | Eva           | 3     | 0         | 7500   |
| 9           | Ivy           | 4     | 0         | 7000   |
| 10          | Judy          | 4     | 0         | 7000   |
| 8           | Hank          | 4     | 0         | 6000   |
+-------------+---------------+-------+-----------+--------+
</pre>

<p><strong>Explanation:</strong></p>

<ul>
	<li><strong>Organization Structure:</strong>

	<ul>
		<li>Alice (ID: 1) is the CEO (level 1) with no manager</li>
		<li>Bob (ID: 2) and Charlie (ID: 3) report directly to Alice (level 2)</li>
		<li>David (ID: 4), Eva (ID: 5) report to Bob, while Frank (ID: 6) and Grace (ID: 7) report to Charlie (level 3)</li>
		<li>Hank (ID: 8) reports to David, and Ivy (ID: 9) and Judy (ID: 10) report to Frank (level 4)</li>
	</ul>
	</li>
	<li><strong>Level Calculation:</strong>
	<ul>
		<li>The CEO (Alice) is at level 1</li>
		<li>Each subsequent level of management adds 1 to the level</li>
	</ul>
	</li>
	<li><strong>Team Size Calculation:</strong>
	<ul>
		<li>Alice has 9 employees under her (the entire company except herself)</li>
		<li>Bob has 3 employees (David, Eva, and Hank)</li>
		<li>Charlie has 4 employees (Frank, Grace, Ivy, and Judy)</li>
		<li>David has 1 employee (Hank)</li>
		<li>Frank has 2 employees (Ivy and Judy)</li>
		<li>Eva, Grace, Hank, Ivy, and Judy have no direct reports (team_size = 0)</li>
	</ul>
	</li>
	<li><strong>Budget Calculation:</strong>
	<ul>
		<li>Alice&#39;s budget: Her salary (12000) + all employees&#39; salaries (72500) = 84500</li>
		<li>Charlie&#39;s budget: His salary (10000) + Frank&#39;s budget (23000) + Grace&#39;s salary (8500) = 41500</li>
		<li>Bob&#39;s budget: His salary (10000) + David&#39;s budget (13500) + Eva&#39;s salary (7500) = 31000</li>
		<li>Frank&#39;s budget: His salary (9000) + Ivy&#39;s salary (7000) + Judy&#39;s salary (7000) = 23000</li>
		<li>David&#39;s budget: His salary (7500) + Hank&#39;s salary (6000) = 13500</li>
		<li>Employees with no direct reports have budgets equal to their own salary</li>
	</ul>
	</li>
</ul>

<p><strong>Note:</strong></p>

<ul>
	<li>The result is ordered first by level in ascending order</li>
	<li>Within the same level, employees are ordered by budget in descending order then by name in ascending order</li>
</ul>
</div>



## Solution

```pythondata
import pandas as pd

def analyze_organization_hierarchy(employees: pd.DataFrame) -> pd.DataFrame:
    id_to_manager = employees.set_index('employee_id')['manager_id'].to_dict()
    id_to_salary = employees.set_index('employee_id')['salary'].to_dict()
    
    levels = {}
    for eid in employees['employee_id']:
        level, mid = 1, id_to_manager[eid]
        while pd.notna(mid):
            level += 1
            mid = id_to_manager.get(mid)
        levels[eid] = level

    team_size = {eid: 0 for eid in employees['employee_id']}
    budget = {eid: id_to_salary[eid] for eid in employees['employee_id']}

    for eid in employees['employee_id']:
        mid = id_to_manager[eid]
        while pd.notna(mid):
            team_size[mid] += 1
            budget[mid] += id_to_salary[eid]
            mid = id_to_manager.get(mid)

    employees['level'] = employees['employee_id'].map(levels)
    employees['team_size'] = employees['employee_id'].map(team_size)
    employees['budget'] = employees['employee_id'].map(budget)

    return (employees[['employee_id', 'employee_name', 'level', 'team_size', 'budget']]
            .sort_values(['level', 'budget', 'employee_name'], ascending=[True, False, True])
            .reset_index(drop=True))
```