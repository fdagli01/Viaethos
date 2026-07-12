<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../api/commands';
  import { today } from '../stores/today.svelte';
  import type { ActionAdminView } from '../api/types';

  let actions = $state<ActionAdminView[]>([]);
  let savedPillar = $state<string | null>(null);
  let savedAction = $state<string | null>(null);

  let newName = $state('');
  let newPillarId = $state('');
  let newKind = $state<'focus' | 'tick'>('tick');
  let newMinutes = $state(25);
  let newSchedule = $state<'daily' | 'weekdays' | 'times_per_week'>('daily');
  let newTimesPerWeek = $state(3);
  let newTarget = $state(1);

  onMount(async () => {
    actions = await api.getManageActions();
  });

  async function savePillar(pillarId: string, name: string, colorToken: string) {
    await api.updatePillar(pillarId, name, colorToken);
    await today.refresh();
    savedPillar = pillarId;
    setTimeout(() => (savedPillar = null), 1500);
  }

  async function saveAction(a: ActionAdminView) {
    actions = await api.updateAction(
      a.id,
      a.name,
      a.kind === 'focus' ? a.default_minutes : null,
      a.schedule_type,
      a.schedule_type === 'times_per_week' ? a.times_per_week : null,
      a.target_per_day,
    );
    savedAction = a.id;
    setTimeout(() => (savedAction = null), 1500);
  }

  async function toggleArchived(a: ActionAdminView) {
    actions = await api.setActionArchived(a.id, !a.archived);
  }

  async function submitNewAction(e: Event) {
    e.preventDefault();
    if (!newName.trim() || !newPillarId) return;
    actions = await api.addAction(
      newPillarId,
      newName.trim(),
      newKind,
      newKind === 'focus' ? newMinutes : null,
      newSchedule,
      newSchedule === 'times_per_week' ? newTimesPerWeek : null,
      newTarget,
    );
    newName = '';
  }

  const actionsByPillar = $derived(
    today.view
      ? today.view.pillars.map((p) => ({
          pillar: p,
          actions: actions.filter((a) => a.pillar_id === p.id),
        }))
      : [],
  );
</script>

<div class="view">
  <div class="card">
    <h3>Pillars</h3>
    {#if today.view}
      {#each today.view.pillars as pillar (pillar.id)}
        {@const name = pillar.name}
        {@const color = pillar.color_token}
        <div class="manage-row">
          <input
            class="task-input manage-name"
            value={name}
            onchange={(e) => savePillar(pillar.id, (e.target as HTMLInputElement).value, color)}
          />
          <input
            type="color"
            value={color}
            onchange={(e) => savePillar(pillar.id, name, (e.target as HTMLInputElement).value)}
          />
          {#if savedPillar === pillar.id}<span class="action-meta">Saved.</span>{/if}
        </div>
      {/each}
    {/if}
  </div>

  {#each actionsByPillar as group (group.pillar.id)}
    <div class="card">
      <h3>{group.pillar.name} Actions</h3>
      {#if group.actions.length === 0}
        <p class="empty-state">No actions in this pillar.</p>
      {:else}
        {#each group.actions as a (a.id)}
          <div class="manage-row" class:manage-archived={a.archived}>
            <input class="task-input manage-name" bind:value={a.name} />
            <span class="action-meta">{a.kind}</span>
            <select class="task-select" bind:value={a.schedule_type}>
              <option value="daily">daily</option>
              <option value="weekdays">weekdays</option>
              <option value="times_per_week">times/week</option>
            </select>
            {#if a.schedule_type === 'times_per_week'}
              <input class="sofra-grams" type="number" min="1" max="7" bind:value={a.times_per_week} />
            {/if}
            {#if a.kind === 'focus'}
              <label class="task-today-toggle"
                >min <input class="sofra-grams" type="number" min="5" bind:value={a.default_minutes} /></label
              >
            {:else}
              <label class="task-today-toggle"
                >&times;/day <input class="sofra-grams" type="number" min="1" bind:value={a.target_per_day} /></label
              >
            {/if}
            <button class="focus-btn" onclick={() => saveAction(a)}>Save</button>
            <button class="focus-btn" onclick={() => toggleArchived(a)}
              >{a.archived ? 'Restore' : 'Archive'}</button
            >
            {#if savedAction === a.id}<span class="action-meta">Saved.</span>{/if}
          </div>
        {/each}
      {/if}
    </div>
  {/each}

  <div class="card">
    <h3>Add Action</h3>
    <form class="manage-row" onsubmit={submitNewAction}>
      <input class="task-input manage-name" placeholder="Name" bind:value={newName} />
      {#if today.view}
        <select class="task-select" bind:value={newPillarId}>
          <option value="">Pillar…</option>
          {#each today.view.pillars as pillar (pillar.id)}
            <option value={pillar.id}>{pillar.name}</option>
          {/each}
        </select>
      {/if}
      <select class="task-select" bind:value={newKind}>
        <option value="tick">tick</option>
        <option value="focus">focus</option>
      </select>
      <select class="task-select" bind:value={newSchedule}>
        <option value="daily">daily</option>
        <option value="weekdays">weekdays</option>
        <option value="times_per_week">times/week</option>
      </select>
      {#if newSchedule === 'times_per_week'}
        <input class="sofra-grams" type="number" min="1" max="7" bind:value={newTimesPerWeek} />
      {/if}
      {#if newKind === 'focus'}
        <label class="task-today-toggle"
          >min <input class="sofra-grams" type="number" min="5" bind:value={newMinutes} /></label
        >
      {:else}
        <label class="task-today-toggle"
          >&times;/day <input class="sofra-grams" type="number" min="1" bind:value={newTarget} /></label
        >
      {/if}
      <button class="btn primary" type="submit">Add</button>
    </form>
  </div>
</div>

<style>
  .manage-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 0;
    border-bottom: 1px solid var(--card-border);
    flex-wrap: wrap;
  }
  .manage-row:last-child {
    border-bottom: none;
  }
  .manage-name {
    max-width: 220px;
  }
  .manage-archived {
    opacity: 0.5;
  }
</style>
