import { listen } from '@tauri-apps/api/event';
import { api } from '../api/commands';
import type { Task } from '../api/types';

function createTasksStore() {
  let list = $state<Task[]>([]);

  async function refresh() {
    list = await api.getTasks();
  }

  async function init() {
    await refresh();
    await listen('entry-logged', () => refresh());
  }

  async function add(title: string, pillarId: string | null, dueOn: string | null) {
    list = await api.addTask(title, pillarId, dueOn);
  }

  async function complete(taskId: string) {
    list = await api.completeTask(taskId);
  }

  async function remove(taskId: string) {
    list = await api.deleteTask(taskId);
  }

  return {
    get list() {
      return list;
    },
    init,
    refresh,
    add,
    complete,
    remove,
  };
}

export const tasks = createTasksStore();
