# Via Ethos — Architecture & Design Proposal

> **Philosophy:** Focused Consistency through Rituals. Not a timer — a tool for
> building character and discipline through focused, habit-based work sessions.

This document is the output of the architecture phase. No application code has
been written yet; implementation begins from this blueprint.

---

## 1. Tech Stack: Tauri v2 + Svelte 5 + SQLite

**Recommendation: Tauri, not Electron.**

- **The app IS its aesthetic.** A calm ritual tool that idles at 400MB RAM with
  a Chromium process tree contradicts its own philosophy. Tauri ships a
  ~5–10MB binary using the OS webview, idles at ~30–60MB, and starts
  near-instantly — which matters for an app that lives in the system tray all day.
- **Tray + background timers are first-class in Tauri v2** (`tray-icon`,
  `notification`, `autostart` plugins). The timer core lives in Rust, so it
  keeps ticking accurately even if the webview is throttled or the window is
  closed — a real Electron pain point (background pages get timer throttling).
- **Electron's advantages don't apply here.** Electron wins when you need
  guaranteed-identical rendering across platforms or heavy Node-ecosystem
  integration. Via Ethos is a minimalist, mostly-static UI with subtle
  animation — well within what every OS webview renders identically.

**Frontend: Svelte 5** (plain Vite + Svelte, no SvelteKit). For a UI built on
gentle transitions (a breathing timer ring, a path that grows), Svelte's
built-in `transition`/`motion`/`tweened` primitives and tiny runtime beat
React + framer-motion in both bundle size and code clarity.

**Persistence: SQLite** via `tauri-plugin-sql` / `rusqlite`. Local-first, zero
server, queryable history for "The Path", trivially backed up (one file). Not
JSON files — session history grows and we need date-range aggregation.

**Layer split (important):** all domain logic — the timer state machine, Ethos
Points calculation, streak rules — lives in **Rust**, exposed to the UI via
Tauri commands and events. The Svelte layer is a pure rendering skin. This
makes the ritual logic testable without a UI and keeps the timer authoritative
in one place.

---

## 2. Data Model

Four tables plus one derived concept:

```sql
habits
  id            TEXT PRIMARY KEY  -- uuid
  name          TEXT              -- "Deep Writing"
  virtue        TEXT NULL         -- optional tag: Discipline, Wisdom, Craft…
  color_token   TEXT              -- one of a fixed calm palette, not free RGB
  target_minutes_per_day INTEGER NULL
  created_at    INTEGER           -- unix seconds
  archived_at   INTEGER NULL      -- never delete; the Path is a record

sessions
  id            TEXT PRIMARY KEY
  habit_id      TEXT REFERENCES habits(id)
  intention     TEXT NULL         -- the ritual's stated purpose, ≤140 chars
  planned_minutes INTEGER         -- e.g. 25
  started_at    INTEGER
  ended_at      INTEGER NULL
  outcome       TEXT              -- 'completed' | 'abandoned' | 'interrupted'
  reflection    TEXT NULL         -- optional 1-line closing note ("seal")

ethos_ledger                      -- append-only; points are EARNED events
  id            TEXT PRIMARY KEY
  habit_id      TEXT REFERENCES habits(id)
  session_id    TEXT REFERENCES sessions(id) NULL
  points        INTEGER           -- signed; decay entries are negative
  reason        TEXT              -- 'session_complete' | 'streak_bonus'
                                  --  | 'intention_kept' | 'decay'
  created_at    INTEGER

settings
  key           TEXT PRIMARY KEY  -- theme, tray behavior, ritual defaults
  value         TEXT
```

### Ethos Points rules (v1)

- **Base:** 1 point per completed focused minute (`completed` sessions only).
- **Streak multiplier:** consecutive *days* with ≥1 completed session on that
  habit. Multiplier = `1 + min(streak_days, 30) × 0.02` (caps at 1.6×) —
  rewards consistency without making a broken streak feel catastrophic.
