# Via Ethos

**A calm, local-first "life OS" for the desktop.** Via Ethos combines a habit tracker, a focus timer, a task list and small life logs (meals, study plan, sleep) into one app, and renders your day as a slowly changing painted landscape instead of a dashboard.

Everything is organised around user-defined **Pillars** (for example Mind, Body, Craft, Life). Every completion, from a one-tap habit to a 50-minute focus session, feeds one points system, so streaks and history are computed the same way across all modules.

## Screens

- **Quiet Mode** – the default view: a full-window canvas painting (wheat-field style brushwork) whose sky, field and scene elements reflect the day: real weather, your self-reported "inner weather", calories, study progress, sleep and the last 14 days.
- **Today** – the day's habits grouped by pillar, focus sessions started through a short ritual (intention, timer, closing reflection), tasks, meal log (*Sofra*), study plan (*Müfredat*), sleep log and an hourly schedule.
- **Path** – a horizontally scrollable history (30/90/365 days) where each day is a stone coloured by pillar, with streak and point milestones and a pillar filter.
- **The Ledger** – statistics: focus hours per pillar, sleep, Ethos Points over 30 days and streaks.
- **Memento Mori** – a life-in-weeks view based on a birth date the user enters.
- **Manage / Settings** – edit pillars and habits, day boundary, weather location and optional features.

## Features

- **Two kinds of actions.** *Focus* actions run through the ritual timer; *Tick* actions are one-tap micro-habits with an optional daily target (e.g. water ×8).
- **Flexible schedules.** Daily, weekdays, specific weekdays or *n* times per week; streaks respect the schedule, so a rest day does not break a 3×/week habit.
- **Configurable day boundary.** Days can end after midnight (e.g. 04:00); all streaks group by this local date.
- **Ethos Points.** An append-only ledger: 1 point per focus minute, flat points for ticks, tasks, meals, lessons and sleep logs, a streak multiplier capped at ×1.6 and a bonus for sealing a session with a reflection. Abandoned sessions earn nothing but are still recorded.
- **Study plan.** Courses with lessons, reordering, skip/complete and automatically scheduled review lessons with an "upcoming reviews" list.
- **Meals.** Meal logging with calories and macros, backed by a small seeded food database that users can extend.
- **Real weather.** Current conditions from [Open-Meteo](https://open-meteo.com) (no API key), cached for 20 minutes; the last snapshot is used when offline.
- **Optional schedule suggestions.** With a user-supplied Anthropic API key, the app sends a short summary of the day's schedule and open tasks to the Messages API and shows a few practical suggestions (in Turkish).

## Tech stack

| Area | Technology |
| --- | --- |
| Desktop shell | Tauri 2 |
| Core logic | Rust 2021 (streaks, points, schedule resolution, commands) |
| Storage | SQLite via `rusqlite` (bundled), schema in `migrations/0001_init.sql` |
| HTTP | `reqwest` with rustls (weather and optional suggestions) |
| UI | Svelte 5 (runes), TypeScript, Vite |
| Painting | Plain HTML canvas rendering, no graphics library |

## Architecture

```
Svelte views ──► stores (view-model mirrors) ──► Tauri commands (Rust)
                                                   │
                                   domain/ (streaks, points, schedules)
                                                   │
                                         db/repo.rs ──► SQLite (app data dir)
                                                   │
                                   weather.rs ──► Open-Meteo
                                   ai.rs ──► Messages API (optional)
```

Rust is the single source of truth for domain state; the frontend only holds mirrors that it refreshes through Tauri commands. Points are never stored as a running total – totals are sums over the `ethos_ledger` table, and streaks are derived from entries rather than stored. The database lock is released before any network request.

More detail: [ARCHITECTURE.md](ARCHITECTURE.md) (data model, points rules, UI concept) and [IDEAS.md](IDEAS.md) (design notes and version log, in Turkish). Static HTML mockups of Quiet Mode and the Ledger are in [`design/mockups/`](design/mockups/).

## Getting started

Requires Node.js, a Rust toolchain (1.77.2 or newer) and the [Tauri 2 system prerequisites](https://v2.tauri.app/start/prerequisites/) for your platform.

```bash
cd app
npm install
npx tauri dev       # starts Vite on port 1420 and opens the desktop window
```

| Command (in `app/`) | Purpose |
| --- | --- |
| `npm run dev` | Frontend only, in the browser (Tauri commands are unavailable there) |
| `npm run build` | Build the frontend into `app/dist` |
| `npm run check` | Svelte and TypeScript type checks |
| `npx tauri build` | Build the desktop bundle |

On first launch the app creates `via-ethos.sqlite3` in the platform's app data directory and seeds default pillars, habits and food items. The weather location defaults to Istanbul and can be changed in Settings.

## Checks

There are currently no automated tests. The available checks are `npm run check` (type checks) and `cargo check` / `cargo build` in `app/src-tauri` (the Rust build expects `app/dist` to exist, so run `npm run build` first).

## Project structure

```
ARCHITECTURE.md        Data model, points rules, UI concept
IDEAS.md               Design notes and version log (Turkish)
design/mockups/        Static HTML mockups
app/
  src/
    App.svelte         Top navigation between the seven screens
    lib/components/    Screens and widgets (Today, QuietMode, Path, Ledger, ...)
    lib/stores/        Svelte 5 state mirrors per module
    lib/painting/      Canvas painting engine and quotes
    lib/api/           Typed wrappers for the Tauri commands
  src-tauri/
    migrations/        SQLite schema
    src/commands.rs    Tauri command handlers
    src/domain/        Streaks, points, schedules, models
    src/db/            Connection setup, seeding and queries
    src/weather.rs     Open-Meteo client
    src/ai.rs          Optional schedule-suggestion client
```

## Status and limitations

- Personal project in active development (internal version 0.1.0); no releases or installers have been published yet.
- No automated test suite yet.
- The interface mixes English labels with Turkish module names and texts; there is no localisation layer.
- Schema changes are applied by a single idempotent init script rather than versioned migrations.
- The optional API key is stored in plain text in the local SQLite database.
- Tray, notifications and autostart described in the architecture document are planned, not implemented.
