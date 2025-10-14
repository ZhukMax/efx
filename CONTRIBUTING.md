# Contributing to EFx

[![PRs Welcome](https://img.shields.io/badge/PullRequest-welcome-brightgreen.svg?style=flat-square)]()

Thank you for your interest in contributing!  
Any help — from filing a bug report to implementing new tags — makes EFx better.

---

## 🧭 Principles

* **Safety & compatibility.** Avoid breaking public APIs unless discussed in advance.
* **Code style.** Follow `cargo fmt`; ensure `clippy` passes without warnings.
* **Tests.** Any logic change should include tests (`cargo test`, doctests, compile-fail tests).
* **Docs.** Public APIs, new tags, and attributes must be documented.
* **Performance.** Avoid regressions; add profiling notes if relevant.

---

## 🐞 Filing Issues

Before opening a new issue:

1. **Search existing issues** to avoid duplicates.
2. Prepare a **minimal reproducible example** — short `efx! { ... }` snippet, log, or screenshot.
3. For new features, provide a **short rationale** or reference an RFC.

**Issue template:**

* **Type:** bug / feature / docs / question  
* **Expected behavior:** …  
* **Actual behavior:** …  
* **Reproduction:** code or steps  
* **Versions:** `rustc --version`, EFx crate versions  
* **Platform:** OS/architecture  
* **Additional info:** backtrace, perf data, etc.

> 💡 *Tip:* issues labeled `good first issue` are designed for first-time contributors.

---

## 🧩 Workflow Overview

### 1. Pick or propose an issue

* **Claim an issue:** comment “I’d like to take this” (optionally with a short plan/ETA).  
  The maintainer will assign it to you.  
* **Propose a new issue:** open a ticket as described above.  
  If relevant, start from the roadmap RFC:  
  [`EFX-0001`](efx/docs/rfcs/EFX-0001-roadmap-0.5-0.7.md).

---

### 2. Fork & branch

Fork the repository and create a **feature branch from `main`**:

```bash
git checkout main
git pull
git checkout -b feat/short-topic
````

Keep your branch focused on **one issue**.

Branch naming convention:

| Type     | Example               | Purpose                   |
|----------|-----------------------|---------------------------|
| `feat/`  | `feat/button-rounded` | New feature or tag        |
| `fix/`   | `fix/text-alignment`  | Bugfix                    |
| `docs/`  | `docs/taglist-update` | Documentation             |
| `chore/` | `chore/ci-cache`      | Maintenance or CI updates |

---

### 3. Implement with tests & docs

Make your changes and ensure **all tests and examples build**.

Local checks:

```bash
# formatting
cargo fmt --all -- --check

# linting
cargo clippy --all-targets --all-features -D warnings

# tests
cargo test --workspace

# docs
cargo doc --workspace --no-deps
```

Examples (build-only; GUI run is not required):

```bash
cargo build -p efx --example eframe_demo --locked
rustup target add wasm32-unknown-unknown
cargo build -p efx --example eframe_demo --target wasm32-unknown-unknown --locked
```

---

### 4. Open a Pull Request

* Target the **next release branch** (e.g. `v0.6`, `v0.7`).
  If the issue has a **milestone**, use the branch named after it.
* Link the issue (e.g. `Closes #123`) and fill in the PR checklist.

**PR checklist:**

* [ ] References related issue / RFC section when behavior changes
* [ ] CI is green (workspace + examples + wasm32 build)
* [ ] Tests/docs updated where applicable
* [ ] No unrelated changes

**PR title examples (Conventional Commits):**

```
feat(button): add rounding attribute
fix(core): correct parsing of numeric attribute
docs: update intro example
```

Small review-fix commits are welcome.

---

### 5. Reviews & merge

* Review comments should be addressed promptly.
* Smaller, focused PRs get merged faster.
* If you go silent for **7 days**, the issue may be unassigned to keep momentum (you can re-claim it anytime).

---

## 🧪 Versioning

We follow **Semantic Versioning (SemVer)**.
Breaking changes require prior discussion in an issue or RFC.
All user-facing changes must be added to `CHANGELOG.md` under **Unreleased**.

---

## ⚖️ License

By submitting a contribution, you agree that your work is licensed under the same terms as EFx —
[MIT or Apache-2.0](efx/LICENSE).

---

## 💬 Contact

Questions, clarifications, or coordination requests:
📧 [mail@zhukmax.com](mailto:mail@zhukmax.com)
