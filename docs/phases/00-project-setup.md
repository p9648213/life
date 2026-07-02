# Phase 00: Project Setup

Goal: create a Rust workflow and understand the backend core you are starting from.

You are not building the server yet. You are preparing the repo so each later phase is easier to test and reason about.

## What to Learn

- What `Cargo.toml` does
- What `src/main.rs` does
- How `cargo run`, `cargo check`, and `cargo test` differ
- How Rust modules are usually organized
- How to keep notes about design decisions
- How to separate backend-core decisions from application-domain decisions

## Where to Look

- Cargo book: https://doc.rust-lang.org/cargo/
- Rust Book, package layout: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
- Rust Book, modules: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

## Small Experiments

Run these commands and write down what each one does:

```bash
cargo run
cargo check
cargo test
```

Try intentionally breaking `src/main.rs`, then run `cargo check`. Restore it after you understand the error.

## Suggested Files

Do not create all future files immediately. Start small.

Useful documentation files:

```text
README.md
docs/
  README.md
  phases/
```

Possible code layout later:

```text
src/
  main.rs
  http/
    mod.rs
    request.rs
    response.rs
  connection.rs
  router.rs
  handlers.rs
  state.rs
  html.rs
  storage.rs
  diagnostics.rs
```

Only split files when `main.rs` becomes hard to read.

## Tasks

1. Run the default program.
2. Add a short `README.md` in your own words.
3. Write down the backend core boundary in your own words.
4. Create a small `docs/journal.md` if you want to record learning notes.
5. Write the first manual checklist:

```text
- Can I run the program?
- Can I explain what main() does?
- Do I know the next phase?
- Can I explain what belongs in backend core versus app code?
```

## Checkpoint

You are done when:

- `cargo run` works.
- You can explain `Cargo.toml`.
- You know which feature you are building first.
