# Solution Template

Use this structure when providing complete solutions to ensure consistency and clarity.

---

## Problem: [Problem Name]

**ID:** [e.g., OB03] | **Group:** [e.g., Ownership & Borrowing] | **Difficulty:** [Easy/Medium/Hard]

### Problem Statement
[Concise restatement of the problem in plain English]

### Why This Matters in Production
[One sentence connecting this to real Rust usage in production code or popular crates]

---

## Approach

### Rust Concept at Play
**Concept:** [e.g., Borrow Checker, Lifetime Annotations, Trait Objects]

**Why this applies:**
- [Reason 1]
- [Reason 2]

### Key Insight
[The "aha!" moment — what does the borrow checker / type system guarantee here?]

### Strategy
1. [High-level step 1]
2. [High-level step 2]
3. [High-level step 3]

---

## Solution

### Approach 1: Naive / Direct

**Idea:** [Simplest approach, even if suboptimal]

```rust
fn naive_solution(/* params */) /* -> ReturnType */ {
    // straightforward but possibly cloning more than needed
    todo!()
}
```

**Why it works:**
- [Explanation]

**Why it's not optimal:**
- [e.g., unnecessary clone, heap allocation, not idiomatic]

---

### Approach 2: Idiomatic Rust

**Idea:** [Improved approach using Rust idioms]

**Walkthrough with Example:**
```
Input: [specific example]

Step 1: [what happens — ownership perspective]
State: [which variables own/borrow what]

Step 2: [what happens]
State: [show borrow state]

Output: [result]
```

**Code:**

```rust
fn idiomatic_solution(/* params */) /* -> ReturnType */ {
    // Uses borrows, iterator chains, ? operator, pattern matching
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        // assert_eq!(idiomatic_solution(input), expected);
    }

    #[test]
    fn test_edge_case() {
        // empty, None, Err, boundary values
    }
}
```

**Ownership/Borrowing Analysis:**
- **Inputs:** [owned vs borrowed, why]
- **Inside function:** [who owns what, borrow lifetimes]
- **Return:** [owned value, borrowed reference, or Result]

**Why this is idiomatic:**
- [e.g., borrows instead of clones, uses iterator chain, proper error handling]

---

## Complexity Analysis

### Time Complexity: O(?)
**Breakdown:**
- [Operation 1]: O(?)
- **Total:** O(?)

### Space Complexity: O(?)
**Breakdown:**
- [Stack allocation]: O(?)
- [Heap allocation (if any)]: O(?)

---

## Edge Cases & Testing

### Edge Cases to Consider
1. **Empty input:** `vec![]` or `""` → Expected: [result]
2. **Single element:** → Expected: [result]
3. **Error path:** → Should return `Err(...)` / `None`
4. **[Rust-specific]:** e.g., empty string vs None, moved vs borrowed value

---

## Common Mistakes in Rust

❌ **Mistake 1:** [e.g., using String instead of &str in parameter]
- **Why it's wrong:** Forces unnecessary allocation / cloning
- **Correct approach:** Use `&str` — accepts both `String` and string literals

❌ **Mistake 2:** [e.g., .unwrap() on Result/Option]
- **Why it's wrong:** Panics on Err/None instead of propagating
- **Correct approach:** Use `?` operator or proper match/if let

❌ **Mistake 3:** [e.g., unnecessary .clone()]
- **Why it's wrong:** Costly allocation; borrow would have worked
- **Correct approach:** Pass `&T` reference instead

---

## Cross-Questioning Prep

If asked to defend this solution:

**"Why did you borrow instead of taking ownership?"**
- Response: [borrowing avoids allocation, the caller might still need the value]

**"What would the borrow checker say if you did X instead?"**
- Response: [specific error: "cannot borrow as mutable because it is also borrowed as immutable"]

**"How would this change if it needed to be thread-safe?"**
- Response: [wrap in Arc<Mutex<T>>, add Send bound to generic, etc.]

---

## Key Takeaways

1. **Ownership:** [what this problem taught about ownership]
2. **Pattern:** This is a [concept] problem — look for [signals]
3. **Idiom:** Use [Rust construct] for [scenario]
4. **Pitfall:** Watch out for [common Rust mistake in this context]
5. **Next time:** When you see [characteristic], think [approach]

---

**Remember:** The borrow checker is enforcing correctness — when it rejects your code, it's telling you something about your design!
