use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pillar {
    pub id: String,
    pub name: String,
    pub color_token: String,
    pub icon: Option<String>,
    pub sort_order: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
    Focus,
    Tick,
}

impl ActionKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActionKind::Focus => "focus",
            ActionKind::Tick => "tick",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "focus" => ActionKind::Focus,
            _ => ActionKind::Tick,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub pillar_id: String,
    pub name: String,
    pub kind: ActionKind,
    pub default_minutes: Option<i64>,
    pub schedule: Schedule,
    pub target_per_day: i64,
    pub sort_order: i64,
}

/// Recurrence rule for an action. Mirrors the JSON stored in `actions.schedule`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Schedule {
    Daily,
    Weekdays,
    Days { days: Vec<u8> }, // 0=Sun .. 6=Sat
    TimesPerWeek { n: u8 },
}

impl Schedule {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{\"type\":\"daily\"}".to_string())
    }

    pub fn from_json(s: &str) -> Self {
        serde_json::from_str(s).unwrap_or(Schedule::Daily)
    }

    /// Is this action "asked for" on the given weekday (0=Sun..6=Sat)?
    /// `times_per_week` is never day-locked — it's satisfied by count, not date.
    pub fn is_due_on_weekday(&self, weekday: u32) -> bool {
        match self {
            Schedule::Daily => true,
            Schedule::Weekdays => (1..=5).contains(&weekday),
            Schedule::Days { days } => days.contains(&(weekday as u8)),
            Schedule::TimesPerWeek { .. } => true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub action_id: String,
    pub kind: ActionKind,
    pub occurred_on: String,
    pub intention: Option<String>,
    pub planned_minutes: Option<i64>,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
    pub outcome: Option<String>,
    pub reflection: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub pillar_id: Option<String>,
    pub action_id: Option<String>,
    pub title: String,
    pub due_on: Option<String>,
    pub completed_at: Option<i64>,
    pub created_at: i64,
}
