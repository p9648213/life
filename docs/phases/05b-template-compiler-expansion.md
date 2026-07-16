# Phase 05B: Need-Driven Template Compiler Expansion

Goal: add exactly one template-compiler capability when a real template cannot be expressed safely or clearly with the Phase 05 foundation.

This phase is optional and repeatable. It is not a checklist to complete before Phase 06A. Skip it while the current compiler meets your needs. Return here from any later phase when a concrete template requirement exposes one missing compiler capability.

One visit to Phase 05B should implement one feature, verify it from template source through rendered output, document what remains unsupported, and then stop.

## First Decide Whether the Compiler Must Change

Do not add syntax merely because a feature might be useful someday.

First ask:

- Which real template is blocked?
- Can the handler calculate the value and pass it through the existing generated view?
- Is the requested behavior presentation structure, or is it business logic that belongs in the handler?
- Does the feature need new template structure, or only another ordinary runtime value?

If a handler can pass one already-prepared string through `{value}` or `{value:escape}`, the compiler may not need another feature.

Enter Phase 05B only when you can show the smallest template that the current compiler cannot express without unsafe output, duplicated markup, or unreasonable handler work.

## Starting Contract From Phase 05

The current compiler already supports:

```text
{name}                 raw HTML text
{name:escape}          HTML-text escaping
{name:escape:escape}   valid; still generates one escape call
```

It also has these behaviors:

- Literal template text becomes direct `out.push_str(...)` calls.
- Repeated variables share one generated `&str` view field.
- Escaping is selected independently for each variable occurrence.
- A template with variables generates a view struct and a render function.
- A static template generates a render function without a view struct or view argument.
- Recognized malformed placeholder forms return a `TemplateError`; some invalid generated-Rust cases, such as a Rust keyword used as a field name, remain normal rustc diagnostics.
- `build.rs` discovers templates and writes generated Rust to `OUT_DIR/templates.rs`.
- The application includes that generated file as ordinary compiled Rust.

Every Phase 05B extension must preserve the central pipeline:

```text
template files
  -> htmlc parses and validates template structure at build time
  -> htmlc generates ordinary Rust
  -> build.rs writes the Rust to OUT_DIR
  -> rustc compiles the generated renderers
  -> request-time code supplies typed values
  -> generated Rust appends directly into a caller-owned String
```

Request-time rendering must not read template files, scan template source, look up placeholder names in a string-keyed map, or replace source text.

Runtime conditionals and loops are compatible with a compiled template engine when the compiler generates ordinary Rust control flow. The condition or collection is request-time data, but the template structure was still parsed before request handling.

## Where to Look

- [Phase 05 foundation](05-html-rendering.md)
- [`htmlc` compiler](../../crates/htmlc/src/compiler.rs)
- [`TemplateError`](../../crates/htmlc/src/error.rs)
- [compiler regression tests](../../crates/htmlc/tests/htmlc.rs)
- [build-time template orchestration](../../build.rs)

## The One-Feature Brief

Before editing the compiler, write down this contract:

```text
Concrete application need:
Smallest blocked template:
Expected rendered HTML:
New template syntax:
Runtime input types:
Valid cases:
Invalid cases:
Escaping context:
Conceptual generated Rust:
Existing behavior that must remain compatible:
Interactions explicitly deferred:
```

If the feature cannot be described narrowly in this brief, it is not ready to implement.

The conceptual generated Rust is especially important. A template construct should have a clear translation into boring Rust before tokenizer or parser code changes begin.

## Candidate Features, Not a Backlog

Choose only the capability demanded by current work.

| Concrete need | Smallest useful first slice | Main design risk |
| --- | --- | --- |
| Literal braces or comments | One unambiguous escape or comment rule | Conflicting delimiter rules |
| Better compiler errors | Template path plus byte offset or line and column | Incorrect Unicode positions |
| Non-string values | One explicit type and formatting rule | Hidden allocations or vague formatting |
| Optional markup | One boolean conditional form | Nested blocks and missing terminators |
| Repeated markup | Iteration over one typed collection shape | Item scope, borrowing, and escaping |
| Shared fragment | One compile-time include form | Missing files, cycles, and path handling |
| Page shell | One layout/body composition rule | Variable scope and block collisions |
| Nested data | One typed field-access rule | Drifting into runtime string lookup |
| Dynamic attribute | One separate attribute encoder | Reusing unsafe HTML-text escaping |
| Faster rendering | Only one measured bottleneck | Speculative complexity |