- **Intention bonus:** +5 if the session had an intention *and* the user
  sealed it with a reflection. This mechanic makes the ritual framing real
  rather than decorative.
- **Abandoned sessions earn nothing but are recorded** — the Path shows honest
  history, not a highlight reel.
- **The ledger is append-only.** Current totals and streaks are computed views
  or caches over it. This gives auditability ("why do I have 3,412 points?"),
  makes future rule changes non-destructive, and lets "The Path" replay history.
- **Streaks are derived, not stored** (computed from `sessions` grouped by day,
  cached in memory) — stored streak counters always drift from truth eventually.

---

## 3. Design Philosophy: "The Path of Character"

Organizing metaphor: **a pilgrim's road, not a dashboard.**

- **One thing on screen.** The main window shows exactly one focus: before a
  session, the ritual prompt; during, the timer; after, the seal. Stats live
  behind a deliberate navigation step, never ambiently competing for attention.
- **The Ritual has three beats:**
  1. *Choose* — pick the habit; large, calm cards, no clutter.
  2. *Intend* — a single quiet text field: "What is this session for?"
     Skippable but gently encouraged.
  3. *Begin* — the UI fades everything except a thin ring or arc; the
     intention stays visible in small type as an anchor.
  Closing a session offers one line of reflection to **seal** it.
- **The Path itself:** progress is a horizontal winding line — each completed
  session a small stone/node placed along it, spaced by day, colored by habit.
  Milestones (7-day streak, 1,000 points) render as subtle roadside markers,
  not confetti. Scrolling left is literally walking back through your history.
  Missed days appear as gaps in the road — visible but unpunished.
- **Visual language:** near-monochrome warm neutrals (paper, ink, stone); one
  accent per habit from a fixed muted palette; a serif display face for ritual
  moments (intentions, milestones), a quiet sans for everything else; generous
  whitespace; motion limited to slow eased fades and the timer's breathing —
  nothing bounces, nothing pulses red.
- **The timer never nags.** Session end is a soft chime and slow brightening —
  a monastery bell, not an alarm. The tray icon is a minimal ring that fills:
  glanceable, silent.

---

## 4. Folder Structure

```
viaethos/
├── src/                        # Svelte frontend (rendering skin only)
│   ├── lib/
│   │   ├── components/
│   │   │   ├── ritual/         # ChooseHabit, IntentionPrompt, FocusRing, SealSession
│   │   │   ├── path/           # PathCanvas, PathNode, MilestoneMarker
│   │   │   └── shared/         # Button, TextField, Chime, ThemeProvider
│   │   ├── stores/             # Svelte stores mirroring Rust state via Tauri events
│   │   ├── api/                # thin typed wrappers around invoke() commands
│   │   └── theme/              # tokens.css — palette, type scale, motion durations
│   ├── routes/                 # simple view switch: Ritual / Path / Settings
│   ├── App.svelte
│   └── main.ts
├── src-tauri/                  # Rust backend (all domain logic)
│   ├── src/
│   │   ├── domain/
│   │   │   ├── timer.rs        # session state machine (authoritative clock)
│   │   │   ├── ethos.rs        # points rules, streak derivation
│   │   │   └── models.rs
│   │   ├── db/                 # migrations/, repository functions
│   │   ├── commands.rs         # Tauri command handlers (thin)
│   │   ├── tray.rs             # tray icon, menu, ring progress
│   │   └── main.rs
│   ├── migrations/             # numbered .sql files
│   ├── tauri.conf.json
│   └── Cargo.toml
├── ARCHITECTURE.md             # this document
└── package.json
```

### Key architectural decisions embedded here

1. The frontend never touches SQLite directly — everything goes through Tauri
   commands.
2. The timer emits `tick` / `phase-changed` events from Rust, so the UI window
   can be closed and reopened mid-session without losing the ritual.
3. Theme tokens live in one CSS file so the calm aesthetic is enforced by a
   constrained vocabulary rather than per-component taste.
