import { listen } from '@tauri-apps/api/event';
import { api } from '../api/commands';
import type { SleepLog } from '../api/types';

function createSleepStore() {
  let last = $state<SleepLog | null>(null);

  async function refresh() {
    last = await api.getLastSleep();
  }

  async function init() {
    await refresh();
    await listen('entry-logged', () => refresh());
  }

  async function log(date: string, bedAt: string, wokeAt: string, quality: number) {
    last = await api.logSleep(date, bedAt, wokeAt, quality);
  }

  return {
    get last() {
      return last;
    },
    init,
    refresh,
    log,
  };
}

export const sleep = createSleepStore();
