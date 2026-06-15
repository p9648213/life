# Phase 07: In-Memory State

Goal: store notes while the process is running.

This phase introduces application state. Do it single-threaded first.

## What to Learn

- Structs for domain data
- Ownership of state
- Mutable access
- IDs
- Borrowing across handler calls

## Where to Look

- Rust structs: https://doc.rust-lang.org/book/ch05-00-structs.html
- Rust ownership: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
- Rust borrowing: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

## Suggested Domain Types

Think in terms of:

```text
Note:
  id
  title
  body

AppState:
  notes
  next_note_id
```

Do not worry about users yet.

## Step-by-Step Work

1. Define the note data you need.
2. Define a state object that owns all notes.
3. Create the state in `main`.
4. Pass mutable access to the router or handlers.
5. On `POST /notes`, add a note.
6. On `GET /notes`, render all notes.
7. On `GET /notes/:id`, render one note.

## Dynamic Path Hint

For `/notes/123`, you can:

1. Check whether the path starts with `/notes/`.
2. Take the remaining text after that prefix.
3. Parse it as a number.
4. Look up the note by ID.

## Questions to Answer

- Who owns `AppState`?
- Why does creating a note require mutable access?
- What happens to state when the process stops?
- Should IDs be reused after deletion?

## Checkpoint

You are done when:

- You can create a note.
- You can list notes.
- You can view one note by ID.
- Restarting the server loses notes, and you understand why.

