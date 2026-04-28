<div align="center">

<img src="https://capsule-render.vercel.app/api?type=transparent&color=gradient&customColorList=6,11,20&height=120&section=header&text=LEETCODE-V-VAULT&fontSize=70&fontColor=ffffff&fontAlignY=50" width="100%"/>

<p align="center">
  <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&weight=700&size=22&duration=3000&pause=1000&color=FFFFFF&center=true&vCenter=true&multiline=true&width=800&height=100&lines=1%2C450+Problems+Solved+%7C+Global+Rank+%237%2C590;Rust+%7C+Python+%7C+C%2B%2B+%7C+Auto-Synced+Daily" alt="Typing Animation" />
</p>

<br/>

[![LeetCode](https://img.shields.io/badge/LeetCode-BBLACK0X80-FFFFFF?style=for-the-badge&logo=leetcode&logoColor=black)](https://leetcode.com/u/BBLACK0X80/)
[![Rank](https://img.shields.io/badge/Global_Rank-%237%2C590-333333?style=for-the-badge&logo=trophy&logoColor=white)](https://leetcode.com/u/BBLACK0X80/)
[![Solved](https://img.shields.io/badge/Solved-1%2C450%2F3%2C907-111111?style=for-the-badge&logo=checkmarx&logoColor=white)](https://leetcode.com/u/BBLACK0X80/)
[![Hard](https://img.shields.io/badge/Hard-746%2F924-CC0000?style=for-the-badge&logo=target&logoColor=white)](https://leetcode.com/u/BBLACK0X80/)

[![Rust](https://img.shields.io/badge/Rust-Primary_Language-000000?style=for-the-badge&logo=rust&logoColor=white)](./Rust)
[![Auto Sync](https://img.shields.io/badge/Sync-Daily_Auto-222222?style=for-the-badge&logo=githubactions&logoColor=white)](/.github/workflows)
[![License](https://img.shields.io/badge/License-MIT-444444?style=for-the-badge&logo=balance-scale&logoColor=white)](LICENSE)

<br/>

**Automated - Structured - Battle-Tested**

A self-syncing archive of every accepted LeetCode solution — automatically fetched, organized by language, and pushed daily via GitHub Actions. No manual uploads. No missing solutions. Absolute coverage.

[**Browse Solutions**](#language-breakdown) - [**How It Works**](#how-it-works) - [**Stats**](#competitive-stats)

</div>

---

## Competitive Stats

<div align="center">

![LeetCode Stats](https://leetcard.jacoblin.cool/BBLACK0X80?theme=dark&font=Fira%20Code&ext=heatmap)

</div>

<br/>

<div align="center">

| Metric | Value |
|:------:|:-----:|
| Global Rank | **#7,590** |
| Problems Solved | **1,450 / 3,907** |
| Easy | 79 / 938 |
| Medium | 538 / 2,045 |
| Hard | **746 / 924** |

</div>

---

## Performance Benchmark

<div align="center">

### Difficulty Completion Rate

| Category | Solved | Total | Completion |
|:--------:|:------:|:-----:|:----------:|
| Easy | 79 | 938 | 8.4% |
| Medium | 538 | 2,045 | 26.3% |
| **Hard** | **746** | **924** | **80.7%** |
| **Overall** | **1,363** | **3,907** | **34.9%** |

<br/>

### Global Standing

| Metric | Value | Standing |
|:------:|:-----:|:--------:|
| Global Rank | **#7,590** | Top 0.25% worldwide |
| Hard Solved | **746 / 924** | 80.7% completion |
| Acceptance Rate | **86.48%** | Precision over volume |

<br/>

### Language Dominance

| Language | Problems | Share |
|:--------:|:--------:|:-----:|
| **Rust** | 1,307 | ~95.8% |
| Python / Pandas | 34 | ~2.5% |
| C++ | 12 | ~0.9% |
| BASH | 4 | ~0.3% |
| TypeScript | 2 | ~0.1% |
| Go / Java / C | 3 | ~0.2% |

> Solving 1,307 Hard-heavy problems in Rust — a language with zero runtime safety nets — places this vault in a category of its own.

</div>

---

## Language Breakdown

<div align="center">

| Language | Folder |
|:--------:|:------:|
| **Rust** (1-1000) | [`/Rust`](./Rust) |
| **Rust** (1001+) | [`/Rust_2`](./Rust_2) |
| **Python 3** | [`/Python`](./Python) |
| **Pandas** | [`/PYTHONDATA`](./PYTHONDATA) |
| **C++** | [`/C++`](./C++) |
| **C** | [`/C`](./C) |
| **Java** | [`/Java`](./Java) |
| **TypeScript** | [`/TypeScript`](./TypeScript) |
| **Go** | [`/GOLANG`](./GOLANG) |
| **BASH** | [`/BASH`](./BASH) |

> When any language exceeds 1,000 solutions, the vault automatically overflows into a sequenced folder (`Rust_2`, `Rust_3`, ...).

</div>

---

## How It Works

The vault runs on a custom `sync.js` engine backed by GitHub Actions. Every day at midnight UTC, or on manual trigger, the pipeline executes a full incremental sync:

```
LeetCode GraphQL API
        |
        v
  Fetch all accepted submissions
  (paginated - deduplicated by slug)
        |
        v
  For each new solution:
  |-- fetchCode()      -> submissionDetails query
  |-- fetchQuestion()  -> questionData query
  +-- Write:
       |-- solution.<ext>   <- accepted code
       +-- README.md        <- problem + difficulty + tags + hints + solution
        |
        v
  Organize by language folder
  (overflow at 1,000 -> new sequenced folder)
        |
        v
  Regenerate root README.md with full index
        |
        v
  git commit -> git push
```

---

## Vault Structure

```
LEETCODE-V-VAULT/
|-- Rust/                  <- solutions 1-1000
|-- Rust_2/                <- solutions 1001+
|-- Python/
|-- PYTHONDATA/
|-- C++/
|-- C/
|-- Java/
|-- TypeScript/
|-- GOLANG/
|-- BASH/
|-- sync.js                <- sync engine
|-- .github/workflows/     <- daily cron pipeline
+-- README.md              <- auto-regenerated index
```

Each problem lives in its own folder with exactly two files:

```
Rust/Two_Sum/
|-- solution.rs     <- accepted code
+-- README.md       <- full problem + hints + solution
```

---

## Top Skills

<div align="center">

| Advanced | Intermediate | Fundamental |
|:--------:|:------------:|:-----------:|
| Dynamic Programming x 394 | Math x 236 | Array x 743 |
| Segment Tree x 57 | Hash Table x 226 | String x 260 |
| Backtracking x 54 | Binary Search x 153 | Sorting x 152 |

</div>

---

## License

```
MIT License

Copyright (c) 2026 BLACK (BLACK0X80)
https://github.com/BLACK0X80
https://black0x80.vercel.app

All rights reserved under the terms of this license.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software, source code, solution files, documentation, and associated
materials (the "Software"), to deal in the Software without restriction,
including without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to permit
persons to whom the Software is furnished to do so, subject to the following
conditions:

  1. ATTRIBUTION REQUIRED
     The above copyright notice, this permission notice, and the name "BLACK"
     or "BLACK0X80" must be included in all copies, distributions, forks, or
     substantial portions of the Software. Removing or obscuring credit to the
     original author is strictly prohibited.

  2. NO MISREPRESENTATION
     You may not represent this work, or any derivative of it, as your own
     original work in any academic, professional, or competitive context.
     This includes but is not limited to submitting solutions from this
     repository as personal submissions on LeetCode or any similar platform.

  3. NO COMMERCIAL EXPLOITATION WITHOUT CONSENT
     You may not sell, license, or monetize this Software or any substantial
     portion of it without explicit written permission from BLACK (BLACK0X80).

  4. DERIVATIVE WORKS
     Any derivative work, fork, or adaptation of this Software must carry this
     same license, must visibly credit BLACK (BLACK0X80) as the original author,
     and must clearly indicate what changes were made.

  5. INTEGRITY OF AUTHORSHIP
     The intellectual effort, problem-solving approaches, and implementations
     contained within this repository are the exclusive creative output of
     BLACK (BLACK0X80). Any use that undermines or erases this authorship
     is a violation of the spirit and terms of this license.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE, AND NONINFRINGEMENT. IN NO EVENT SHALL
BLACK (BLACK0X80) BE LIABLE FOR ANY CLAIM, DAMAGES, OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF,
OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

<div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=gradient&customColorList=6,11,20&height=120&section=footer" width="100%"/>

**Engineered by BLACK - 2026**

#7,590 worldwide. 746 Hards. All in Rust.*

[![Profile](https://img.shields.io/badge/LeetCode_Profile-BBLACK0X80-000000?style=for-the-badge&logo=leetcode&logoColor=white)](https://leetcode.com/u/BBLACK0X80/)

</div>
