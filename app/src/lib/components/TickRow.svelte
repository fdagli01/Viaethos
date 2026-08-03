<script lang="ts">
  import type { ActionView } from '../api/types';
  import { api } from '../api/commands';

  let { action, color }: { action: ActionView; color: string } = $props();

  const done = $derived(action.today_count >= action.target_per_day);

  async function toggle() {
    if (done) return;
    await api.completeTick(action.id);
  }
</script>

<div class="action-row" class:done>
  <button
    class="tick-circle"
    class:filled={done}
    style={done ? `background:${color}` : ''}
    onclick={toggle}
    aria-label={`${action.name} tamamlandı`}
  >
    {#if done}&#10003;{/if}
  </button>
  <span class="action-name">{action.name}</span>
  {#if action.target_per_day > 1}
    <span class="action-meta">{action.today_count}/{action.target_per_day}</span>
  {/if}
  {#if action.streak > 0}
    <span class="action-meta">{action.streak}g seri</span>
  {/if}
</div>
