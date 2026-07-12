import { listen } from '@tauri-apps/api/event';
import { api } from '../api/commands';
import type { CourseView } from '../api/types';

function createCurriculumStore() {
  let courses = $state<CourseView[]>([]);

  async function refresh() {
    courses = await api.getCurriculum();
  }

  async function init() {
    await refresh();
    await listen('entry-logged', () => refresh());
  }

  async function addCourse(name: string, pillarId: string, targetHoursWeek: number) {
    courses = await api.addCourse(name, pillarId, targetHoursWeek);
  }

  async function addLesson(courseId: string, title: string, plannedOn: string) {
    courses = await api.addLesson(courseId, title, plannedOn);
  }

  async function completeLesson(lessonId: string) {
    courses = await api.completeLesson(lessonId);
  }

  async function skipLesson(lessonId: string) {
    courses = await api.skipLesson(lessonId);
  }

  return {
    get courses() {
      return courses;
    },
    init,
    refresh,
    addCourse,
    addLesson,
    completeLesson,
    skipLesson,
  };
}

export const curriculum = createCurriculumStore();
