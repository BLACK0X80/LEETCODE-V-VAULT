<div align="center">

<img src="https://capsule-render.vercel.app/api?type=transparent&color=gradient&customColorList=6,11,20&height=120&section=header&text=LEETCODE-V-VAULT&fontSize=70&fontColor=ffffff&fontAlignY=50" width="100%"/>

<p align="center">
  <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&weight=700&size=22&duration=3000&pause=1000&color=FFFFFF&center=true&vCenter=true&multiline=true&width=800&height=100&lines=1%2C220+Problems+Solved+%7C+Rank+%2318%2C049+Worldwide;Rust+%7C+Python+%7C+C%2B%2B+%7C+Auto-Synced+Daily" alt="Typing Animation" />
</p>

<br/>

[![LeetCode](https://img.shields.io/badge/LeetCode-BBLACK0X80-FFFFFF?style=for-the-badge&logo=leetcode&logoColor=black)](https://leetcode.com/u/BBLACK0X80/)
[![Rank](https://img.shields.io/badge/Global_Rank-%2318%2C049-333333?style=for-the-badge&logo=trophy&logoColor=white)](https://leetcode.com/u/BBLACK0X80/)
[![Solved](https://img.shields.io/badge/Solved-1%2C220%2F3%2C906-111111?style=for-the-badge&logo=checkmarx&logoColor=white)](https://leetcode.com/u/BBLACK0X80/)
[![Acceptance](https://img.shields.io/badge/Acceptance-85.4JUdGzvrMFDWrUUwY3toJATSeNwjn54LkCnKBPRzDuhzi5vSepHfUckJNxRL2gjkNrSqtCoRUrEDAgRwsQvVCjZbRyFTLRNyDmT1a1boZV/)

[![Rust](https://img.shields.io/badge/Rust-1%2C167_Problems-000000?style=for-the-badge&logo=rust&logoColor=white)](./Rust)
[![Auto Sync](https://img.shields.io/badge/Sync-Daily_Auto-222222?style=for-the-badge&logo=githubactions&logoColor=white)](/.github/workflows)
[![License](https://img.shields.io/badge/License-MIT-444444?style=for-the-badge&logo=balance-scale&logoColor=white)](LICENSE)

<br/>

**Automated • Structured • Battle-Tested**

A self-syncing archive of every accepted LeetCode solution — automatically fetched, organized by language, and pushed daily via GitHub Actions. No manual uploads. No missing solutions. Absolute coverage.

[**Browse Solutions**](#language-breakdown) • [**How It Works**](#how-it-works) • [**Stats**](#competitive-stats)

</div>

---

## Competitive Stats

<div align="center">

| Metric | Value |
|:------:|:-----:|
| **Global Rank** | #18,049 |
| **Problems Solved** | 1,220 / 3,906 |
| **Acceptance Rate** | 85.43% |
| **Streak** | 20 Active Days · 17 Max Streak |
| **Easy** | 68 / 938 |
| **Medium** | 406 / 2,044 |
| **Hard** | 746 / 924 |

</div>

<br/>

<div align="center">

### Top Skills

| Advanced | Intermediate | Fundamental |
|:--------:|:------------:|:-----------:|
| Dynamic Programming × 394 | Math × 236 | Array × 743 |
| Segment Tree × 57 | Hash Table × 226 | String × 260 |
| Backtracking × 54 | Binary Search × 153 | Sorting × 152 |

</div>

---

## Language Breakdown

<div align="center">

| Language | Problems | Folder |
|:--------:|:--------:|:------:|
| **Rust** | 1,167 | [`/Rust`](./Rust) · [`/Rust_2`](./Rust_2) |
| **Python / Pandas** | 32+ | [`/Python`](./Python) · [`/PYTHONDATA`](./PYTHONDATA) |
| **C++** | 9+ | [`/C++`](./C++) |
| **JavaScript** | — | [`/JavaScript`](./JavaScript) |
| **TypeScript** | — | [`/TypeScript`](./TypeScript) |
| **Java** | — | [`/Java`](./Java) |
| **SQL** | — | [`/SQL`](./SQL) |

> When any language exceeds **1,000 solutions**, the vault automatically overflows into a sequenced folder (e.g. `Rust_2`, `Rust_3`).

</div>

---

## How It Works

The vault runs on a custom `sync.js` engine backed by GitHub Actions. Every day at midnight, or on manual trigger, the pipeline executes a full sync cycle:

```
LeetCode GraphQL API
        ?
        ?
  Fetch all accepted submissions
  (paginated, deduplicated by slug)
        ?
        ?
  For each new solution:
  ??? Fetch source code via submissionDetails
  ??? Fetch problem metadata via questionData
  ??? Write:
       ??? solution.<ext>   ? raw accepted code
       ??? README.md        ? problem + hints + solution
        ?
        ?
  Organize by language folder
  (overflow at 1,000 ? new folder)
        ?
        ?
  Regenerate root README.md
        ?
        ?
  git commit ? git push
```

Each problem folder contains exactly two files:

```
Rust/Two_Sum/
??? solution.rs     ? accepted submission code
??? README.md       ? difficulty · tags · problem · hints · solution
```

---

## Vault Structure

```
LEETCODE-V-VAULT/
??? Rust/                  ? Rust solutions (1–1000)
??? Rust_2/                ? Rust solutions (1001+)
??? Python/                ? Python 3 solutions
??? PYTHONDATA/            ? Pandas solutions
??? C++/                   ? C++ solutions
??? Java/                  ? Java solutions
??? JavaScript/            ? JavaScript solutions
??? TypeScript/            ? TypeScript solutions
??? SQL/                   ? MySQL solutions
??? sync.js                ? Sync engine
??? .github/workflows/     ? Auto-sync pipeline
??? README.md              ? This file (auto-regenerated)
```

---

## GitHub Actions Pipeline

The workflow runs on a daily cron schedule and supports manual dispatch:

```yaml
on:
  workflow_dispatch:
  schedule:
    - cron: '0 0 * * *'
```

The bot authenticates to LeetCode via `LEETCODE_SESSION` and `LEETCODE_CSRF_TOKEN` secrets stored in the repository, fetches every accepted submission, and only writes files that don't already exist — making each run incremental and fast.

---

## Solution Format

Every `README.md` inside a problem folder follows this structure:

```markdown
# Problem Title

**Difficulty:** Hard
**Tags:** Dynamic Programming, Segment Tree

---

## Problem

<full problem statement>

## Hints

1. ...

## Solution

?```rust
// accepted code
?```
```

---

## License

```text
MIT License — Copyright (c) 2026 BLACK
```

---

<div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=gradient&customColorList=6,11,20&height=120&section=footer" width="100%"/>

**Engineered by BLACK • 2026**

*1,220 down. The rest are next.*

[![Profile](https://img.shields.io/badge/LeetCode_Profile-BBLACK0X80-000000?style=for-the-badge&logo=leetcode&logoColor=white)](https://leetcode.com/u/BBLACK0X80/)

</div>
