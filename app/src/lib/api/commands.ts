import { invoke } from '@tauri-apps/api/core';
import type {
  CourseView,
  FoodItem,
  LedgerStats,
  MealsView,
  MementoMoriView,
  QuietModeView,
  SleepLog,
  Task,
  TimeSlot,
  TodayView,
  WeatherSnapshot,
} from './types';

export const api = {
  getToday: () => invoke<TodayView>('get_today'),
  completeTick: (actionId: string) => invoke<TodayView>('complete_tick', { actionId }),
  startFocus: (
    actionId: string,
    intention: string | null,
    plannedMinutes: number,
    lessonId: string | null = null,
  ) => invoke<TodayView>('start_focus', { actionId, intention, plannedMinutes, lessonId }),
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
  searchFoodItems: (query: string) => invoke<FoodItem[]>('search_food_items', { query }),
  addFoodItem: (
    name: string,
    kcalPer100g: number,
    proteinPer100g: number,
    carbPer100g: number,
    fatPer100g: number,
  ) =>
    invoke<FoodItem>('add_food_item', {
      name,
      kcalPer100g,
      proteinPer100g,
      carbPer100g,
      fatPer100g,
    }),
  getMealsToday: () => invoke<MealsView>('get_meals_today'),
  addMeal: (
    timeSlot: TimeSlot,
    name: string,
    kcal: number,
    proteinG: number,
    carbG: number,
    fatG: number,
    note: string | null,
  ) => invoke<MealsView>('add_meal', { timeSlot, name, kcal, proteinG, carbG, fatG, note }),
  deleteMeal: (mealId: string) => invoke<MealsView>('delete_meal', { mealId }),
  getCurriculum: () => invoke<CourseView[]>('get_curriculum'),
  addCourse: (name: string, pillarId: string, targetHoursWeek: number) =>
    invoke<CourseView[]>('add_course', { name, pillarId, targetHoursWeek }),
  addLesson: (courseId: string, title: string, plannedOn: string) =>
    invoke<CourseView[]>('add_lesson', { courseId, title, plannedOn }),
  completeLesson: (lessonId: string) => invoke<CourseView[]>('complete_lesson', { lessonId }),
  skipLesson: (lessonId: string) => invoke<CourseView[]>('skip_lesson', { lessonId }),
  getLastSleep: () => invoke<SleepLog | null>('get_last_sleep'),
  logSleep: (date: string, bedAt: string, wokeAt: string, quality: number) =>
    invoke<SleepLog>('log_sleep', { date, bedAt, wokeAt, quality }),
  getMementoMori: () => invoke<MementoMoriView>('get_memento_mori'),
  setBirthDate: (birthDate: string) => invoke<MementoMoriView>('set_birth_date', { birthDate }),
};
