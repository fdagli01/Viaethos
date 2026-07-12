<script lang="ts">
  import { onMount } from 'svelte';
  import { today } from '../stores/today.svelte';
  import { tasks } from '../stores/tasks.svelte';
  import { meals } from '../stores/meals.svelte';
  import type { ActionView } from '../api/types';
  import PillarSection from './PillarSection.svelte';
  import RitualModal from './RitualModal.svelte';
  import TaskList from './TaskList.svelte';
  import Sofra from './Sofra.svelte';

  let ritualAction = $state<ActionView | null>(null);

  function enterRitual(action: ActionView) {
    ritualAction = action;
  }

  function closeRitual() {
    ritualAction = null;
  }

  onMount(() => {
    tasks.init();
    meals.init();
  });
</script>

<div class="view">
  {#if today.loading}
    <p class="empty-state">Loading today&rsquo;s path&hellip;</p>
  {:else if today.view}
    {#each today.view.pillars as pillar (pillar.id)}
      <PillarSection {pillar} onEnterRitual={enterRitual} />
    {/each}
    <TaskList />
    <Sofra />
  {/if}
</div>

{#if ritualAction}
  <RitualModal action={ritualAction} onClose={closeRitual} />
{/if}
