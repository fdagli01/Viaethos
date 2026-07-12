import { listen } from '@tauri-apps/api/event';
import { api } from '../api/commands';
import type { MealsView, TimeSlot } from '../api/types';

function createMealsStore() {
  let view = $state<MealsView | null>(null);

  async function refresh() {
    view = await api.getMealsToday();
  }

  async function init() {
    await refresh();
    await listen('entry-logged', () => refresh());
  }

  async function log(
    timeSlot: TimeSlot,
    name: string,
    kcal: number,
    proteinG: number,
    carbG: number,
    fatG: number,
  ) {
    view = await api.addMeal(timeSlot, name, kcal, proteinG, carbG, fatG, null);
  }

  async function remove(mealId: string) {
    view = await api.deleteMeal(mealId);
  }

  return {
    get view() {
      return view;
    },
    init,
    refresh,
    log,
    remove,
  };
}

export const meals = createMealsStore();
