<div align="center">

# Rust Sensei 🦀

**Your Personal Rust Core Concepts Mentor**

Rust language skill-building via AI. Practice ownership, borrowing, traits, concurrency, async, and more — with intelligent hints, code review, and mock interviews.

[![Claude Code](https://img.shields.io/badge/Claude-Code-blueviolet)](https://claude.ai/code)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## What This Is

Rust Sensei is a Claude Code skill that helps developers master **Rust's core concepts** through structured practice. It is not a DSA/LeetCode tool — it focuses on Rust-specific fundamentals:

- Ownership, borrowing, and move semantics
- Lifetimes and reference validity
- Traits, generics, and trait objects (static vs dynamic dispatch)
- Enums, pattern matching, and algebraic data types
- Error handling (Result, Option, ?, thiserror, anyhow)
- Collections and iterators (lazy chains, custom iterators)
- Closures (Fn, FnMut, FnOnce) and functional patterns
- Concurrency (threads, Arc<Mutex>, RwLock, mpsc channels)
- Async/await with Tokio
- Smart pointers (Box, Rc, Arc, RefCell, Mutex)
- Testing, macros, modules, and performance

**84 challenges** across 15 topic groups, from foundational ownership to production-grade async patterns.

---

## Setup

```bash
# Clone the repo
git clone <your-repo-url>
cd rust-ai-practice

# Open Claude Code in this directory
claude .
```

That's it. The skill activates automatically through `SKILL.md` and `CLAUDE.md`.

---

## Daily Workflow

```
1. Open terminal in repo → run: claude .

2. Type:  lets solve
   → Claude finds your current ► problem, presents the challenge
      and a one-line "why this matters in production" hook.
      No hints yet — you figure out the approach first.

3. Think it through, write your Rust solution in solution.rs

4. Type:  here is my solution
   [paste your code]

5. Claude reviews for: unnecessary clones, borrow checker issues,
   idiomatic style, proper error handling

6. Answer 3-5 cross-questions about WHY your solution works

7. Problem gets marked [x], ► moves to the next one

8. Optionally type:  log today's session
   → Updates PROGRESS.md with streak and notes
```

---

## Commands Reference

| What you type | What happens |
|---------------|-------------|
| `lets solve` | Presents the current ► challenge |
| `lets solve #OB03` | Jump to a specific challenge by ID |
| `here is my solution` | Enter post-solve review + cross-questioning |
| `I solved it` | Same as above |
| `skip` | Marks current problem `[~]` (attempted), moves ► forward |
| `mark solved #OB03` | Marks OB03 as `[x]` without a full review |
| `update practice` | Shows current topic progress summary |
| `log today's session` | Updates PROGRESS.md with today's notes |
| `check solution` | Get hints on your current solution without giving the answer |

---

## Modes (Auto-detected)

Rust Sensei detects what you need and switches modes automatically:

### Tutor Mode
Triggered by: "explain ownership", "how do lifetimes work", "I don't understand RefCell"

Teaches Rust at depth — ownership model, borrow checker rules, trait resolution, drop order. Not syntax tutorials.

```
You: "explain why I can't have two mutable references"
→ Gets a peer-level explanation of the exclusivity guarantee,
  what the borrow checker enforces, and what data race it prevents
```

### Hint Mode
Triggered by: "give me a hint", "I'm stuck", "don't tell me the answer"

5-level progressive hints anchored to Rust's failure modes: move errors, borrow conflicts, lifetime issues, trait bounds, clone overuse.

```
You: "stuck on the lifetime annotation, give me a hint"
→ Hint #1: "What is the lifetime of the reference you're returning?"
  (guides you to the annotation without writing it for you)
```

### Review Mode
Triggered by: "check solution", "review my code", "is this idiomatic?"

Checks for:
- Unnecessary `.clone()` calls — could they be borrows?
- `.unwrap()` in library code — should be `Result`
- Iterator chains instead of manual loops
- Correct lifetime annotations or elision
- Proper error propagation with `?`

### Interview Mode
Triggered by: "mock interview", "practice machine round", "be the interviewer"

Simulates a Rust technical interview at a company using Rust in production. Evaluates ownership correctness, idiomatic style, communication, and production thinking. Picks from Easy / Medium / Hard challenges.

```
You: "mock interview, medium difficulty"
→ Interviewer roleplay starts, presents a Rust problem,
  evaluates approach before code, asks about ownership decisions,
  gives detailed feedback rubric at the end
```

### Pattern Mapper Mode
Triggered by: "what pattern should I use", "Arc<Mutex> vs channels here?", "what's the right Rust design for this?"

Maps your problem to the right Rust pattern from the catalog: Newtype, Builder, Type State, RAII, Interior Mutability, Extension Trait, Iterator Adapter, Error Chain, Async patterns.

```
You: "I need multiple owners of shared mutable state across threads"
→ "This is Arc<Mutex<T>> — here's why and how"
```

---

## Project Structure

```
rust-ai-practice/
├── SKILL.md                    # Rust Sensei skill (mode router)
├── CLAUDE.md                   # Review behaviour, language rules
├── PRACTICE.md                 # 84 Rust challenges with progress tracking
├── PROGRESS.md                 # Streak, session log, milestones
├── solution.rs                 # Your scratch file for solutions
├── modes/
│   ├── tutor-mode.md           # Ownership, lifetimes, traits deep dives
│   ├── hint-mode.md            # Progressive hint system
│   ├── review-mode.md          # Rust-specific code review checklist
│   ├── interview-mode.md       # Machine-round simulation
│   └── pattern-mapper-mode.md  # Rust pattern catalog
└── docs/
    └── rust-cheatsheet.md      # Quick reference: ownership, traits, concurrency
```

---

## The 54 Group Challenges at a Glance

| Group | Topics | Count |
|-------|--------|-------|
| Ownership & Borrowing | Move, Copy, mutable borrow conflicts | 4 |
| Lifetimes | Annotations, struct lifetimes, 'static | 3 |
| Traits & Generics | Trait definition, dyn vs generic, custom Iterator, blanket impl | 4 |
| Enums & Pattern Matching | Associated data, guards, destructuring, state machine | 4 |
| Error Handling | Custom error, Box<dyn>, thiserror, anyhow | 4 |
| Collections & Iterators | Custom Iterator, chaining, Entry API, fold/scan | 4 |
| Closures & Functional | Fn/FnMut/FnOnce, higher-order, lazy iterators | 3 |
| Concurrency | Arc<Mutex>, RwLock, mpsc, thread pool, Send+Sync | 5 |
| Async/Await | async fn, tokio::spawn, select!, async mpsc | 4 |
| Smart Pointers | Deref, Drop, RefCell, Rc vs Arc | 4 |
| Structs & Polymorphism | Builder, Newtype, dynamic dispatch | 3 |
| Testing | Integration tests, mocks, should_panic, benchmarks | 4 |
| Macros | macro_rules!, derive macros, vec! impl | 3 |
| Modules & Cargo | pub/private, feature flags | 2 |
| Memory & Performance | Stack vs heap, String vs &str, avoid clones | 3 |

---

## Manual Progress Edits

`PRACTICE.md` is a plain Markdown file — edit it directly any time:

- Mark solved: `[ ]` → `[x]`
- Mark attempted: `[ ]` → `[~]`
- Mark mastered: `[x]` → `[★]`
- Move `►` to whatever problem you want to work on next

---

## What Cross-Questioning Looks Like

After a correct solution, Claude asks 3-5 questions anchored to your actual code — not generic Rust trivia:

- "What does the borrow checker guarantee about the reference on line 8?"
- "If you removed the `.clone()` on line 12, what compile error would you get?"
- "Why does this function parameter need to be `&str` instead of `String`?"
- "Under what conditions would the `Mutex::lock()` on line 15 panic?"
- "How does Tokio's task scheduler relate to the async pattern you just wrote?"

You only mark the problem solved after you've answered satisfactorily. That's what builds the instinct.

---

## License

MIT — see [LICENSE](LICENSE).
