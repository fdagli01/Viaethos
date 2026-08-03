import { listen } from '@tauri-apps/api/event';
import { api } from '../api/commands';
import type { MealPreset, MealsView, TimeSlot } from '../api/types';

function createMealsStore() {
  let view = $state<MealsView | null>(null);
  let presets = $state<MealPreset[]>([]);

  async function refresh() {
    view = await api.getMealsToday();
    presets = await api.getMealPresets();
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
    presets = await api.getMealPresets();
  }

  async function logPreset(timeSlot: TimeSlot, preset: MealPreset) {
    await log(timeSlot, preset.name, preset.kcal, preset.protein_g, preset.carb_g, preset.fat_g);
  }

  async function remove(mealId: string) {
    view = await api.deleteMeal(mealId);
  }

  return {
    get view() {
      return view;
    },
    get presets() {
      return presets;
    },
    init,
    refresh,
    log,
    logPreset,
    remove,
  };
}

export const meals = createMealsStore();
