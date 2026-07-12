import { invoke } from '@tauri-apps/api/core';
import type { LedgerStats, QuietModeView, Task, TodayView, WeatherSnapshot } from './types';

export const api = {
  getToday: () => invoke<TodayView>('get_today'),
  completeTick: (actionId: string) => invoke<TodayView>('complete_tick', { actionId }),
  startFocus: (actionId: string, intention: string | null, plannedMinutes: number) =>
    invoke<TodayView>('start_focus', { actionId, intention, plannedMinutes }),
  endFocus: (entryId: string, outcome: 'completed' | 'abandoned' | 'interrupted', reflection: string | null) =>
    invoke<TodayView>('end_focus', { entryId, outcome, reflection }),
  getLedgerStats: () => invoke<LedgerStats>('get_ledger_stats'),
  getQuietMode: () => invoke<QuietModeView>('get_quiet_mode'),
  setInnerWeather: (weather: 'clear' | 'radiant' | 'heavy' | 'stormy') =>
    invoke<void>('set_inner_weather', { weather }),
  getTasks: () => invoke<Task[]>('get_tasks'),
  addTask: (title: string, pillarId: string | null, dueOn: string | null) =>
    invoke<Task[]>('add_task', { title, pillarId, dueOn }),
  completeTask: (taskId: string) => invoke<Task[]>('complete_task', { taskId }),
  deleteTask: (taskId: string) => invoke<Task[]>('delete_task', { taskId }),
  getWeather: () => invoke<WeatherSnapshot | null>('get_weather'),
};
