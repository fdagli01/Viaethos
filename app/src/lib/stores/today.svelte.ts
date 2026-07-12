import { listen } from '@tauri-apps/api/event';
import { api } from '../api/commands';
import type { TodayView } from '../api/types';

function createTodayStore() {
  let view = $state<TodayView | null>(null);
  let loading = $state(true);

  async function refresh() {
    view = await api.getToday();
    loading = false;
  }

  async function init() {
    await refresh();
    await listen<string>('entry-logged', () => {
      refresh();
    });
  }

  return {
    get view() {
      return view;
    },
    get loading() {
      return loading;
    },
    refresh,
    init,
  };
}

export const today = createTodayStore();
