import type { ActionView } from '../api/types';

interface RitualTarget {
  action: ActionView;
  lessonId: string | null;
  lessonTitle: string | null;
}

function createRitualStore() {
  let target = $state<RitualTarget | null>(null);

  function open(action: ActionView, lessonId: string | null = null, lessonTitle: string | null = null) {
    target = { action, lessonId, lessonTitle };
  }

  function close() {
    target = null;
  }

  return {
    get target() {
      return target;
    },
    open,
    close,
  };
}

export const ritual = createRitualStore();
