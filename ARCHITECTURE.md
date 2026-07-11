# Via Ethos — Architecture & Design Proposal (v2 — Life OS)

> **Philosophy:** Focused Consistency through Rituals. A holistic Life
> Operating System: habit tracker + task manager + pomodoro, organized
> around Pillars and a unified progress metaphor ("the Path").

Scope evolved from v1 (pomodoro-only) to a full Life OS:
- **Macro actions — Focus Sessions:** deep-work tasks run through the
  ritual timer (coding, writing).
- **Micro actions — Quick Ticks:** one-tap micro-habits (vitamins, water,
  making the bed).
- **Pillars of Ethos:** user-defined life categories (Mind, Body, Craft,
  Life) that structure everything and prevent UI clutter.
- **Ethos Points:** unified streak + invested-time metric across all pillars.

No application code yet; implementation begins from this blueprint.

---

## 1. Tech Stack: Tauri v2 + Svelte 5 + SQLite

**Tauri, not Electron.** A calm, tray-resident Life OS should not idle at
400MB with a Chromium process tree. Tauri: ~5–10MB binary, ~30–60MB idle,
instant start, first-class tray/notification/autostart plugins, and the
timer core lives in Rust so it ticks accurately even when the window is
closed or the webview is throttled.

**Frontend: Svelte 5 (runes) over React/Vue.** "Complex state" in this app
is wide but shallow — one day's entries, a handful of pillars, one active
timer. Svelte 5's fine-grained reactivity handles it without a state
library (no Redux/Zustand layer to maintain), and its built-in
transition/motion primitives fit the ritual aesthetic at a fraction of the
bundle. If team familiarity ever forces React, the equivalent is
React + Zustand + framer-motion — heavier, no capability gained.

**State architecture (the important part, framework-agnostic):**
- **Rust is the single source of truth** for domain state: timer state
  machine, today's schedule resolution, streaks, Ethos Points.
- The frontend holds only **view-model mirrors**, hydrated via Tauri
  commands and kept live via events (`tick`, `entry-logged`,
  `day-rolled-over`). UI can crash/close/reopen with zero data risk.
- **Persistence: SQLite** (`rusqlite` behind repository functions). Not
  JSON: a Life OS accumulates years of logs and needs date-range
  aggregation, joins across pillars, and single-file backup.

---

## 2. Data Model

Six tables. The key unification: **macro and micro actions live in one
`actions` table** discriminated by `kind`, and **all completions live in
one `entries` table** — so streaks, the Path, and Ethos Points are computed
by ONE code path regardless of action type.

```sql
pillars
  id            TEXT PRIMARY KEY   -- uuid
  name          TEXT               -- "Mind", "Body", "Craft", "Life"
  color_token   TEXT               -- from a fixed calm palette
  icon          TEXT NULL          -- from a curated minimal icon set
  sort_order    INTEGER
  archived_at   INTEGER NULL       -- never delete; the Path is a record

actions                            -- both macro habits and micro habits
  id            TEXT PRIMARY KEY
  pillar_id     TEXT REFERENCES pillars(id)
  name          TEXT               -- "Deep Writing" / "Take vitamins"
  kind          TEXT               -- 'focus' (macro) | 'tick' (micro)
  default_minutes INTEGER NULL     -- focus only: default session length
  schedule      TEXT               -- JSON recurrence: {"type":"daily"} |
                                   -- {"type":"weekdays"} | {"type":"days","days":[1,3,5]} |
                                   -- {"type":"times_per_week","n":3}
  target_per_day INTEGER DEFAULT 1 -- e.g. water ×8
  sort_order    INTEGER
  created_at    INTEGER
  archived_at   INTEGER NULL

entries                            -- every completion, macro or micro
  id            TEXT PRIMARY KEY
  action_id     TEXT REFERENCES actions(id)
  kind          TEXT               -- copied from action: 'focus' | 'tick'
  occurred_on   TEXT               -- local date 'YYYY-MM-DD' (day-boundary
                                   -- setting applied, e.g. day ends 04:00)
  -- focus-only columns (NULL for ticks):
  intention     TEXT NULL          -- the ritual's stated purpose, ≤140 chars
  planned_minutes INTEGER NULL
  started_at    INTEGER NULL
  ended_at      INTEGER NULL
  outcome       TEXT NULL          -- 'completed' | 'abandoned' | 'interrupted'
  reflection    TEXT NULL          -- optional closing note ("seal")
  created_at    INTEGER

tasks                              -- one-off to-dos (task manager side)
  id            TEXT PRIMARY KEY
  pillar_id     TEXT REFERENCES pillars(id) NULL
  action_id     TEXT REFERENCES actions(id) NULL  -- optional link: task done
                                                  -- via a focus session
  title         TEXT
  due_on        TEXT NULL          -- 'YYYY-MM-DD'
  completed_at  INTEGER NULL
  created_at    INTEGER

ethos_ledger                       -- append-only; points are EARNED events
  id            TEXT PRIMARY KEY
  pillar_id     TEXT REFERENCES pillars(id)
  action_id     TEXT REFERENCES actions(id)
  entry_id      TEXT REFERENCES entries(id) NULL
  points        INTEGER            -- signed
  reason        TEXT               -- 'focus_complete' | 'tick_complete'
                                   --  | 'streak_bonus' | 'intention_kept'
  created_at    INTEGER

settings (key TEXT PRIMARY KEY, value TEXT)  -- theme, day boundary, tray, defaults
```

### Design decisions

- **`occurred_on` is a local date string with a user-configurable day
  boundary** (night owls: day ends at 04:00). All streaks group by this
  column — never by raw timestamps.
