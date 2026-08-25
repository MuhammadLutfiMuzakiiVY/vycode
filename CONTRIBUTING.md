# Contributing Guidelines

Thank you for your interest in contributing! We welcome bug reports, feature suggestions, documentation improvements, and code contributions.

---

## 🛠️ Development Workflow

1. **Fork the Repository** and clone your fork locally:
   ```bash
   git clone https://github.com/<your-username>/<repo-name>.git
   cd <repo-name>
   ```
2. **Create a Feature Branch**:
   ```bash
   git checkout -b feat/your-feature-name
   # or
   git checkout -b fix/your-bugfix-name
   ```
3. **Set Up & Test Locally**:
   - For **Rust**: `cargo check && cargo test && cargo clippy`
   - For **Node / TypeScript**: `npm install && npm run lint && npm test`
   - For **Python**: `pytest && ruff check .`
4. **Commit Your Changes**: Follow [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat: add post-quantum signature verification`
   - `fix: resolve race condition in websocket reconnection`
   - `docs: update quickstart instructions in README`
   - `refactor: simplify token parsing logic`
5. **Push and Open a Pull Request**:
   ```bash
   git push origin feat/your-feature-name
   ```
   Fill out the Pull Request template thoroughly.

---

## 📜 Code Style & Quality Standards

- **Minimal & Clean**: No dead code, unnecessary dependencies, or unrequested boilerplate.
- **Type Safety**: Ensure strict typing across TypeScript, Rust, or Python type hints.
- **Formatting**:
  - Rust: `cargo fmt`
  - JS/TS: `prettier` / `eslint`
  - Python: `ruff format` or `black`
- **Tests**: Add unit/integration tests for any non-trivial logic changes.

---

## ⚖️ License
By contributing, you agree that your contributions will be licensed under the project's open-source license.
