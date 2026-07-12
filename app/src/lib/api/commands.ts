import { invoke } from '@tauri-apps/api/core';
import type { LedgerStats, TodayView } from './types';

export const api = {
  getToday: () => invoke<TodayView>('get_today'),
  completeTick: (actionId: string) => invoke<TodayView>('complete_tick', { actionId }),
  startFocus: (actionId: string, intention: string | null, plannedMinutes: number) =>
    invoke<TodayView>('start_focus', { actionId, intention, plannedMinutes }),
  endFocus: (entryId: string, outcome: 'completed' | 'abandoned' | 'interrupted', reflection: string | null) =>
    invoke<TodayView>('end_focus', { entryId, outcome, reflection }),
  getLedgerStats: () => invoke<LedgerStats>('get_ledger_stats'),
};
