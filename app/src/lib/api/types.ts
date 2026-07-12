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
