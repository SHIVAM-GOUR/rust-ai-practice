# Progress Tracker

## Streak
**Current streak:** 2 days
**Last practiced:** 2026-05-08
**Longest streak:** 2 days

---

## Milestones
- [x] First problem solved
- [ ] Complete Fundamentals (30/30)
- [ ] Complete Ownership & Borrowing (4/4)
- [ ] Complete Lifetimes (3/3)
- [ ] 20 problems solved
- [ ] Complete Traits & Generics + Enums + Error Handling
- [ ] 40 problems solved
- [ ] Complete Collections + Closures + Concurrency
- [ ] 60 problems solved
- [ ] Complete Async + Smart Pointers + Structs + Testing
- [ ] Complete Macros + Modules + Memory
- [ ] All 84 problems solved 🎉

**Next milestone:** Complete Fundamentals (30/30)

---

## Weak Patterns

> Patterns where you keep getting tripped up. Update via "log today's session".

| Pattern | Attempted | Mastered | Notes |
|---------|-----------|----------|-------|
| — | — | — | No data yet |

---

## Session Log

| Date | Problems Attempted | Problems Solved | Notes |
|------|-------------------|-----------------|-------|
| 2026-05-05 | F01, F02, F03, F04 | 4 | Variables — immutability, shadowing; Ownership — return from function; Borrowing — &String |
| 2026-05-06 | F05, F06 | 2 | Borrowing — &mut reference, dereferencing with * to modify in place; Lifetimes — explicit 'a annotation on function returning a reference |
| 2026-05-07 | F07, F08, F09 | 3 | Structs — impl block with constructor using Self return type, shorthand field init, #[derive(Debug)]; Traits — manual Display impl with fmt::Formatter and write! macro; Enums — exhaustive match, &'static str return type |
| 2026-05-08 | F10, F11, F12, F13 | 4 | Option — Some/None variants, returning Option<i32> from a function, match vs if let for safe unwrapping; Result — Ok/Err variants, String error type, division by zero guard; ? operator — propagating errors up the call stack, map_err for type conversion; Custom error type — enum with variants, impl Display + Error, impl Error for empty body |

---

## Notes & Observations

> Recurring borrow checker errors, lifetime gotchas, aha moments.

- `?` expands to a match with early return + `.into()` for error type conversion via `From`
- `impl Error for T {}` can be empty — only supertrait bounds `Debug + Display` are required
- Storing `String` in error variants loses the original error and breaks `source()` chaining; prefer owning the original error type
- `'static` on error types means no borrowed references, not "lives forever" — errors should own their data
