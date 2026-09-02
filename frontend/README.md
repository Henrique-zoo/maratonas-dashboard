# Frontend

The frontend uses Bun as its package manager, JavaScript runtime, script runner,
and unit-test runner. Use the Bun version declared in `package.json`; the
committed `bun.lock` is the sole dependency lockfile.

## Setup and checks

```sh
bun ci
bun run dev
```

The main verification commands are:

```sh
bun run quality
bun run format:check
bun run build
bun run test:unit
bun run test:coverage
bun run test:contract
bun run test:e2e
bun run test:e2e:fullstack
```

`bun run check` combines static quality checks, the production build, and unit
tests with coverage. Contract tests require a reachable backend API. End-to-end
tests require Chromium and its system dependencies, which can be installed with
`bun run test:e2e:install`. The E2E command builds the production bundle, serves
it with Vite Preview, and runs only `*.spec.js` files. `bun run test:e2e:list`
lists the selected mocked scenarios without opening a browser.

`bun run test:e2e:fullstack` runs the separate Chromium smoke test in
`tests/full-stack`. It does not mock HTTP requests: Vite Preview proxies `/api`
to `API_PROXY_TARGET` (by default `http://127.0.0.1:8000`), so the command
requires the real API and its PostgreSQL database. In CI, Ubuntu, Rust, Bun,
Playwright, and PostgreSQL are pinned, the deterministic BDD fixture is loaded
after the embedded migrations, and the contract suite runs before the browser
exercises the frontend, API, SQL queries, and database together.

## Coverage

`bun run test:coverage` uses Bun's native coverage instrumentation. It prints
function and line percentages and writes an LCOV report to `coverage/lcov.info`.
Test files are excluded from the calculation. The 75% function and line
thresholds in `bunfig.toml` apply to every reported source file and prevent
regressions in the modules exercised by the unit suite.

Bun reports coverage only for modules loaded by the test process. Therefore,
the result describes the exercised JavaScript modules rather than every source
file in the frontend. Browser behavior is additionally covered by Playwright,
but those scenarios do not contribute to Bun's unit-coverage percentage.

## Static quality

`bun run quality` uses ESLint and Stylelint for JavaScript and CSS analysis.
Prettier remains an independent formatting check through
`bun run format:check`. In addition to correctness rules, ESLint reports JavaScript
functions that exceed these maintainability limits:

- cyclomatic complexity: 15;
- nesting depth: 4;
- parameters: 4;
- statements per function: 40.

These limits are diagnostic warnings: they expose maintainability hotspots
without making existing technical debt fail the build. They should guide code
review and focused refactoring, but no isolated complexity metric constitutes
proof of software quality. Test effectiveness, correctness, readability, and
the suitability of the design must be considered together.
