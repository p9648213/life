# Phase 07: In-Memory State

Goal: define an application state boundary and store sample data while the process is running.

This phase introduces application state. Do it single-threaded first. The sample data is only a stand-in for your future app domain.

## Mental Model

Application state is data that survives across requests but only for the lifetime of the running server process.

It is similar in purpose to Axum's `State`: handlers can use it for data shared by the application. Later, such state might contain a database connection pool, configuration, a cache, or service clients. In this phase, however, use a small in-memory collection so that ownership and borrowing remain visible.

Keep the initial flow explicit:

```text
main creates one AppState
  -> server accepts a connection
  -> server parses one Request
  -> router selects a handler
  -> handler receives access to the same AppState
  -> handler reads or mutates it
  -> handler returns a Response
```

The state must be created once, outside the connection-accept loop. Creating it inside `handle_client` or inside a handler would give every request fresh state, so records would appear to disappear immediately.

This phase is deliberately simpler than typical Axum state:

- The server is still single-threaded, so direct borrowing is enough.
- Do not introduce `Arc`, `Mutex`, or other synchronization yet.
- Do not add a database yet; persistence belongs to later phases.
- Restarting the process intentionally loses all records.

## What to Learn

- Structs for domain data
- Ownership of state
- Mutable access
- IDs
- Borrowing across handler calls
- Separating application data from reusable backend infrastructure
- Bounding memory that accumulates across requests

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

`Record` and `AppState` are application-domain types. The TCP server, HTTP parser, response builder, and router are backend-core types.

Do not worry about users yet. Do not make the backend core import or name the temporary `Record` or `AppState` type. Instead, design the state-passing boundary so the core can carry an application-chosen state type to a handler without understanding its fields. A generic state parameter is one direct option; avoid introducing a framework-like abstraction that hides the call flow.

## Ownership and Borrowing Target

Choose one clear owner for the state:

- `main` can retain ownership and pass `&mut AppState` into the server run loop, or
- `main` can create the state and move it into `Server`, after which `Server` owns it.

Either design is acceptable if there is exactly one long-lived state value and you can explain its lifetime. Do not clone the whole state for each request.

The current handler boundary accepts only a request, conceptually:

```text
handler(request) -> response
```

For this phase, evolve the router/handler boundary toward:

```text
handler(request, mutable_state) -> response
```

A creating handler needs mutable access because it changes both `records` and `next_record_id`. A listing or detail handler only reads state. It is acceptable to give every handler mutable access initially to keep one handler signature, but recognize that this grants more capability than read-only handlers require.

Follow one mutable borrow through the call chain rather than reaching for global variables:

```text
server run loop
  -> connection handling
  -> route dispatch
  -> selected handler
```

The mutable borrow should end when that request has been handled. Never use `static mut` or unsafe code to bypass Rust's ownership rules.

## Backend Core and Application Boundary

Use this split when deciding where code belongs:

- Backend core owns TCP reading, HTTP parsing, route lookup, and response serialization.
- Application code defines `Record`, `AppState`, field validation, ID allocation policy, and resource handlers.
- The router knows how to pass state to a handler, but does not know what the state contains.
- A handler reads request data, applies application rules, accesses state, and constructs a response.

The temporary resource name and fields are exercises, not permanent backend concepts. You should be able to replace `AppState` with a different domain state without rewriting the HTTP parser or response builder.

## Step-by-Step Work

1. Define the temporary record data you need.
2. Define a state object that owns those records.
3. Decide the maximum number of records the in-memory exercise will retain.
4. Create exactly one state value in `main`, outside the accept loop.
5. Decide whether `main` keeps ownership or moves the state into `Server`.
6. Adjust the handler and dispatch boundary so the selected handler receives state without teaching the backend core about `Record`.
7. Pass the same state through the server, router, and handler call chain.
8. On a sample `POST /resources` route, validate the decoded fields and add one record.
9. Allocate an ID using `next_record_id`, and handle counter overflow explicitly rather than wrapping or panicking.
10. On `GET /resources` without an `id`, render all records.
11. On `GET /resources?id=123`, validate the identifier and render one matching record.
12. Return an explicit client error for an invalid identifier and `404 Not Found` for a valid identifier that does not exist.
13. Restart the process and confirm that the records are gone.

Do not implement deletion unless you want it as an extra exercise. You should still decide now that allocated IDs are not silently reused; this keeps identity stable if deletion is added later.

## Static Route and Query Identifier

