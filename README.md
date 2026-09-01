# MedSup Dash

```
███╗   ███╗███████╗██████╗  ██████╗██╗   ██╗██████╗ ██████╗  █████╗  ██████╗██╗  ██╗
████╗ ████║██╔════╝██╔══██╗██╔════╝██║   ██║██╔══██╗██╔══██╗██╔══██╗██╔════╝██║  ██║
██╔████╔██║█████╗  ██║  ██║███████╗██║   ██║██████╔╝██║  ██║███████║███████╗███████║
██║╚██╔╝██║██╔══╝  ██║  ██║╚════██║██║   ██║██╔═══╝ ██║  ██║██╔══██║╚════██║██║  ██║
██║ ╚═╝ ██║███████╗██████╔╝██████╔╝╚██████╔╝██║     ██████╔╝██║  ██║██████╔╝██║  ██║
╚═╝     ╚═╝╚══════╝╚═════╝ ╚═════╝ ╚═══════╝╚═╝     ╚═════╝ ╚═╝  ╚═╝╚═════╝ ╚═╝  ╚═╝
```

---

## ◆ PULSE

A support budget is a promise with a ledger. MedSup Dash tracks the
value of supported medication at Sabot Hospital - every `med_transactions`
row, bucketed into Thai fiscal quarters (ต.ค. - ก.ย.) - and answers the
question a budget review starts with: what has the support money bought
so far this year? KPI cards, a quarterly report, and a recent
transactions table that refreshes itself. Signed in, rendered in WASM,
deployed as a static page - no Node toolchain anywhere at runtime.

| KPIs ▣ | Quarterly ▣ | Live table ▣ | Auth ▣ |
|---|---|---|---|

*v1.1.12 - the fiscal year is told, quarter by quarter.*

> Built with Rust + Leptos 0.8, talking to PostgREST and GoTrue through
> hand-rolled clients - the same Supabase, a leaner messenger.
>
> **suradet-ps**, artifact keeper

---

## ◆ IGNITION

One target, one tool, one command.

```
⟫ rustup target add wasm32-unknown-unknown
⟫ cargo install trunk --locked
⟫ trunk serve
```

Open [http://127.0.0.1:3000](http://127.0.0.1:3000) - SPA fallback is on.

<details>
<summary>Configuration</summary>

Supabase credentials resolve in this order (`src/core/supabase.rs`):

1. Build-time env vars baked into the WASM via `option_env!`:
   `SUPABASE_URL`, `SUPABASE_ANON_KEY`
2. Runtime `localStorage` fallback (no rebuild):
   `medsup_supabase_url`, `medsup_supabase_anon`

Copy `.env.example` and export the values before a production build.

</details>

---

## ◆ ANATOMY

One screen, one ledger, a handful of honest numbers.

- **Authenticates** - GoTrue email/password sign-in with persistent
  sessions: the token lives in `localStorage` and is revalidated against
  `/auth/v1/user` on startup - no silent session drift.
- **Aggregates** - the KPI cards bucket `med_transactions` into Thai
  fiscal quarters; the fiscal year selector defaults to the current
  Oct-Sep year.
- **Refreshes** - the recent-transactions table polls every 30 seconds -
  the realtime channel from the Vue days became a quiet poll, matching
  the old debounced behaviour without a websocket dependency.
- **Renders** - Leptos 0.8 CSR compiles the whole app to WASM; Tailwind
  v4 classes live in the Rust `view!` markup with the CSS committed
  precompiled - no Node toolchain required to build or run.
- **Guards** - the route guard became nested `<Show>`s around two route
  trees; `/login` and `/` redirects mirror the old guard rules exactly.

---

## ◆ RITUALS

**The core ceremony** - the quarterly budget review:

1. Sign in. The session remembers you; the token is revalidated before
   the first number loads.
2. Pick the fiscal year - the current Oct-Sep year is already chosen.
3. Read the KPIs: what the support money bought, quarter by quarter.
4. Scan the recent transactions - the table refreshes itself every 30
   seconds, so the page is never stale.

**The ceremony of the ledger** - every Baht in the dashboard traces to
a row in `med_transactions`. The aggregation is quartered, the table is
recent-first, and the total is computed - never remembered.

**The ceremony of the migration** - Vue and supabase-js are gone, but
the behaviour is documented: token persistence, polling instead of
websockets, guard semantics - every replacement named in the migration
notes, nothing silently changed.

---

## ◆ ECHOES

**Where this artifact is heading**

```
auth     ▸ GoTrue sign-in, persistent session revalidation ────────── ▸ sealed
aggregate ▸ fiscal-quarter KPI cards, year selector ────────────────── ▸ sealed
refresh  ▸ 30s polling table, no websocket dependency ──────────────── ▸ sealed
deliver  ▸ WASM static SPA on Vercel, CSP with wasm-unsafe-eval ────── ▸ sealed
```

**Raising the artifact** - the quality gates are explicit: `cargo fmt
--all --check`, `cargo clippy --target wasm32-unknown-unknown --
-D warnings`, `cargo test --lib`, `trunk build --release`. Open an
issue first to discuss a change.

**Status** - CI and CodeQL gate every push; releases are tagged.
[Watch the gates](.github/workflows).

---

```
  ─────────────────────────────────────────
   A support budget without a ledger
   is a promise with amnesia.
  ─────────────────────────────────────────
```

Distributed under the [MIT License](LICENSE).