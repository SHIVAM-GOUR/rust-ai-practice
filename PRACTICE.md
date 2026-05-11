# Rust Core Concepts Practice Tracker

**Overall: 12/84 solved**

> Legend: `[ ]` Unsolved · `[~]` Attempted · `[x]` Solved · `[★]` Mastered · `►` Current

---

## Fundamentals [5/30]

> Mixed topics, very easy to easy. Start here to build your Rust muscle memory before the deeper groups.

| ID | Challenge | Topic | Status |
|----|-----------|-------|--------|
| F01 | Declare a mutable variable, reassign it; demonstrate that immutability is default | Variables | [x] |
| F02 | Demonstrate shadowing: let x = 5; let x = x + 1; let x = x * 2 | Variables | [x] |
| F03 | Write a function that takes ownership of a String and returns it | Ownership | [x] |
| F04 | Borrow a String with & — read its length without taking ownership | Borrowing | [x] |
| F05 | Use a mutable reference &mut to modify a value inside a function | Borrowing | [x] |
| F06 | Return a reference from a function with explicit lifetime annotations | Lifetimes | [x] |
| F07 | Implement a struct with fields and an impl block with a constructor and method | Structs | [x] |
| F08 | Derive Debug and implement Display for a struct | Traits | [x] |
| F09 | Define an enum with variants, use match to handle all arms | Enums | [x] |
| F10 | Use Option<T> to represent a possibly absent value; unwrap safely with if let | Option | [x] |
| F11 | Write a function returning Result<T, E>; handle both Ok and Err variants | Error Handling | [x] |
| F12 | Use the ? operator to propagate errors up the call stack | Error Handling | [x] |
| F13 | Implement a custom error type that implements std::error::Error | Error Handling | [x] |
| F14 | Create a Vec<i32>, push elements, iterate with for and with .iter() | Collections | [x] |
| F15 ► | Use HashMap<String, i32> to count word frequencies in a sentence | Collections | [ ] |
| F16 | Chain Iterator methods: map → filter → collect into a Vec | Iterators | [ ] |
| F17 | Write a closure that captures a variable from the enclosing scope by reference | Closures | [ ] |
| F18 | Define a generic function with a trait bound (e.g., fn largest<T: PartialOrd>) | Generics | [ ] |
| F19 | Implement the Display trait (fmt::Display) for a custom struct | Traits | [ ] |
| F20 | Implement From<&str> for a custom struct, use .into() to convert | Traits | [ ] |
| F21 | Use Box<T> to allocate a value on the heap; demonstrate deref coercion | Smart Pointers | [ ] |
| F22 | Use Rc<T> to share ownership of a value between two variables | Smart Pointers | [ ] |
| F23 | Use Arc<Mutex<T>> to share a counter across two threads | Concurrency | [ ] |
| F24 | Spawn a thread with thread::spawn, pass data in, join it | Concurrency | [ ] |
| F25 | Use std::sync::mpsc to send values from a worker thread to main | Concurrency | [ ] |
| F26 | Write a generic struct Stack<T> with push and pop methods | Generics | [ ] |
| F27 | Use destructuring in let and match (tuples, structs, enums) | Pattern Matching | [ ] |
| F28 | Use if let and while let to unpack Option values cleanly | Pattern Matching | [ ] |
| F29 | Write a unit test with #[test], assert_eq!, and assert! | Testing | [ ] |
| F30 | Use Vec::with_capacity to pre-allocate and show it avoids reallocations | Performance | [ ] |

---

## Group 1: Ownership & Borrowing [0/4]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| OB01 | Move semantics: show that a moved value cannot be used; fix with clone or borrow | Easy | [ ] |
| OB02 | Copy vs Clone: demonstrate Copy types (i32, bool) vs non-Copy (String, Vec) | Easy | [ ] |
| OB03 | Multiple borrows: fix a compile error caused by mutable + immutable borrow overlap | Medium | [ ] |
| OB04 | Borrow checker puzzle: return a reference to data created inside a function — why does it fail? | Hard | [ ] |

---

## Group 2: Lifetimes [0/3]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| L01 | Annotate a function returning the longer of two string slices with explicit lifetimes | Easy | [ ] |
| L02 | Struct with a lifetime parameter: hold a &str field, annotate correctly | Medium | [ ] |
| L03 | 'static lifetime: understand string literals; create a 'static reference | Medium | [ ] |

---

## Group 3: Traits & Generics [0/4]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| TG01 | Define a Drawable trait; implement it on Circle and Rectangle structs | Easy | [ ] |
| TG02 | Trait objects: use Box<dyn Trait> for runtime polymorphism vs generics for static dispatch | Medium | [ ] |
| TG03 | Implement Iterator for a custom type by implementing next() | Medium | [ ] |
| TG04 | Blanket impl: implement a custom trait for all types that implement Display | Hard | [ ] |

---

## Group 4: Enums & Pattern Matching [0/4]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| EP01 | Enum with associated data: model a Message enum with variants holding different data | Easy | [ ] |
| EP02 | Match with guards: use if conditions inside match arms | Easy | [ ] |
| EP03 | Nested destructuring: destructure nested enums and tuples in match | Medium | [ ] |
| EP04 | State machine: model a traffic light or connection state with enum transitions | Medium | [ ] |

