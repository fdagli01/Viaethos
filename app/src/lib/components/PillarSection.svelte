<script lang="ts">
  import type { PillarView, ActionView } from '../api/types';
  import TickRow from './TickRow.svelte';
  import FocusRow from './FocusRow.svelte';

  let { pillar, onEnterRitual }: { pillar: PillarView; onEnterRitual: (a: ActionView) => void } =
    $props();

  const dueActions = $derived(pillar.actions.filter((a) => a.due_today));
  const honoredCount = $derived(
    dueActions.filter((a) => a.today_count >= a.target_per_day).length,
  );
  const progressPct = $derived(
    dueActions.length === 0 ? 100 : Math.round((honoredCount / dueActions.length) * 100),
  );
</script>

<section class="pillar-section">
  <div class="pillar-header">
    <span class="pillar-dot" style={`background:${pillar.color_token}`}></span>
    <h2>{pillar.name}</h2>
    <div class="pillar-hairline">
      <div style={`width:${progressPct}%;background:${pillar.color_token}`}></div>
    </div>
    <span class="action-meta">{honoredCount}/{dueActions.length}</span>
  </div>

  {#each pillar.actions as action (action.id)}
    {#if action.kind === 'tick'}
      <TickRow {action} color={pillar.color_token} />
    {:else}
      <FocusRow {action} color={pillar.color_token} onEnter={onEnterRitual} />
    {/if}
  {/each}
</section>
