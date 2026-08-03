<script lang="ts">
  import { onMount } from 'svelte';
  import { today } from '../stores/today.svelte';
  import { tasks } from '../stores/tasks.svelte';
  import { meals } from '../stores/meals.svelte';
  import { ritual } from '../stores/ritual.svelte';
  import type { ActionView } from '../api/types';
  import PillarSection from './PillarSection.svelte';
  import RitualModal from './RitualModal.svelte';
  import Schedule from './Schedule.svelte';
  import TaskList from './TaskList.svelte';
  import Sofra from './Sofra.svelte';
  import Mufredat from './Mufredat.svelte';
  import Sleep from './Sleep.svelte';

  function enterRitual(action: ActionView) {
    ritual.open(action);
  }

  onMount(() => {
    tasks.init();
    meals.init();
  });
</script>

<div class="view">
  {#if today.loading}
    <p class="empty-state">Bugün yükleniyor&hellip;</p>
  {:else if today.view}
    <Schedule />
    {#each today.view.pillars as pillar (pillar.id)}
      <PillarSection {pillar} onEnterRitual={enterRitual} />
    {/each}
    <TaskList />
    <Sofra />
    <Mufredat />
    <Sleep />
  {/if}
</div>

{#if ritual.target}
  <RitualModal
    action={ritual.target.action}
    lessonId={ritual.target.lessonId}
    lessonTitle={ritual.target.lessonTitle}
    onClose={ritual.close}
  />
{/if}