---

## Group 5: Error Handling [0/4]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| EH01 | Custom error enum: variants for different failures, impl Display + Error | Easy | [ ] |
| EH02 | Box<dyn Error>: return a boxed trait object for mixed error types | Medium | [ ] |
| EH03 | thiserror: rewrite a custom error type using #[derive(Error)] | Medium | [ ] |
| EH04 | anyhow: use anyhow::Result and context() to add error context in an app | Easy | [ ] |

---

## Group 6: Collections & Iterators [0/4]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| CI01 | Implement Iterator on a custom Fibonacci struct using next() | Medium | [ ] |
| CI02 | Iterator chaining: use zip, enumerate, flat_map in a real data transformation | Medium | [ ] |
| CI03 | Entry API: use entry().or_insert_with() to group or count efficiently | Easy | [ ] |
| CI04 | fold vs scan: compute running totals with scan, final aggregate with fold | Medium | [ ] |

---

## Group 7: Closures & Functional Patterns [0/3]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| CF01 | Fn vs FnMut vs FnOnce: write one closure for each, explain the difference | Medium | [ ] |
| CF02 | Higher-order function: write a function that takes a closure as parameter and returns one | Easy | [ ] |
| CF03 | Lazy iterators: show that Iterator chains are lazy; force with collect or sum | Medium | [ ] |

---

## Group 8: Concurrency [0/5]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| CON01 | Mutex<T>: protect a shared Vec across multiple threads using Arc<Mutex<T>> | Easy | [ ] |
| CON02 | RwLock<T>: multiple reader threads read simultaneously, writer gets exclusivity | Medium | [ ] |
| CON03 | mpsc channels: fan-out to N worker threads, collect results in main | Medium | [ ] |
| CON04 | Thread pool pattern: limit concurrency to N threads using a scoped approach | Medium | [ ] |
| CON05 | Explain why Rust prevents data races at compile time (Send + Sync traits) | Hard | [ ] |

---

## Group 9: Async/Await [0/4]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| ASY01 | async fn + .await: write a basic async function using Tokio | Easy | [ ] |
| ASY02 | tokio::spawn: run two async tasks concurrently, join both | Easy | [ ] |
| ASY03 | tokio::select!: race two futures, act on whichever completes first | Medium | [ ] |
| ASY04 | Async mpsc: tokio::sync::mpsc producer-consumer pipeline | Medium | [ ] |

---

## Group 10: Smart Pointers [0/4]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| SP01 | Implement Deref for a custom smart pointer wrapping a value | Medium | [ ] |
| SP02 | Implement Drop to run custom cleanup; observe drop order | Easy | [ ] |
| SP03 | RefCell<T>: interior mutability — mutate through a shared reference | Medium | [ ] |
| SP04 | Rc<RefCell<T>> vs Arc<Mutex<T>>: explain when to use each with an example | Medium | [ ] |

---

## Group 11: Structs & Polymorphism [0/3]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| ST01 | Builder pattern: use method chaining (fn name(mut self, ...) -> Self) | Medium | [ ] |
| ST02 | Newtype pattern: wrap a primitive for type safety (e.g., Meters(f64)) | Easy | [ ] |
| ST03 | Dynamic dispatch: store mixed types in a Vec<Box<dyn Trait>> and call methods | Medium | [ ] |

---

## Group 12: Testing [0/4]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| TST01 | Integration test: write tests in tests/ directory; import your crate | Easy | [ ] |
| TST02 | Test doubles: implement a mock struct satisfying a trait for testing | Medium | [ ] |
| TST03 | should_panic: test that a function panics with a specific message | Easy | [ ] |
| TST04 | Benchmark: write a Criterion benchmark and interpret throughput output | Medium | [ ] |

---

## Group 13: Macros [0/3]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| MAC01 | Write a declarative macro with macro_rules! that generates repetitive code | Medium | [ ] |
| MAC02 | Use derive macros: add #[derive(Debug, Clone, PartialEq, Hash)] and understand what they generate | Easy | [ ] |
| MAC03 | vec! macro internals: implement a simplified my_vec! macro | Medium | [ ] |

---

## Group 14: Modules & Cargo [0/2]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| MOD01 | Organize code into modules: pub mod, use, pub(crate), re-exports | Easy | [ ] |
| MOD02 | Cargo features: add an optional dependency behind a feature flag | Medium | [ ] |

---

## Group 15: Memory & Performance [0/3]

| ID | Challenge | Difficulty | Status |
|----|-----------|------------|--------|
| MEM01 | Stack vs heap: show Box<T> forces heap allocation; plain T stays on stack | Easy | [ ] |
| MEM02 | String vs &str: when to own vs borrow string data in function signatures | Easy | [ ] |
| MEM03 | Avoid clones: refactor a function that clones excessively to use references | Medium | [ ] |

---

> Edit this file to update status: `[ ]` → `[x]` solved · `[~]` attempted · `[★]` mastered
> Move `►` to the next unsolved problem when you advance.
