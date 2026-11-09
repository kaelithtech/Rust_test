# Rust Hiring Test: Find a Free Window

```
rust-hiring-test/
├── README.md     # This file
├── solution.rs   # Your implementation goes here
├── test.rs       # Test suite (do not modify)
└── NOTES.md      # A few lines from you when you're done
```

## Time

**15-20 minutes. Please stop at 20.** One small problem. A correct, clean solution matters more than a fast one.

## Task

A hotel keeps a list of booked stays for one room. Implement `first_available` in `solution.rs`: replace the `todo!()` and add any helpers you need. Keep the types and signature.

```rust
first_available(&[(0, 3), (5, 8)], 2, 0)    // Ok(3): nights 3 and 4 are free
```

`first_available(stays, nights, from)` returns the earliest day `d >= from` such that the nights `d .. d + nights` don't overlap any stay.

- Each stay is `(check_in, check_out)` with `check_in < check_out`. The stay covers the nights `check_in .. check_out`; `check_out` is **exclusive**. So a stay ending on day 5 and another starting on day 5 don't clash.
- Stays can come in **any order** and may **overlap or nest**.
- `nights == 0` returns `InvalidNights`.
- Day numbers are `u32`. If the window would run past `u32::MAX`, return `NoRoomLeft`. Never wrap around and never panic.

## Run the tests

```
rustc --edition 2021 test.rs -o test && ./test
```

(`test.exe` on Windows.) No `cargo`, no crates, just `rustc` and the standard library.

## Submit (Git)

```
git clone <repo-url> && cd rust-hiring-test
git checkout -b <your-name>
# work in solution.rs, committing as you go
git add -A && git commit -m "first_available passing"
git push origin <your-name>
```

Before you finish, add 3-4 lines to `NOTES.md` and commit it. It includes one optional bonus question if you have time left.

## What we look at

Correctness on edge cases, error handling (no `unwrap()` or panics on input-dependent paths), idiomatic Rust, and clear thinking in your notes.
