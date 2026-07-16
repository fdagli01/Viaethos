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

export interface Pillar {
  id: string;
  name: string;
  color_token: string;
  icon: string | null;
  sort_order: number;
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

export interface SleepDay {
  date: string;
  hours: number;
  quality_1_5: number | null;
}

export interface LedgerStats {
  total_points: number;
  points_trend: DayPoint[];
  focus_by_pillar_day: PillarFocusDay[];
  streaks: StreakRow[];
  sleep_by_day: SleepDay[];
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

export interface FoodItem {
  id: string;
  name: string;
  kcal_per_100g: number;
  protein_per_100g: number;
  carb_per_100g: number;
  fat_per_100g: number;
  user_defined: boolean;
}

export type TimeSlot = 'breakfast' | 'lunch' | 'dinner' | 'snack';

export interface Meal {
  id: string;
  occurred_on: string;
  time_slot: TimeSlot;
  name: string;
  kcal: number;
  protein_g: number;
  carb_g: number;
  fat_g: number;
  note: string | null;
  created_at: number;
}

export interface MealsView {
  meals: Meal[];
  kcal_total: number;
  kcal_budget: number;
}

export interface Lesson {
  id: string;
  course_id: string;
  title: string;
  planned_on: string;
  status: 'planned' | 'done' | 'skipped';
  review_of: string | null;
  sort_order: number;
  created_at: number;
}

export interface CourseDetailView {
  course: CourseView;
  all_lessons: Lesson[];
  upcoming: Lesson[];
}

export interface CourseView {
  id: string;
  name: string;
  pillar_id: string;
  action_id: string;
  color_token: string;
  target_hours_week: number;
  done_lessons: number;
  total_lessons: number;
  due_lessons: Lesson[];
}

export interface PathMilestone {
  date: string;
  label: string;
  kind: 'streak' | 'points';
}

export interface PathHistoryView {
  days: PathDay[];
  milestones: PathMilestone[];
}

export interface ActionAdminView {
  id: string;
  pillar_id: string;
  name: string;
  kind: 'focus' | 'tick';
  default_minutes: number | null;
  schedule_type: 'daily' | 'weekdays' | 'times_per_week';
  times_per_week: number | null;
  target_per_day: number;
  archived: boolean;
}

export interface SettingsView {
  day_boundary_hour: number;
  calorie_budget: number;
  weather_lat: number;
  weather_lon: number;
  has_ai_api_key: boolean;
}

export type RecurrenceType = 'daily' | 'weekdays' | 'days' | 'once';

export interface ScheduleBlockView {
  id: string;
  title: string;
  pillar_id: string | null;
  pillar_color_token: string | null;
  start_time: string;
  end_time: string;
  recurrence_type: RecurrenceType;
  recurrence_days: number[] | null;
  once_date: string | null;
  note: string | null;
  is_now: boolean;
}

export interface MementoMoriView {
  birth_date: string | null;
  weeks_lived: number | null;
  weeks_total: number;
}

export interface SleepLog {
  id: string;
  date: string;
  bed_at: string;
  woke_at: string;
  quality_1_5: number;
  created_at: number;
}

export interface WeatherSnapshot {
  fetched_at: number;
  temperature_c: number;
  weather_code: number;
  wind_speed_kmh: number;
  precipitation_mm: number;
  is_day: boolean;
  stale: boolean;
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
  kcal_today: number;
  kcal_budget: number;
  course_bands: { name: string; color_token: string; ratio: number }[];
  sleep_hours: number | null;
  sleep_quality: number | null;
}
