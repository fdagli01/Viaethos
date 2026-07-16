<script lang="ts">
  import { onMount } from 'svelte';
  import { schedule } from '../stores/schedule.svelte';
  import { today } from '../stores/today.svelte';
  import type { RecurrenceType } from '../api/types';

  let title = $state('');
  let pillarId = $state('');
  let startTime = $state('09:00');
  let endTime = $state('10:00');
  let recurrenceType = $state<RecurrenceType>('once');
  let onceDate = $state(todayStr());

  function todayStr() {
    const d = new Date();
    return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
  }

  onMount(() => {
    schedule.init();
  });

  function pillarColor(id: string | null): string {
    if (!id || !today.view) return 'var(--ink-faint)';
    return today.view.pillars.find((p) => p.id === id)?.color_token ?? 'var(--ink-faint)';
  }

  async function submit(e: Event) {
    e.preventDefault();
    const t = title.trim();
    if (!t) return;
    await schedule.add(
      t,
      pillarId || null,
      startTime,
      endTime,
      recurrenceType,
      null,
      recurrenceType === 'once' ? onceDate : null,
    );
    title = '';
  }
</script>

<section class="pillar-section">
  <div class="pillar-header">
    <h2>Program</h2>
    <button
      class="focus-btn"
      style="margin-left:auto"
      onclick={() => schedule.suggest()}
      disabled={schedule.suggesting}
    >
      {schedule.suggesting ? 'Düşünüyor…' : 'AI ile öner'}
    </button>
  </div>

  <form class="schedule-add" onsubmit={submit}>
    <input class="task-input" placeholder="Ne var? (örn. Kimya dersi)" bind:value={title} />
    {#if today.view}
      <select class="task-select" bind:value={pillarId}>
        <option value="">Sütun yok</option>
        {#each today.view.pillars as pillar (pillar.id)}
          <option value={pillar.id}>{pillar.name}</option>
        {/each}
      </select>
    {/if}
    <input class="schedule-time" type="time" bind:value={startTime} />
    <span class="action-meta">–</span>
    <input class="schedule-time" type="time" bind:value={endTime} />
    <select class="task-select" bind:value={recurrenceType}>
      <option value="once">Tek seferlik</option>
      <option value="daily">Her gün</option>
      <option value="weekdays">Hafta içi</option>
    </select>
    {#if recurrenceType === 'once'}
      <input class="schedule-time" type="date" bind:value={onceDate} />
    {/if}
    <button class="btn primary" type="submit">Ekle</button>
  </form>

  {#if schedule.suggestion || schedule.suggestError}
    <div class="schedule-suggestion">
      {#if schedule.suggestError}
        <p class="schedule-suggestion-error">{schedule.suggestError}</p>
      {:else}
        <p class="schedule-suggestion-text">{schedule.suggestion}</p>
      {/if}
      <button class="focus-btn" onclick={() => schedule.clearSuggestion()}>Kapat</button>
    </div>
  {/if}

  {#if schedule.blocks.length === 0}
    <p class="empty-state">Bugün için program bloğu yok.</p>
  {:else}
    {#each schedule.blocks as block (block.id)}
      <div class="action-row" class:schedule-now={block.is_now}>
        <span class="pillar-dot" style={`background:${pillarColor(block.pillar_id)}`}></span>
        <span class="action-meta schedule-block-time">{block.start_time}–{block.end_time}</span>
        <span class="action-name">{block.title}</span>
        {#if block.is_now}
          <span class="action-meta schedule-now-label">şimdi</span>
        {/if}
        <button class="focus-btn" onclick={() => schedule.remove(block.id)}>Kaldır</button>
      </div>
    {/each}
  {/if}
</section>

<style>
  .schedule-add {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 18px;
    border-bottom: 1px solid var(--card-border);
    flex-wrap: wrap;
  }
  .schedule-time {
    background: var(--surface);
    border: 1px solid var(--card-border);
    border-radius: var(--radius-sm);
    color: var(--ink);
    padding: 8px 10px;
    font-size: 13px;
  }
  .schedule-block-time {
    min-width: 96px;
  }
  .schedule-now {
    background: var(--surface);
  }
  .schedule-now-label {
    text-transform: uppercase;
    font-size: 10.5px;
    letter-spacing: 0.06em;
    color: var(--craft, var(--ink-muted));
  }
  .schedule-suggestion {
    margin: 12px 18px;
    padding: 12px 14px;
    background: var(--surface);
    border: 1px solid var(--card-border);
    border-radius: var(--radius-sm);
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .schedule-suggestion-text {
    white-space: pre-wrap;
    font-size: 13.5px;
    line-height: 1.6;
    margin: 0;
  }
  .schedule-suggestion-error {
    color: var(--body);
    font-size: 13px;
    margin: 0;
  }
</style>
