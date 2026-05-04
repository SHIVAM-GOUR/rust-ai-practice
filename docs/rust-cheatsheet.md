# Rust Concepts Cheatsheet

Quick reference for Rust ownership, types, traits, concurrency, and common patterns.

---

## Ownership Rules

| Rule | Detail |
|------|--------|
| Every value has one owner | Exactly one variable owns each value at any time |
| One owner at a time | You can't have two owners simultaneously |
| Value dropped when owner drops | RAII — cleanup is deterministic, no GC |
| Assignment moves non-Copy types | `let b = a;` makes `a` invalid for non-Copy types |
| Copy types are bitwise copied | `i32`, `bool`, `f64`, `char`, `&T` are Copy |

```rust
let s1 = String::from("hello");
let s2 = s1;           // s1 MOVED into s2
// println!("{}", s1); // ❌ compile error: use of moved value

let n1: i32 = 5;
let n2 = n1;           // n1 COPIED — i32 is Copy
println!("{}", n1);    // ✅ still valid
```

---

## Borrowing

| Type | Rule |
|------|------|
| `&T` (immutable) | Many immutable borrows at once — read-only |
| `&mut T` (mutable) | Exactly one mutable borrow — no other borrows |
| Borrows must be valid | Reference must not outlive the owned value |
| NLL (Non-Lexical Lifetimes) | Borrow ends at last use, not end of block |

```rust
let mut s = String::from("hello");

let r1 = &s;           // ✅ immutable borrow
let r2 = &s;           // ✅ another immutable borrow (ok!)
println!("{} {}", r1, r2);
// r1 and r2 are no longer used after this line (NLL)

let r3 = &mut s;       // ✅ mutable borrow (r1, r2 no longer active)
r3.push_str(" world");
```

---

## Lifetimes

```rust
// Explicit lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// "returned reference lives as long as the shorter of x and y"

// Struct holding a reference
struct Important<'a> {
    part: &'a str,
}

// 'static: lives for entire program
let s: &'static str = "I live forever";
```

**Lifetime elision rules** (compiler infers these automatically):
1. Each input reference gets its own lifetime parameter
2. If there's exactly one input lifetime, it's assigned to all outputs
3. If there's a `&self` or `&mut self`, its lifetime is assigned to all outputs

---

## Traits

```rust
// Define a trait
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str { "shape" } // default implementation
}

// Implement
struct Circle { radius: f64 }
impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

// Generic: static dispatch (monomorphized, no overhead)
fn print_area<T: Shape>(s: &T) {
    println!("Area: {}", s.area());
}

// Trait object: dynamic dispatch (vtable, heap allocation)
fn print_area_dyn(s: &dyn Shape) {
    println!("Area: {}", s.area());
}
```

| Approach | Dispatch | Overhead | Use When |
|----------|----------|----------|----------|
| `fn foo<T: Trait>(t: T)` | Static | None (monomorphized) | Performance-critical, known types |
| `fn foo(t: &dyn Trait)` | Dynamic | vtable lookup | Mixed types in collection |
| `fn foo(t: Box<dyn Trait>)` | Dynamic | heap + vtable | Owned mixed types, stored in struct |

---

## Common Standard Traits

| Trait | What it enables |
|-------|-----------------|
| `Debug` | `{:?}` formatting (derive with `#[derive(Debug)]`) |
| `Display` | `{}` formatting (implement manually) |
| `Clone` | `.clone()` deep copy |
| `Copy` | Implicit copy on assignment (no move) |
| `PartialEq` | `==` comparison |
| `Eq` | Total equality (requires PartialEq) |
| `Hash` | Use as HashMap key |
| `PartialOrd / Ord` | Comparison and sorting |
| `From<T> / Into<T>` | Type conversion |
| `Iterator` | `.next()` — enables all iterator methods |
| `Deref` | `*` dereference, deref coercion |
| `Drop` | Custom cleanup on scope exit |
| `Send` | Safe to transfer to another thread |
| `Sync` | Safe to share reference across threads |

---

## Error Handling

```rust
// ? operator — propagates error, converts via From
fn load(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

// Custom error type (thiserror)
#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Not found: {0}")]
    NotFound(String),
}

// Application error (anyhow)
use anyhow::{Context, Result};
fn run() -> Result<()> {
    let s = std::fs::read_to_string("config.toml")
        .context("failed to read config")?;
    Ok(())
}

// Option methods
let x: Option<i32> = Some(5);
x.unwrap_or(0)          // 5 or default
x.map(|v| v * 2)        // Some(10)
x.and_then(|v| if v > 0 { Some(v) } else { None })
x.unwrap_or_else(|| expensive_default())
```

**Rules:**
- `unwrap()` / `expect()` only in tests, prototypes, or when failure is truly impossible
- Use `?` for propagation in functions returning `Result` or `Option`
- Libraries: typed errors (`thiserror`). Applications: boxed errors (`anyhow`).

---

## Enums & Pattern Matching

```rust
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
}

match cmd {
    Command::Quit => println!("quit"),
    Command::Move { x, y } => println!("move to {x},{y}"),
    Command::Write(s) => println!("write: {s}"),
    Command::Color(r, g, b) => println!("color: {r},{g},{b}"),
}

// Match guards
match value {
    x if x < 0 => println!("negative"),
    0 => println!("zero"),
    x => println!("positive: {x}"),
}

// if let (single variant check)
if let Some(val) = option_value {
    println!("{val}");
}

// while let
while let Some(top) = stack.pop() {
    process(top);
}
```

---

## Collections

