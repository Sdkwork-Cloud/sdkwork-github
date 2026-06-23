# Repository Guidelines

<!-- SDKWORK-AGENTS-GENERATED: v1 -->

## SDKWORK Soul

Read `../sdkwork-specs/SOUL.md` before executing tasks in this root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this root:

- `../sdkwork-specs/README.md`
- `../sdkwork-specs/SOUL.md`
- `../sdkwork-specs/AGENTS_SPEC.md`
- `../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../sdkwork-specs/NAMING_SPEC.md`

Do not copy root standard text into this repository. If these relative paths do not resolve, stop and report the broken workspace layout.

## Application Identity

Application manifest: `apps/sdkwork-github-pc/sdkwork.app.config.json`. Root manifest: `sdkwork.app.config.json`.

## Local Dictionary Structure

- `AGENTS.md`: local agent entrypoint and relative SDKWORK spec index.
- `CLAUDE.md`, `GEMINI.md`, `CODEX.md`: compatibility shims pointing to `AGENTS.md`.
- `sdkwork.app.config.json`: workspace application manifest.
- `.sdkwork/`: source-controlled workspace metadata (`skills/`, `plugins/`).
- `specs/`: topology and component contracts.
- `sdks/`: SDK family workspaces and route manifests.
- `database/`: application database lifecycle assets.
- `apis/`: author-owned OpenAPI authorities.
- `crates/`: Rust route crates, API server, services, repositories.
- `apps/sdkwork-github-pc/`: PC React application surface.

## Spec Resolution Order

1. Read this `AGENTS.md` and any nearer component-level `AGENTS.md`.
2. Read `sdkwork.app.config.json` and `apps/sdkwork-github-pc/sdkwork.app.config.json`.
3. Read local `specs/` when present.
4. Read `../sdkwork-specs/README.md` and task-specific root specs.
5. Inspect implementation files only after dictionary entries are clear.

## Build, Test, and Verification

Run commands from this repository root.

- `pnpm dev`: start browser development.
- `pnpm build`: build the Rust workspace.
- `pnpm test`: run contract and architecture verification.
- `pnpm check`: Rust checks, PNPM script standard, architecture alignment, topology, database, API checks.
- `pnpm verify`: `cargo test --workspace` and `pnpm test`.
- `pnpm db:*`: database lifecycle via `sdkwork-database` CLI.

## Agent Execution Rules

Do not hand-edit generated SDK output. Do not replace generated SDK integration with raw HTTP. Record verification evidence before reporting completion.
