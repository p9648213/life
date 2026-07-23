# Phase 08: Redirects

Goal: use Post/Redirect/Get after successful form submissions.

Design the response API and route flow yourself.

## Expected Behavior

A successful resource `POST` returns `303 See Other` with a `Location` header pointing to a suitable `GET` route. Following the redirect shows the result, and refreshing that page does not resubmit the form.

## Requirements

- Serialize the `303 See Other` status correctly.
- Include one valid `Location` response header.
- Use an empty or small fallback body.
- Redirect only after the mutation succeeds.
- Keep validation failures as normal error responses.
- Do not allow untrusted header values to inject CR or LF.

## Tests to Write

- a redirect serializes status `303` and `Location`;
- successful creation redirects;
- invalid creation does not redirect;
- redirect headers reject CR/LF injection;
- following the redirect reaches a `GET` response.

## Checkpoint

You are done when successful form submission redirects to a readable result and refreshing that result does not repeat the mutation.

After this, continue with [Phase 09: File-Backed Storage](09-file-backed-storage.md).
