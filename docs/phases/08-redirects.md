# Phase 08: Redirects

Goal: use normal web behavior after form submissions.

After a successful `POST`, a web app usually redirects to a `GET` page. This is called Post/Redirect/Get.

## What to Learn

- `Location` header
- `303 See Other`
- Browser redirect behavior
- Avoiding duplicate form submission

## Where to Look

- MDN redirects: https://developer.mozilla.org/en-US/docs/Web/HTTP/Redirections
- MDN 303: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/303

## Step-by-Step Work

1. Create a response helper for redirects.
2. Set status to `303 See Other`.
3. Add `Location: /notes` or `Location: /notes/{id}`.
4. Use an empty body or a tiny fallback body.
5. After creating a note, return this redirect response.

## Experiments

```bash
curl -i -X POST http://127.0.0.1:8080/notes -d "title=A&body=B"
curl -L -i -X POST http://127.0.0.1:8080/notes -d "title=A&body=B"
```

Then try in the browser and refresh after submission.

## Questions to Answer

- Why use redirect after POST?
- What does `Location` contain?
- Why is `303` a good fit for form submission?

## Checkpoint

You are done when:

- Creating a note redirects.
- Refreshing the result page does not resubmit the form.

