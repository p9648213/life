# Phase 13: Passwords and Authentication

Goal: understand authentication flow without implementing unsafe crypto.

This is the main exception to the from-scratch rule. Do not write your own password hashing.

## What to Learn

- Registration
- Login
- Password hashing
- Salt
- Password verification
- Authentication versus authorization

## Where to Look

- OWASP password storage: https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
- RustCrypto password hashes: https://github.com/RustCrypto/password-hashes

## Step-by-Step Work

1. Add a user type.
2. Add a registration form.
3. Validate username and password.
4. Hash password with a proven crate such as `argon2`.
5. Store only the hash.
6. Add a login form.
7. Verify submitted password against stored hash.
8. On success, create a session.
9. Protect note creation behind login.

## Do Not Do This

- Do not store raw passwords.
- Do not use plain SHA-256 for passwords.
- Do not invent your own hashing function.
- Do not put passwords in logs.

## Questions to Answer

- Why is a password hash different from encryption?
- What is a salt?
- What is the difference between authentication and authorization?
- Which routes require login?

## Checkpoint

You are done when:

- Users can register.
- Users can log in.
- Notes can require authentication.
- Raw passwords are never stored.

