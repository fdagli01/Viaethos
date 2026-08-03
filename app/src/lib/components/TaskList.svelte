<script lang="ts">
  import { tasks } from '../stores/tasks.svelte';
  import { today } from '../stores/today.svelte';

  let adding = $state(false);
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
    <h2>Yapılacaklar</h2>
    <span class="action-meta" style="margin-left:auto">{tasks.list.length} açık</span>
    <button
      class="add-toggle"
      class:open={adding}
      style="margin-left:0"
      onclick={() => (adding = !adding)}
      aria-label="Görev ekle">+</button
    >
  </div>

  {#if adding}
    <form class="add-form" onsubmit={submit}>
      <input class="task-input" placeholder="Ne yapılacak?" bind:value={title} />
      {#if today.view}
        <select class="task-select" bind:value={pillarId} aria-label="Sütun">
          <option value="">Sütun yok</option>
          {#each today.view.pillars as pillar (pillar.id)}
            <option value={pillar.id}>{pillar.name}</option>
          {/each}
        </select>
      {/if}
      <label class="task-today-toggle">
        <input type="checkbox" bind:checked={dueToday} /> bugün
      </label>
      <button class="btn primary" type="submit">Ekle</button>
    </form>
  {/if}

  {#if tasks.list.length === 0}
    <p class="empty-state">Önün açık.</p>
  {:else}
    {#each tasks.list as task (task.id)}
      <div class="action-row">
        <button
          class="tick-circle"
          onclick={() => tasks.complete(task.id)}
          aria-label={`${task.title} tamamlandı`}
        ></button>
        <span class="pillar-dot" style={`background:${pillarColor(task.pillar_id)}`}></span>
        <span class="action-name">{task.title}</span>
        {#if task.due_on}
          <span class="action-meta" class:overdue={isOverdue(task.due_on)}>{task.due_on}</span>
        {/if}
        <button class="focus-btn" onclick={() => tasks.remove(task.id)}>Sil</button>
      </div>
    {/each}
  {/if}
</section>