Phase 04 deliberately uses exact static paths rather than dynamic path segments. For `/resources?id=123`, you can:

1. Match the exact `GET /resources` route.
2. Read `id` from the parsed query parameters.
3. Validate and parse it as a number.
4. Look up the record by ID.

Return a client error when `id` is empty, repeated when repetition is unsupported, incorrectly encoded, or not a valid identifier. Whether a missing `id` is valid depends on the route contract below.

For one exact route to support both list and detail behavior, define the contract explicitly:

```text
GET /resources          -> list all records
GET /resources?id=123   -> show record 123
```

Under that contract, a missing `id` means “list” rather than an error. An empty, repeated, malformed, or non-numeric `id` is a client error. If you instead choose a separate exact detail route, document that route and then treat a missing `id` there as an error. Keep this decision in the handler; route lookup still matches only the method and exact path.

## Lifetime and Persistence

The state lives in RAM:

- A successful request can change what later requests observe.
- Closing one client connection does not remove the state.
- Restarting or crashing the process removes the state.
- Running two server processes creates two independent state values.

Phase 09 will introduce file-backed persistence. Phase 18 will introduce a database layer and connection lifecycle. A future `AppState` may hold a database connection or pool, but Phase 07 should not skip directly to that design.

## Limits and Algorithmic Cost

Request-size limits do not bound application state. A client can send many individually valid requests, so an unlimited `Vec<Record>` can consume memory for the entire server lifetime.

For this phase:

- set a small maximum record count and reject creation when it is reached;
- keep the existing total-request limit in force and choose application-level limits for fields retained in each record;
- use checked arithmetic for the next ID;
- avoid cloning the complete records collection for every request;
- render with work proportional to the number and total size of records being returned.

A linear scan of a bounded `Vec<Record>` for one ID is acceptable here. Record its `O(n)` lookup cost and the configured maximum. Do not add an index or change collections until a real requirement or measurement justifies it. Re-evaluate the representation if the record limit later grows substantially.

## Tests to Write

Keep most state tests below the TCP layer. Construct a router, one state value, and parsed requests directly where practical.

Useful cases:

- initial list is empty;
- one valid POST creates one record;
- two POST requests use the same state and receive distinct increasing IDs;
- listing after creation contains the created records;
- a valid existing `id` returns the matching record;
- a valid but unknown `id` returns `404`;
- empty, malformed, non-numeric, and unsupported repeated IDs return the chosen client error;
- invalid form data does not mutate records or advance the ID;
- reaching the record limit rejects creation without partially changing state;
- ID overflow is handled without wrapping or partially changing state.

For mutation failures, assert the state before and after the request. The important invariant is that either the complete record is inserted and the counter advances once, or neither change happens.

## Common Problems

Problem: every request sees an empty list.

Cause:

- `AppState` was created inside the connection or handler function instead of once at startup.

Problem: the router imports the temporary `Record` type.

Cause:

- Application behavior leaked into the backend core. The router should transport state to a handler without inspecting it.

Problem: Rust reports more than one mutable borrow.

Cause:

- A borrow of the state or one of its fields remains alive while another mutation begins. Keep borrows narrow and compute validated inputs before mutating state.

Problem: adding one record requires cloning all existing records.

Cause:

- Ownership is being transferred where a temporary borrow would be enough.

Problem: the single-threaded phase immediately uses `Arc<Mutex<AppState>>`.

Cause:

- Concurrency machinery was introduced before it solves a current problem. Direct mutable borrowing is the learning target; synchronization comes in Phase 15.

## Questions to Answer

- Who owns `AppState`?
- Where is the single state value created, and why is it outside the accept loop?
- Why does creating a record require mutable access?
- Why should listing records need only shared/read access conceptually?
- What happens to state when the process stops?
- Should IDs be reused after deletion?
- What happens when the ID counter reaches its maximum?
- How is total in-memory growth bounded across many requests?
- What belongs in backend core, and what belongs in app state?
- How could the application state later hold a database pool without making the HTTP router database-specific?

## Checkpoint

You are done when:

- You can create a sample record.
- You can list sample records.
- You can view one sample record by ID.
- Multiple requests observe the same state value.
- Invalid input and failed creation leave state unchanged.
- Record count and ID growth have explicit limits and failure behavior.
- Restarting the server loses records, and you understand why.
- The backend core does not depend on a specific product domain.
- State-focused router tests run without opening a TCP socket.

## Continue

After state can be created, read, and mutated through handlers, continue with [Phase 08: Redirects](08-redirects.md).
