<script lang="ts">
  import { tasks } from '../stores/tasks.svelte';
  import { today } from '../stores/today.svelte';

  let title = $state('');
  let pillarId = $state<string>('');
  let dueToday = $state(false);

  function todayStr() {
    const d = new Date();
    return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
  }

  async function submit(e: Event) {
    e.preventDefault();
    const t = title.trim();
    if (!t) return;
    await tasks.add(t, pillarId || null, dueToday ? todayStr() : null);
    title = '';
    dueToday = false;
  }

  function pillarColor(id: string | null): string {
    if (!id || !today.view) return 'var(--ink-faint)';
    return today.view.pillars.find((p) => p.id === id)?.color_token ?? 'var(--ink-faint)';
  }

  function isOverdue(dueOn: string | null): boolean {
    if (!dueOn) return false;
    return dueOn < todayStr();
  }
</script>

<section class="pillar-section">
  <div class="pillar-header">
    <h2>The Path Ahead</h2>
    <span class="action-meta" style="margin-left:auto">{tasks.list.length} open</span>
  </div>

  <form class="task-add" onsubmit={submit}>
    <input class="task-input" placeholder="Add to the path…" bind:value={title} />
    {#if today.view}
      <select class="task-select" bind:value={pillarId}>
        <option value="">No pillar</option>
        {#each today.view.pillars as pillar (pillar.id)}
          <option value={pillar.id}>{pillar.name}</option>
        {/each}
      </select>
    {/if}
    <label class="task-today-toggle">
      <input type="checkbox" bind:checked={dueToday} /> today
    </label>
    <button class="btn primary" type="submit">Add</button>
  </form>

  {#if tasks.list.length === 0}
    <p class="empty-state">The path ahead is clear.</p>
  {:else}
    {#each tasks.list as task (task.id)}
      <div class="action-row">
        <button class="tick-circle" onclick={() => tasks.complete(task.id)} aria-label={`Complete ${task.title}`}
        ></button>
        <span class="pillar-dot" style={`background:${pillarColor(task.pillar_id)}`}></span>
        <span class="action-name">{task.title}</span>
        {#if task.due_on}
          <span class="action-meta" class:overdue={isOverdue(task.due_on)}>{task.due_on}</span>
        {/if}
        <button class="focus-btn" onclick={() => tasks.remove(task.id)}>Remove</button>
      </div>
    {/each}
  {/if}
</section>

<style>
  .task-add {
    display: flex;
    gap: 8px;
    align-items: center;
    padding: 12px 18px;
    border-bottom: 1px solid var(--card-border);
    flex-wrap: wrap;
  }
  .task-input {
    flex: 1;
    min-width: 160px;
    background: var(--surface);
    border: 1px solid var(--card-border);
    border-radius: var(--radius-sm);
    color: var(--ink);
    padding: 8px 12px;
    font-size: 13.5px;
  }
  .task-select {
    background: var(--surface);
    border: 1px solid var(--card-border);
    border-radius: var(--radius-sm);
    color: var(--ink);
    padding: 8px 10px;
    font-size: 13px;
  }
  .task-today-toggle {
    font-size: 12.5px;
    color: var(--ink-muted);
    display: flex;
    align-items: center;
    gap: 4px;
  }
  .overdue {
    color: var(--body);
  }
</style>
