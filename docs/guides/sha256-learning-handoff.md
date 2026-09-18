# SHA-256 learning handoff

Snapshot: 2026-09-18. Resume at **message schedule expansion**, after checking the current source.

## Start a new session

Copy this prompt into the new chat:

> Read AGENTS.md and docs/guides/sha256-learning-handoff.md, then inspect
> src/storage/util.rs. Continue my SHA-256 learning exercise from the message
> schedule. Teach one small step at a time with concrete examples. Do not write
> the full implementation for me. I implement each step, then you review and
> verify it before advancing. Padding, block-to-words conversion, and the two
> small-sigma helpers were checked in the previous session. Verify the current
> files rather than assuming this snapshot is still current. This is an
> educational exercise, not a password verifier for actual accounts.

Transfer both this document and your current source changes to the other
computer. At handoff, `src/storage/util.rs` and several application files have
uncommitted changes; cloning the repository alone may not bring them along.
This document describes the state; it is not a backup of those source files.

## Scope and teaching preferences

- The project is a custom Rust backend built from low-level pieces.
- The user originally wanted to build password hashing without a library.
  The conversation moved from a toy rolling hash to learning SHA-256 internals.
- Keep the distinction clear: SHA-256 alone is not suitable for password
  storage. Actual authentication should use a proven password-hashing
  implementation, consistent with [Phase 13](../phases/13-passwords-authentication.md).
- Explain bytes, offsets, types, and individual operations before abstractions.
- Give small snippets, formulas, function contracts, and checkpoints, not full
  implementation solutions. Let the user do the implementation.
- When asked to review, inspect the actual function and run focused checks.
  Do not edit production code as part of a review unless requested.
- Do not advance through several stages at once or repeat completed toy
  exercises. The next unfinished stage is the message schedule.
- An unrelated CMS UI report in the old conversation was explicitly withdrawn
  as a wrong-chat message. It is not part of this task.

## Current source and integration status

All learning helpers currently live in [src/storage/util.rs](../../src/storage/util.rs):

| Function | Current contract and status |
|---|---|
| `pad_256(input: &[u8]) -> Vec<u8>` | Implemented and checked for small inputs and padding boundaries. |
| `block_to_words(block: &[u8; 64]) -> [u32; 16]` | Implemented and checked. Private helper. |
| `small_sigma0(x: u32) -> u32` | Implemented and checked. Private helper. |
| `small_sigma1(x: u32) -> u32` | Implemented and checked. Private helper. |
| `hash_password(password: &str)` | Only calls `pad_256(password.as_bytes())` and discards the result. Returns `()`. It does not yet compute a digest or verify passwords. |

`create_session` and `HasId` also live in this file; they were not reviewed as
part of this exercise. Avoid unrelated changes to them or the unfinished
login/register work.

The user has corrected the padding comment to say the total is **a multiple
of 64 bytes** and added parentheses around shifts in the sigma helpers.

## What the user has learned

### Toy hash and collisions

The original rolling hash used `accumulator * 31 + byte`, with wrapping
arithmetic discussed for predictable overflow. It is no longer the current
`hash_password` implementation.

- `"ab"` gives `3105`.
- `"aa"` and `"bB"` both give `3104`: an easily constructed collision.
- Comparing those hashes accepts the wrong password.
- XOR plus `.count_ones()` measures output bit differences.
- `"ab"` versus `"ac"` changes one input bit and two output bits in that toy hash.
- Avalanche behavior is a statistical property; it does not alone prove security.
- Salts differ between registrations and are reused during verification.
  They do not prevent offline password guessing.

### Padding

The user needed concrete explanations of bytes versus bits and `% 64`.
Use the `"abc"` example if a refresher is needed:

```text
3 original bytes + 1 marker byte + 52 zero bytes + 8 length bytes = 64 bytes
```

- Marker: `0x80`.
- Zero padding: numeric zero, not `b'0'` (which is 48).
- Append zeros until the current vector length modulo 64 equals 56.
- The final eight bytes encode the original length in bits, as big-endian `u64`.
- `"abc"` has 24 original bits; its length field ends in `0x18`.
- For 10 input bytes, add 45 zero bytes, not 54.
- Original lengths 0, 3, and 55 pad to 64 bytes; 56 and 64 pad to 128 bytes.
- Current length expression is `(input.len() as u64) * 8`. Checked conversion,
  arithmetic overflow, and allocation limits remain necessary before treating
  this as a general-purpose input boundary.