```rust
// Vec
let mut v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];
v.push(4);
v.pop();
v.len();
v.iter()         // &T iterator
v.iter_mut()     // &mut T iterator
v.into_iter()    // T iterator (consumes vec)

// HashMap
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", 42);
map.get("key");          // Option<&V>
map.contains_key("key");
map.entry("key").or_insert(0);           // insert if absent
map.entry("key").and_modify(|v| *v += 1); // modify if present

// String
let s = String::from("hello");
let s = "hello".to_string();
s.push_str(" world");
s.len();
s.is_empty();
s.contains("ell");
s.replace("hello", "hi");
s.split(' ').collect::<Vec<_>>();
```

---

## Iterators

```rust
let nums = vec![1, 2, 3, 4, 5];

// Lazy — nothing computed until consumed
nums.iter()
    .filter(|&&x| x % 2 == 0)   // [2, 4]
    .map(|&x| x * x)             // [4, 16]
    .collect::<Vec<_>>();         // forces evaluation

// Common terminal operations
.sum::<i32>()
.count()
.any(|&x| x > 3)
.all(|&x| x > 0)
.find(|&&x| x == 3)
.max() / .min()
.fold(0, |acc, &x| acc + x)

// zip, enumerate, flat_map
nums.iter().enumerate()               // (index, &value)
nums.iter().zip(other.iter())         // (&a, &b) pairs
nested.iter().flat_map(|v| v.iter())  // flatten one level

// chain
nums.iter().chain(more.iter())        // concatenate two iterators
```

---

## Closures

```rust
// Fn: borrows immutably — can be called many times
let factor = 2;
let double = |x| x * factor;   // borrows factor

// FnMut: borrows mutably — can be called many times, mutates captured state
let mut count = 0;
let mut increment = || { count += 1; count };

// FnOnce: takes ownership — can only be called once
let name = String::from("Alice");
let greet = move || println!("Hello, {name}!"); // moves name
greet(); // name is consumed
```

| Trait | Captures | Call times |
|-------|----------|------------|
| `Fn` | immutable borrow | Many |
| `FnMut` | mutable borrow | Many |
| `FnOnce` | takes ownership | Once |

---

## Concurrency

```rust
use std::thread;
use std::sync::{Arc, Mutex, RwLock};

// Basic thread
let handle = thread::spawn(|| {
    println!("in thread");
});
handle.join().unwrap();

// Move data into thread
let data = vec![1, 2, 3];
thread::spawn(move || println!("{:?}", data));

// Shared mutable state
let counter = Arc::new(Mutex::new(0));
let c = Arc::clone(&counter);
thread::spawn(move || {
    *c.lock().unwrap() += 1;
});

// RwLock: multiple readers or one writer
let cache = Arc::new(RwLock::new(HashMap::new()));
cache.read().unwrap().get("key");    // many readers ok
cache.write().unwrap().insert("k", "v"); // exclusive write

// mpsc channels
use std::sync::mpsc;
let (tx, rx) = mpsc::channel();
thread::spawn(move || tx.send(42).unwrap());
let val = rx.recv().unwrap();
```

**Send + Sync:**
- `T: Send` — T can be transferred to another thread
- `T: Sync` — `&T` can be shared across threads
- `Rc<T>` is neither Send nor Sync — use `Arc<T>` for threads
- `RefCell<T>` is Send but not Sync — use `Mutex<T>` for threads

---

## Async/Await (Tokio)

```rust
#[tokio::main]
async fn main() {
    let result = fetch().await;
}

async fn fetch() -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    "data".to_string()
}

// Spawn concurrent tasks
let h1 = tokio::spawn(task_one());
let h2 = tokio::spawn(task_two());
let (r1, r2) = tokio::join!(h1, h2); // wait for both

// Race futures
tokio::select! {
    result = primary() => handle(result),
    result = fallback() => handle(result),
    _ = tokio::time::sleep(Duration::from_secs(5)) => timeout(),
}

// Async channel
let (tx, mut rx) = tokio::sync::mpsc::channel(32);
tokio::spawn(async move { tx.send("hello").await.unwrap(); });
while let Some(msg) = rx.recv().await { process(msg); }
```

---

## Smart Pointers

| Type | Owner | Threads | Mutability | Use Case |
|------|-------|---------|------------|----------|
| `Box<T>` | Single | Any (T: Send) | Via `&mut` | Heap alloc, recursive types |
| `Rc<T>` | Multiple | Single only | Via `RefCell` | Shared ownership, no threads |
| `Arc<T>` | Multiple | Multi-thread | Via `Mutex`/`RwLock` | Shared ownership, threads |
| `Cell<T>` | Single | Single only | Interior | Copy types, no refs needed |
| `RefCell<T>` | Single | Single only | Interior (runtime) | Mutate through shared ref |
| `Mutex<T>` | Via Arc | Multi-thread | Interior (blocking) | Shared mutable, threads |
| `RwLock<T>` | Via Arc | Multi-thread | Interior (read/write) | Read-heavy shared state |

---

## Quick Rules

| Rule | Detail |
|------|--------|
| No GC | Memory freed when owner goes out of scope |
| Only one `&mut T` at a time | Mutable reference = exclusive access |
| `?` needs From conversion | Error type must impl `From<SourceError>` |
| String in params = `&str` | More flexible: accepts both String and literals |
| Clone is explicit | Nothing is silently cloned — if you see clone, it costs |
| unwrap in library = bad | Library callers can't handle panics — return Result |
| Send + Sync = thread safe | Arc<T> requires T: Send; Arc<T>: Sync requires T: Sync |
| Iterators are lazy | Chain methods; nothing runs until consumed |
| derive Debug everywhere | Always derive Debug — costs nothing, saves debugging |
