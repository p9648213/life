# Phase 07: In-Memory State

Goal: define an application state boundary and store sample data while the process is running.

This phase introduces application state. Do it single-threaded first. The sample data is only a stand-in for your future app domain.

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
Record:
  id
  fields needed by your temporary exercise

AppState:
  records
  next_record_id
```

Do not worry about users yet. Do not bake the temporary sample type into the backend core.

## Step-by-Step Work

1. Define the temporary record data you need.
2. Define a state object that owns those records.
3. Create the state in `main`.
4. Pass mutable access to the router or handlers.
5. On a sample `POST` route, add a record.
6. On a sample `GET` route, render all records.
7. On a sample query-bearing route, render one record.

## Static Route and Query Identifier

Phase 04 deliberately uses exact static paths rather than dynamic path segments. For `/resources?id=123`, you can:

1. Match the exact `GET /resources` route.
2. Read `id` from the parsed query parameters.
3. Validate and parse it as a number.
4. Look up the record by ID.

Return a client error when `id` is missing, repeated when repetition is unsupported, incorrectly encoded, or not a valid identifier.

## Questions to Answer

- Who owns `AppState`?
- Why does creating a record require mutable access?
- What happens to state when the process stops?
- Should IDs be reused after deletion?
- What belongs in backend core, and what belongs in app state?

## Checkpoint

You are done when:

- You can create a sample record.
- You can list sample records.
- You can view one sample record by ID.
- Restarting the server loses records, and you understand why.
- The backend core does not depend on a specific product domain.
