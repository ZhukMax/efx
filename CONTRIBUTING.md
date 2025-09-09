# Contributing to EFx

Thank you for your interest in contributing!
Any help — from filing a bug report to implementing new tags — makes EFx better.

---

## Principles

* **Safety & compatibility.** Avoid breaking public APIs unless agreed upon in advance.
* **Code style.** Follow `cargo fmt` and ensure `clippy` passes without warnings.
* **Tests.** Any logic change should come with tests (`cargo test`, doctests, compile-fail tests).
* **Docs.** Public APIs, new tags, and attributes must be documented.
* **Performance.** Avoid regressions; add profiling notes if relevant.

---

## Filing Issues

Before opening a new issue:

1. **Search existing issues** to avoid duplicates.
2. Prepare a **minimal reproducible example** (short `efx! { ... }` snippet, log, or screenshot).

**Template for issues:**

* **Type:** bug / feature / docs / question
* **Expected behavior:** …
* **Actual behavior:** …
* **Reproduction:** code or steps
* **Versions:** `rustc --version`, EFx crate versions
* **Platform:** OS/architecture
* **Additional info:** backtrace, perf data, etc.

---

## Submitting Pull Requests

1. **Fork** and create a branch:

    * `feat/<feature>` — new functionality
    * `fix/<bug>` — bugfix
    * `docs/<topic>` — docs only
    * `chore/<topic>` — maintenance
2. Make your changes and ensure **all tests pass**.
3. Update **docs** and **CHANGELOG.md** if behavior is user-visible.
4. Run local checks (see below).
5. Open a PR with:

    * **Title:** short and clear
    * **Description:** what/why, how tested, API impact
    * Link to relevant **issue or RFC**

Small follow-up commits for review fixes are welcome.

---

## Pre-PR Checklist

```bash
# formatting
cargo fmt --all -- --check

# linting
cargo clippy --all-targets --all-features -D warnings

# tests (unit, doctests, compile-fail)
cargo test --workspace

# docs build
cargo doc --workspace --no-deps
```

You can also check examples manually:

```bash
cargo run -p efx-sandbox
cargo run -p efx --example eframe_demo
```

---

## Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(button): add rounding attribute
fix(core): correct parsing of numeric attribute
docs: update intro example
```

---

## Versioning

We follow **SemVer**.
Breaking changes require discussion in an issue/RFC.
All user-facing changes should be added to `CHANGELOG.md` under **Unreleased**.

---

## License

By submitting a contribution, you agree that your work is licensed under the same terms as EFx
([MIT OR Apache-2.0](efx/LICENSE)).