- **Streaks are derived, never stored.** Computed per action from `entries`
  grouped by `occurred_on` against the action's `schedule` (a
  `times_per_week` habit doesn't break its streak on a rest day). Cached in
  memory, rebuilt on launch.
- **The ledger is append-only.** Totals per pillar / per action are views
  over it. Auditable, and future rule changes never corrupt history.
- **Tasks are separate from actions.** Habits recur; tasks die when done.
  Linking a task to a focus session is optional metadata, not a forced
  unification — merging them is the classic Life-OS modeling mistake.

### Ethos Points rules (v1)

| Event | Points |
|---|---|
| Focus minute completed | 1 / minute |
| Quick Tick completed | 5 flat |
| Streak multiplier (per action) | ×(1 + min(streak_days, 30) × 0.02), cap ×1.6 |
| Intention set AND sealed with reflection | +5 |
| Abandoned focus session | 0 (but recorded — the Path is honest) |

Points aggregate upward: entry → action → pillar → total. The Path can
therefore be rendered at any zoom level from the same ledger.

---

## 3. UI/UX Vision: a day without overwhelm

Organizing metaphor: **a pilgrim's road, not a dashboard.** The
anti-overwhelm strategy is structural, not cosmetic:

1. **"Today" is the only landing screen** — never an all-time dashboard.
   It answers one question: *what does my day ask of me?*
2. **Pillars are the collapse mechanism.** Today's screen is a vertical
   flow of pillar sections. Each shows a thin progress hairline and its
   remaining items; completed items fold away into a muted "done" line
   (visible count, collapsed detail). A fully honored pillar collapses to
   a single serene row with a filled hairline.
3. **Micro vs macro have different interaction weights.**
   - *Quick Ticks:* a single row with a circle — one click, soft ink-fill
     animation, done. Multi-count ticks (water ×8) fill a segmented circle.
     Zero navigation, zero modal.
   - *Focus Sessions:* clicking one **leaves the dashboard entirely** and
     enters the Ritual: Choose (already chosen) → Intend (one quiet field:
     "What is this session for?") → Begin (everything fades except a thin
     breathing ring + the intention in small type). Ending offers one line
     of reflection to seal it. Full-screen takeover is the feature: focus
     and overview are different modes and never share a screen.
4. **The Path is a ribbon, then a place.** Today's screen carries only a
   thin horizontal path ribbon at the top — last ~14 days as small stones,
   colored by pillar mix, gaps visible but unpunished. Clicking it opens
   the full Path view: a winding road through history with milestone
   markers (7-day streaks, 1,000 points) as subtle roadside stones, per-
   pillar filtering as a quiet legend. Scrolling left is walking back.
5. **Numbers are ambient, not central.** Ethos Points appear as a small
   total near the ribbon; no charts on Today. Analytics live inside the
   Path view for when the user *chooses* reflection.
6. **Visual language:** warm paper/ink/stone neutrals; one muted accent
   per pillar (fixed palette, not free RGB); serif display face reserved
   for ritual moments (intentions, milestones), quiet sans elsewhere;
   motion limited to slow eased fades and the ring's breathing. Session
   end is a soft chime and slow brightening — a monastery bell, not an
   alarm. The tray icon is a minimal filling ring; tray menu offers the
   top 3 pending Quick Ticks for logging without opening the window.

Views total: **Today / Ritual (modal state) / Path / Manage (pillars,
actions, tasks) / Settings.** Nothing else.

---

## 4. Folder Structure

```
viaethos/
├── src/                        # Svelte frontend (rendering skin only)
│   ├── lib/
│   │   ├── components/
│   │   │   ├── today/          # PillarSection, TickRow, FocusRow, PathRibbon
│   │   │   ├── ritual/         # IntentionPrompt, FocusRing, SealSession
│   │   │   ├── path/           # PathCanvas, PathNode, MilestoneMarker
│   │   │   ├── manage/         # PillarEditor, ActionEditor, TaskList
│   │   │   └── shared/         # Button, TextField, Chime, ThemeProvider
│   │   ├── stores/             # view-model mirrors of Rust state (via events)
│   │   ├── api/                # thin typed wrappers around invoke() commands
│   │   └── theme/              # tokens.css — palette, type scale, motion
│   ├── routes/                 # Today / Path / Manage / Settings switch
│   ├── App.svelte
│   └── main.ts
├── src-tauri/                  # Rust backend (all domain logic)
│   ├── src/
│   │   ├── domain/
│   │   │   ├── timer.rs        # focus-session state machine (authoritative clock)
│   │   │   ├── schedule.rs     # recurrence resolution → "what does today ask?"
│   │   │   ├── streak.rs       # streak derivation per action/schedule
│   │   │   ├── ethos.rs        # points rules, ledger writes
│   │   │   └── models.rs
│   │   ├── db/                 # repository functions
│   │   ├── commands.rs         # Tauri command handlers (thin)
│   │   ├── events.rs           # tick / entry-logged / day-rolled-over
│   │   ├── tray.rs             # tray ring, quick-tick menu
│   │   └── main.rs
│   ├── migrations/             # numbered .sql files
│   ├── tauri.conf.json
│   └── Cargo.toml
├── ARCHITECTURE.md             # this document
└── package.json
```

### Key architectural invariants

1. Frontend never touches SQLite — everything through commands.
2. The timer and day-rollover logic live in Rust; the window can close and
   reopen mid-session without losing the ritual.
3. One `entries` table + one streak engine serves both macro and micro —
   no duplicated logic paths.
4. Theme tokens live in one CSS file: calm is enforced by a constrained
   vocabulary, not per-component taste.
