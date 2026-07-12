export interface ActiveSessionView {
  entry_id: string;
  intention: string | null;
  started_at: number;
  planned_minutes: number;
}

export interface ActionView {
  id: string;
  pillar_id: string;
  name: string;
  kind: 'focus' | 'tick';
  default_minutes: number | null;
  target_per_day: number;
  today_count: number;
  streak: number;
  best_streak: number;
  due_today: boolean;
  active_session: ActiveSessionView | null;
}

export interface PillarView {
  id: string;
  name: string;
  color_token: string;
  actions: ActionView[];
}

export interface TodayView {
  pillars: PillarView[];
  total_points: number;
}

export interface DayPoint {
  date: string;
  points: number;
}

export interface PillarFocusDay {
  date: string;
  by_pillar: [string, number][];
}

export interface StreakRow {
  action_name: string;
  pillar_name: string;
  color_token: string;
  kind: 'focus' | 'tick';
  current: number;
  best: number;
}

export interface LedgerStats {
  total_points: number;
  points_trend: DayPoint[];
  focus_by_pillar_day: PillarFocusDay[];
  streaks: StreakRow[];
}

export interface PathDay {
  date: string;
  pillar_colors: string[];
}

export interface Task {
  id: string;
  pillar_id: string | null;
  action_id: string | null;
  title: string;
  due_on: string | null;
  completed_at: number | null;
  created_at: number;
}

export interface Milestone {
  title: string;
  color_token: string;
  overdue: boolean;
}

export interface QuietModeView {
  honored_today: number;
  due_today: number;
  focus_minutes_today: number;
  total_points: number;
  best_streak: number;
  inner_weather: 'clear' | 'radiant' | 'heavy' | 'stormy';
  path: PathDay[];
  milestones: Milestone[];
}
