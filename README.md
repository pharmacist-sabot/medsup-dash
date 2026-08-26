# MedSup Dash

[![CI](https://img.shields.io/github/actions/workflow/status/suradet-ps/medsup-dash/ci.yml?branch=main&label=CI)](https://github.com/suradet-ps/medsup-dash/actions/workflows/ci.yml)
[![CodeQL](https://img.shields.io/github/actions/workflow/status/suradet-ps/medsup-dash/codeql.yml?branch=main&label=CodeQL)](https://github.com/suradet-ps/medsup-dash/actions/workflows/codeql.yml)
[![Release](https://img.shields.io/github/v/release/suradet-ps/medsup-dash)](https://github.com/suradet-ps/medsup-dash/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Medical Support Dashboard - Sabot Hospital. Tracks the budget value of
supported medication (`med_transactions`) per Thai fiscal year (ต.ค. - ก.ย.),
with KPI cards, a quarterly report, and a recent-transactions table.

Rewritten from Vue 3 + `@supabase/supabase-js` to **Rust / Leptos 0.8 (CSR)**
compiled to WebAssembly. The Supabase project itself is unchanged: the app
talks to the same PostgREST (`/rest/v1`) and GoTrue (`/auth/v1`) HTTPS APIs.

## Features

- KPI cards with quarterly aggregation over Thai fiscal quarters
- Recent-transactions table with live refresh polling
- Email/password auth via Supabase GoTrue with persistent sessions
- Fully client-side WASM build, no Node toolchain required at runtime
- Static SPA deployment on Vercel

## Tech Stack

| Concern        | Choice                                               |
| -------------- | ---------------------------------------------------- |
| UI framework   | [Leptos](https://leptos.dev) 0.8, CSR only           |
| Router         | `leptos_router`                                      |
| Meta           | `leptos_meta`                                        |
| HTTP           | `gloo-net` (hand-rolled PostgREST + GoTrue clients)  |
| Storage        | `gloo-storage` (session token persistence)           |
| Time           | `chrono` with the `wasmbind` feature                 |
| Bundler / dev  | [Trunk](https://trunkrs.dev)                         |
| Styling        | Tailwind v4, precompiled to `public/styles/main.css` |

## Getting Started

### Prerequisites

- Rust stable with the `wasm32-unknown-unknown` target:
  ```sh
  rustup target add wasm32-unknown-unknown
  ```
- [Trunk](https://trunkrs.dev):
  ```sh
  cargo install trunk --locked
  ```

### Configuration

Supabase credentials are read in this order (see `src/core/supabase.rs`):

1. Build-time env vars - baked into the WASM via `option_env!`:
   - `SUPABASE_URL`
   - `SUPABASE_ANON_KEY`
2. Runtime `localStorage` fallback keys (no rebuild needed):
   - `medsup_supabase_url`
   - `medsup_supabase_anon`

Copy `.env.example` and export the values before building for production.

### Development

```sh
trunk serve            # http://127.0.0.1:3000, SPA fallback enabled
```

## Quality Gates

```sh
cargo fmt --all --check
cargo clippy --target wasm32-unknown-unknown -- -D warnings
cargo test --lib
trunk build --release
```

## Styling

Tailwind v4 classes live directly in the Rust `view!` markup. The compiled CSS
is committed at `public/styles/main.css`, so builds require **no Node
toolchain**. After adding/changing class names, regenerate it once:

```sh
bunx @tailwindcss/cli -i ./tailwind.input.css -o ./public/styles/main.css --minify
```

Design tokens (Mistral palette), golden shadow utilities, base styles, and the
custom scrollbar are defined in `tailwind.input.css`.

## Deployment (Vercel)

`vercel.json` builds with Trunk and serves `dist/` as a static SPA:

- catch-all rewrite `/(.*) -> /index.html` for deep links,
- CSP including `'wasm-unsafe-eval'` (required to instantiate WASM),
- `.wasm` served as `application/wasm` with immutable caching.

Set `SUPABASE_URL` / `SUPABASE_ANON_KEY` as build environment variables so
they get baked into the WASM bundle.

## Migration Notes (Vue → Leptos)

- Auth session persistence: the GoTrue access token is stored in
  `localStorage` (`medsup_supabase_token`) and revalidated against
  `GET /auth/v1/user` on startup - equivalent to supabase-js
  `persistSession`.
- Realtime: supabase-js realtime channels were replaced by a 30-second poll
  (`subscribe_refresh`) that refetches the selected fiscal year, matching the
  original debounced-refetch behaviour without a websocket dependency.
- Route guard (`beforeEach`) became nested `<Show>`s around two route trees;
  `/login ↔ /` redirects mirror the old guard rules.
- Fiscal year selector defaults to the current Thai fiscal year (Oct-Sep).

## License

Distributed under the [MIT License](LICENSE).
