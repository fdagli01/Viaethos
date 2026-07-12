<script lang="ts">
  import type { ActionView } from '../api/types';

  let {
    action,
    color,
    onEnter,
  }: { action: ActionView; color: string; onEnter: (action: ActionView) => void } = $props();

  const doneToday = $derived(action.today_count >= action.target_per_day);
  const active = $derived(!!action.active_session);
</script>

<div class="action-row" class:done={doneToday && !active}>
  <button
    class="tick-circle"
    class:filled={doneToday && !active}
    style={doneToday && !active ? `background:${color}` : `border-color:${active ? color : ''}`}
    onclick={() => onEnter(action)}
    aria-label={`Enter ritual for ${action.name}`}
  >
    {#if doneToday && !active}&#10003;{/if}
  </button>
  <span class="action-name">{action.name}</span>
  {#if action.streak > 0}
    <span class="action-meta">{action.streak}d streak</span>
  {/if}
  <button class="focus-btn" class:active onclick={() => onEnter(action)}>
    {active ? 'Resume' : doneToday ? 'Again' : 'Begin'}
  </button>
</div>
