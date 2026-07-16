import { listen } from '@tauri-apps/api/event';
import { api } from '../api/commands';
import type { RecurrenceType, ScheduleBlockView } from '../api/types';

function createScheduleStore() {
  let blocks = $state<ScheduleBlockView[]>([]);
  let suggestion = $state<string | null>(null);
  let suggesting = $state(false);
  let suggestError = $state<string | null>(null);

  async function refresh() {
    blocks = await api.getSchedule();
  }

  async function init() {
    await refresh();
    await listen('entry-logged', () => refresh());
  }

  async function add(
    title: string,
    pillarId: string | null,
    startTime: string,
    endTime: string,
    recurrenceType: RecurrenceType,
    recurrenceDays: number[] | null,
    onceDate: string | null,
  ) {
    blocks = await api.addScheduleBlock(
      title,
      pillarId,
      startTime,
      endTime,
      recurrenceType,
      recurrenceDays,
      onceDate,
      null,
    );
  }

  async function remove(blockId: string) {
    blocks = await api.deleteScheduleBlock(blockId);
  }

  async function suggest() {
    suggesting = true;
    suggestError = null;
    try {
      suggestion = await api.suggestSchedule();
    } catch (e) {
      suggestError = String(e);
    } finally {
      suggesting = false;
    }
  }

  function clearSuggestion() {
    suggestion = null;
    suggestError = null;
  }

  return {
    get blocks() {
      return blocks;
    },
    get suggestion() {
      return suggestion;
    },
    get suggesting() {
      return suggesting;
    },
    get suggestError() {
      return suggestError;
    },
    init,
    refresh,
    add,
    remove,
    suggest,
    clearSuggestion,
  };
}

export const schedule = createScheduleStore();