- The zero-padding loop executes at most 63 times. Padding adds 9–72 bytes.
  Current implementation takes O(n) time and O(n) output memory.

### Reading a block

`block_to_words` uses `std::array::from_fn`, groups bytes with
`block[i * 4..i * 4 + 4]`, converts each group to a four-byte array, and uses
`u32::from_be_bytes`.

The conversion's `unwrap()` is justified by the fixed 64-byte input and exactly
four-byte slices for indices 0 through 15. No heap allocation is needed.

For padded `"abc"`:

```text
words[0]  = 0x61626380
words[1] through words[14] = 0
words[15] = 0x00000018
```

In Rust, that middle range is `words[1..15]`: the upper bound is excluded.

### Small-sigma helpers

Each operation starts from the same original `u32`, not the previous rotation's
result. Both helpers use constant work and no allocation.

| Input | `small_sigma0` | `small_sigma1` |
|---|---|---|
| `0` | `0x00000000` | `0x00000000` |
| `1` | `0x02004000` | `0x0000a000` |

See [RFC 6234 section 5.1](https://www.rfc-editor.org/rfc/rfc6234.html#section-5.1)
for the exact helper definitions.

## Verification already performed

The previous assistant extracted the actual functions into temporary Rust
harnesses under `/tmp`, compiled them with `rustc --edition=2024`, and ran:

- Padding: `"abc"` and lengths 0, 3, 55, 56, 63, 64, 119, 120, 128. Checked
  total size, unchanged prefix, marker, every padding zero, and length field.
- Block conversion: padded `"abc"`, all-zero and all-`0xFF` blocks, and all
  sixteen groups of a block containing sequential byte values.
- Sigma helpers: known zero/one outputs, mixed and edge values, and every
  single-bit input, compared with rotations expressed using shifts and OR.

All those checks passed after the user's fixes. These were isolated checks,
not a full project test run. The temporary harnesses were deleted after use;
no permanent regression tests were added. Full SHA-256 correctness has not
been established because the algorithm is still incomplete.

## Next checkpoint: message schedule

Explain the purpose first: each block supplies 16 original words, while the
compression stage needs 64 schedule words, one for each round. The remaining
48 words are derived from earlier ones. This is not the final digest.

Let the user choose the helper name and boundary. A natural contract is
16 input `u32` words to 64 output `u32` words.

Requirements from [RFC 6234 section 6.2](https://www.rfc-editor.org/rfc/rfc6234.html#section-6.2):

1. Preserve the original words at indices 0 through 15.
2. Fill indices 16 through 63 in ascending order.
3. Use this recurrence:

```text
W[t] = small_sigma1(W[t-2])
     + W[t-7]
     + small_sigma0(W[t-15])
     + W[t-16]
```

Every addition wraps modulo 2^32. In Rust, each addition needs
`wrapping_add`; wrapping only the final addition does not protect earlier
ordinary additions from overflow.

Start by explaining only `t = 16`: it reads indices 14, 9, 1, and 0.
Have the user calculate that word before implementing the entire expansion.

Checkpoint values for padded `"abc"` (locally calculated from the recurrence,
not quoted official test vectors):

| Index | Expected word |
|---|---|
| 16 | `0x61626380` |
| 17 | `0x000f0000` |
| 18 | `0x7da86405` |
| 19 | `0x600003c6` |
| 63 | `0x12b1edeb` |

Review preservation of the first 16 words, index boundaries, and overflow
behavior. Expansion performs exactly 48 iterations per block and uses a fixed
64-word output (256 bytes). Processing all message blocks should remain linear
in input size; do not retain every schedule unnecessarily.

## Later stages — do not implement yet

After the schedule checkpoint: learn compression helpers and initial state,
process the 64 rounds for one block, accumulate state across blocks, and encode
the final 32-byte digest. Then verify against published SHA-256 test vectors,
including empty input, `"abc"`, and multi-block messages. Use
[RFC 6234](https://www.rfc-editor.org/rfc/rfc6234.html) as the algorithm reference.

Keep this educational SHA-256 work separate from real password authentication.