This table is a routing guide. It does not mean all of these features should be implemented.

Some features have prerequisites. A real boolean conditional needs a generated boolean field rather than string truthiness. Iteration needs a typed collection and an item scope. Those data-shape decisions should be made before adding surface syntax that depends on them.

## Decide Whether the Representation Still Fits

The Phase 05 compiler has a flat stream shaped conceptually like:

```text
Literal
Variable
Literal
Variable
```

That remains suitable for non-nested additions such as a new variable output mode or better source diagnostics.

Do not force nested conditionals, loops, layouts, or blocks into a growing set of scanner flags and string searches. Once a feature introduces nesting or scopes, consider a small tree representation:

```text
Literal
Variable
If { condition, children }
For { item, collection, children }
```

This is not a requirement to build a general-purpose AST in advance. Introduce only the nodes required by the selected feature.

Likewise, the current `escape: bool` representation is enough for raw versus HTML-text output. If a real feature adds several output contexts or operations, replace the boolean with an explicit enum or operation representation rather than accumulating unrelated booleans.

## Keep Build-Time and Request-Time Work Separate

Build time may:

- Read template files.
- Resolve compile-time includes or layouts.
- Parse template syntax.
- Validate block structure and variable names.
- Detect include cycles or generated-name collisions.
- Generate Rust source.

Request time may:

- Construct typed view data.
- Evaluate generated Rust conditions.
- Iterate collections through generated Rust.
- Format typed values according to the compiled rule.
- Append literals and encoded values into the output buffer.

Request-specific values cannot be sent into `build.rs`. `build.rs` produces the renderer; the handler supplies values when it calls that renderer.

Includes and layouts must also remain compiled. Do not reopen template files during rendering. If an extension reads additional source files, ensure Cargo knows when those files should cause regeneration.

## Step-by-Step Extension Loop

1. Reproduce the limitation with the smallest real template.
2. Complete the one-feature brief.
3. Add regression tests for existing syntax that the change could disturb.
4. Add focused tests for the new valid and invalid cases before implementing broad interactions.
5. Decide whether the current flat tokens are sufficient or the selected feature requires one small structural node.
6. Extend parsing and validation only enough for the agreed syntax.
7. Add useful compiler errors for malformed forms of that syntax.
8. Generate direct, readable Rust that writes into the existing output buffer.
9. Inspect the generated Rust instead of reviewing only the compiler source.
10. Compile the generated Rust through the real Cargo/build-script path.
11. Invoke one generated renderer with runtime data and assert the exact output.
12. Run all existing Phase 05 regressions.
13. Document unsupported interactions and stop.
14. Return to the application or backend phase that exposed the missing feature.

Do not combine conditionals, loops, includes, layouts, helpers, and expression parsing into one Phase 05B visit.

## Testing Ladder

For the selected feature, cover the layers that can fail independently.

### 1. Existing contract regressions

Keep verifying:

- Literal HTML remains unchanged.
- `{name}` stays raw.
- `{name:escape}` stays escaped.
- Repeated `:escape` produces one escape call.
- Repeated variables generate one view field.
- Static templates still need no view struct or view argument.

### 2. Parser and validation tests

Cover:

- The smallest valid form.
- Empty or false behavior when relevant.
- Missing opening or closing syntax.
- Invalid names and unsupported operations.
- One deliberately deferred interaction that must return an error instead of compiling incorrectly.

### 3. Generated-source tests

Use the existing `normalize_generated_rust` style when comparing generated code. It removes irrelevant Rust formatting while preserving whitespace inside string literals.

Assert the important generated structure, not every incidental character. A structural feature should generate normal Rust control flow and continue writing to `out`.

### 4. Compilation and runtime integration

A generated-code string can look plausible and still be invalid Rust. Compile at least one real template through `build.rs`, call its generated renderer, and verify exact HTML output.

Useful validation commands are:

```bash
cargo test --manifest-path crates/htmlc/Cargo.toml
cargo check
cargo test
cargo fmt --manifest-path crates/htmlc/Cargo.toml --check
cargo fmt --check
git diff --check
```

## Generated API Boundary

Prefer to preserve the current handler-facing shape:

```text
handler builds typed view data
handler chooses or allocates one output String
generated render function writes into that String
handler turns the completed String into a response
```

Do not replace typed view fields with `HashMap<String, String>` just to make the compiler easier to extend. That would move name lookup and type mistakes back to request time.

If the selected feature genuinely requires booleans, collections, nested values, or formatting traits, evolve the generated view types deliberately. Add a compile/runtime test for the new public shape and regression tests for existing templates.

## Escaping and Trust Boundaries

The current `:escape` operation is for HTML text content. It is not automatically correct for:

- HTML attribute values
- URLs
- JavaScript
- CSS
- JSON embedded in HTML

If a real template needs another output context, define a separate operation and proven encoding rule for that context. Do not silently reuse HTML-text escaping.

Keep raw output explicit and limited to trusted HTML. Do not add arbitrary Rust expressions or unrestricted helper calls to templates; that grows the template language and its security boundary much faster than it appears.

Compile-time includes need restricted, deterministic path resolution. Reject traversal outside the template root, include cycles, and generated-name collisions rather than allowing surprising file access or recursion.

## Performance Implications

Preserve these properties unless measurement proves a redesign is necessary:

- Template structure is parsed before request handling.
- Generated renderers append into one caller-owned buffer.
- Literal runs become large direct appends where practical.
- Escaping writes into the final output buffer.
- Includes write into the same output buffer, whether their generated code is inlined or composed through another renderer.
- Loops do not allocate a new `String` for each item.

Do not add an optimizer, bytecode, parser generator, cache, or runtime interpreter merely because the compiler now has another feature. Add dependencies only when the selected problem justifies them.

## Common Problems

Problem: Phase 05B turns into a plan to build an entire template language.

Cause:

- The feature menu was treated as a backlog. Choose one concrete capability and stop after its checkpoint passes.

Problem: request-time rendering starts scanning template text again.

Cause:

- The compiler emitted a generic runtime interpreter instead of ordinary specialized Rust for the chosen template.

Problem: a conditional uses strings such as `"true"` and `"false"`.

Cause:

- Surface syntax was added before the generated context gained an honest boolean type.

Problem: nested blocks become difficult to close or validate.

Cause:

- A structural feature was forced into the original flat scanner. Introduce only the small tree structure needed for that nesting.

Problem: an include works in development but does not rebuild after editing the included file.

Cause:

- The new source dependency was not part of Cargo's rerun tracking.

Problem: generated code assertions pass but the project fails to compile.

Cause:

- Tests compared strings without compiling and invoking a real generated renderer.

Problem: template logic starts deciding permissions, database queries, or application rules.

Cause:

- Business logic crossed into the presentation compiler. Compute those decisions in the handler and pass typed presentation data.

## Questions to Answer for Each Visit

- What concrete template need triggered this extension?
- Could the handler solve it cleanly without new syntax?
- What is the smallest valid syntax and what malformed forms must fail?
- Which input types does the generated renderer need?
- What ordinary Rust should this construct generate?
- Which work happens at build time and which operation remains at request time?
- Does the feature still write directly into the caller-owned output buffer?
- Which escaping context applies?
- Does the feature require nesting, scopes, or a tree representation?
- How will a test prove that the generated Rust compiles and renders correctly?
- Which interactions are intentionally deferred?

## Per-Visit Checkpoint

One visit to Phase 05B is complete when:

- One real template need is satisfied.
- The syntax, runtime types, escaping rule, and invalid cases are documented.
- Positive and negative compiler tests exist.
- Existing Phase 05 syntax remains compatible or has an explicit migration.
- Generated Rust has been inspected and compiled.
- A focused runtime test verifies exact output.
- Rendering performs no runtime template parsing or file access.
- Output still goes directly into the caller-owned buffer.
- Unsupported interactions are documented.
- No unrelated compiler capabilities were added.

Phase 05B as a whole does not have a final "all features complete" state. It remains available for later, evidence-driven compiler work.

## Return to the Main Roadmap

If you opened Phase 05B while working on a later phase, return to that phase after the selected compiler feature passes its checkpoint.

If you are continuing directly from the Phase 05 foundation and do not currently need another compiler feature, proceed with [Phase 06A: Minimal Request Body Accumulation](06a-minimal-request-body-accumulation.md).

Do not remain in Phase 05B to implement unused features.
