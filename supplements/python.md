---
supplement_slug: python
languages_or_interfaces: [Python]
domains_in_scope: [software-engineer, quality-engineer, platform-engineer, performance-engineer, security, solution-architect]
extensions: []
---

# Python Supplement

Per-domain extensions for Python projects. Loads alongside the domain prompts in scope when the project declares Python as a primary language.

## Software Engineer extensions

- **Type hints required.** PEP 484 type hints on every public function; private helpers may omit when local types are obvious. Static analysis via `mypy` / `pyright` in CI; type errors block merge.
- **Exception discipline.** Specific exception types over catch-all `Exception`. `try`/`except` blocks name the exception class; bare `except:` is the anti-pattern. Exceptions are part of the function's contract.
- **Mutable default arguments.** `def f(x=[]):` is a load-bearing footgun; default to `None` + sentinel-check. Pre-commit linter catches.
- **`pathlib` over `os.path`.** Path operations use `pathlib.Path`; cross-platform path handling via the library, not string concat.

## Quality Engineer extensions

- **`pytest` over `unittest`.** Standard runner; fixture composition; parametrize for case-based testing.
- **`hypothesis` for property-based testing.** Phase 5 surface A. Properties express DESIGN.md invariants; default 100-case budget for typical workloads; expand budget for narrow search spaces.
- **`mutmut` for Mutation Testing.** Phase 5 surface B. Per-mutant disposition required.
- **`atheris` for Fuzz Testing.** Phase 5 surface C. libFuzzer-based for Python.
- **`pytest-cov` for coverage measurement.** Coverage is necessary-not-sufficient; mutation kill rate is the stronger signal.

## Platform Engineer extensions

- **`pyproject.toml` discipline.** PEP 517/518 build system declaration. Single source-of-truth for project metadata, dependencies, dev dependencies, tool config.
- **`poetry.lock` / `pdm.lock` / `requirements.txt` committed.** Reproducible builds; pinned versions.
- **`.python-version` pinned.** Toolchain version managed via `pyenv` / `asdf` / equivalent.
- **`pip-audit` for supply-chain.** CVE detection at PR-time. Per the dependency-approval discipline.
- **Virtual environment hygiene.** Per-project venv; CI activates explicitly; no global pip installs.

## Performance Engineer extensions

- **`cProfile` / `py-spy` for profiling.** Hot-path identification before optimization. Profile-driven, not assumed.
- **Cython / Rust-via-PyO3 / `numpy` vectorization** for compute-bound hot paths when the workload justifies. Native-extension dependency is a Platform Engineer + Security review surface.
- **`async`/`await` discipline.** `asyncio` for I/O-bound concurrency; not for compute-bound (GIL constrains parallelism in single-process). `concurrent.futures.ProcessPoolExecutor` for true parallelism.

## Security extensions

- **`bandit` for security linting.** CI-side static analysis for common Python security anti-patterns (`subprocess` with shell-injection, `pickle` of untrusted data, `eval` / `exec` of untrusted strings).
- **`safety` / `pip-audit` for dependency CVEs.** Supply-chain audit at every PR.
- **Deserialization discipline.** `pickle` rejected for untrusted input; `json` / `msgpack` / explicit schema validation for cross-trust-boundary data.
- **Secret management.** Never `os.getenv('API_KEY')` and log result; redaction at the credential-handling boundary.

## Solution Architect extensions

- **Module + package boundaries.** `__init__.py` exposes the public API surface; private modules prefixed `_`. Re-exports explicit; deep imports across package boundaries are the maintainability gap.
- **Duck typing + protocols.** PEP 544 `Protocol` types make duck-typed contracts explicit. Per-protocol behavioral contracts are part of the architecture.
- **Async + sync boundary discipline.** Mixing async + sync code requires explicit `asyncio.run` / `loop.run_until_complete` at the boundary; async-in-sync without explicit boundary is the failure mode.
