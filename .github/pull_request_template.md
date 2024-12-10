<!--
    A general pull request template to help ensure code quality and legibility
-->

# What does this implement/fix?

## Types of Changes

- [ ] Bug Fix
- [ ] New Feature
- [ ] Refactor
- [ ] Other

## Related Issue

Fixes...

## Test Environments

This feature/bug fix was tested on...

- [ ] Linux
- [ ] Windows
- [ ] Mac OSX
- [ ] Android
- [ ] iOS
- [ ] Browser Web Assembly
- [ ] Other Web Assembly (Please specify)
- [ ] Other

## General Submission Checklist

- [ ] Have you checked for other [pull requests][pr-tab] for the same or similar changes?
- [ ] Have you previously signed the Contributor(s) License Agreement?
- [ ] Does your pull request pass the dependency license checker, `cargo deny --all-features check licenses`?
- [ ] Have you ensured relevant files are handled by git lfs?
- [ ] Does your pull request pass cargo checks, `cargo check --workspace --all-targets --all-features --bins --tests --benches --examples`?
- [ ] Have you written relevant unit tests and they are [organized correctly][unit-test-organization]?
- [ ] Does your pull request pass unit tests, `cargo test --workspace --all-targets --all-features --bins --tests --benches --examples`?
- [ ] Does your pull request pass the clippy test, `cargo clippy --workspace --all-targets --all-features`?
- [ ] Have you ran the cargo formatter, `cargo fmt --all`
- [ ] Have you ran the git pre-commit hooks?

## Documentation Submission Checklist

- [ ] If a documentation update, have you tested against, `cargo doc --no-deps --workspace --all-features --document-private-items`?

## Banned/Restricted Features

- Generative AI (for example, Large Language Models) are entirely banned due to both copyright and moral reasons. This includes both code and assets.
- Regex usage is restricted and should be evaluated more closely. This is to reduce the potential misuse of regex by using it as a parser or sanitizer which can introduce security vulnerabilities. If possible, other methods of parsing and sanitization should be considered both to maintain code legibility, and to make it easier to write unit tests.

## Notes

- Reduce reliance on new dependencies
- Avoid linking in external libraries where possible (prefer static code)
- Try to focus on code legibility and future maintainability. This is subjective, but the goal is to reduce technical debt.

[pr-tab]: https://github.com/foxgirl-labs/catgirl-engine/pulls
[unit-test-organization]: https://doc.rust-lang.org/book/ch11-03-test-organization.html
