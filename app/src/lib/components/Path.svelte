<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../api/commands';
  import { today } from '../stores/today.svelte';
  import type { PathHistoryView } from '../api/types';

  let view = $state<PathHistoryView | null>(null);
  let rangeDays = $state(90);
  let filterPillarId = $state<string | null>(null);

  async function load() {
    view = await api.getPathHistory(rangeDays);
  }

  onMount(load);

  function setRange(days: number) {
    rangeDays = days;
    load();
  }

  const columns = $derived.by(() => {
    if (!view) return [];
    const days = view.days;
    const cols: typeof days[] = [];
    for (let i = 0; i < days.length; i += 7) {
      cols.push(days.slice(i, i + 7));
    }
    return cols;
  });

  const milestonesByDate = $derived.by(() => {
    const map = new Map<string, string[]>();
    if (!view) return map;
    for (const m of view.milestones) {
      const list = map.get(m.date) ?? [];
      list.push(m.kind);
      map.set(m.date, list);
    }
    return map;
  });

  function pillarColor(id: string): string {
    return today.view?.pillars.find((p) => p.id === id)?.color_token ?? '#888';
  }

  function stoneStyle(colors: string[]): string {
    if (colors.length === 0) return '';
    if (colors.length === 1) return `background:${colors[0]}`;
    return `background: linear-gradient(135deg, ${colors[0]} 0 50%, ${colors[1]} 50% 100%)`;
  }

  function isDimmed(colors: string[]): boolean {
    if (!filterPillarId) return false;
    const color = pillarColor(filterPillarId);
    return !colors.includes(color);
  }

  const recentMilestones = $derived(
    view ? [...view.milestones].sort((a, b) => (a.date < b.date ? 1 : -1)) : [],
  );
</script>

<div class="view">
  <div class="card">
    <div class="path-header">
      <h3>The Path</h3>
      <div class="filters">
        <button class:on={rangeDays === 30} onclick={() => setRange(30)}>30d</button>
        <button class:on={rangeDays === 90} onclick={() => setRange(90)}>90d</button>
        <button class:on={rangeDays === 365} onclick={() => setRange(365)}>1y</button>
      </div>
    </div>

    {#if today.view}
      <div class="legend path-legend">
        <button
          class="path-legend-item"
          class:active={filterPillarId === null}
          onclick={() => (filterPillarId = null)}
        >
          All
        </button>
        {#each today.view.pillars as pillar (pillar.id)}
          <button
            class="path-legend-item"
            class:active={filterPillarId === pillar.id}
            onclick={() => (filterPillarId = filterPillarId === pillar.id ? null : pillar.id)}
          >
            <span class="swatch" style={`background:${pillar.color_token}`}></span>{pillar.name}
          </button>
        {/each}
      </div>
    {/if}

    {#if !view}
      <p class="empty-state">Walking back through the record&hellip;</p>
    {:else}
      <div class="path-scroll">
        <div class="path-grid">
          {#each columns as col, ci}
            <div class="path-col">
              {#each col as day (day.date)}
                {@const marks = milestonesByDate.get(day.date) ?? []}
                <div
                  class="path-stone"
                  class:path-gap={day.pillar_colors.length === 0}
                  class:path-dim={isDimmed(day.pillar_colors)}
                  class:path-milestone-streak={marks.includes('streak')}
                  class:path-milestone-points={marks.includes('points')}
                  style={stoneStyle(day.pillar_colors)}
                  title={day.date}
                ></div>
              {/each}
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>

  <div class="card">
    <h3>Milestones</h3>
    {#if recentMilestones.length === 0}
      <p class="empty-state">None yet in this range &mdash; the road is still being laid.</p>
    {:else}
      <table class="streak-table">
        <thead>
          <tr>
            <th>Date</th>
            <th>Milestone</th>
          </tr>
        </thead>
        <tbody>
          {#each recentMilestones as m}
            <tr>
              <td>{m.date}</td>
              <td>{m.kind === 'streak' ? '◆' : '★'} {m.label}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</div>

<style>
  .path-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .path-header h3 {
    margin: 0;
  }
  .filters {
    display: flex;
    gap: 6px;
  }
  .filters button {
    background: var(--surface);
    border: 1px solid var(--card-border);
    color: var(--ink-muted);
    border-radius: 999px;
    padding: 5px 12px;
    font-size: 12px;
    cursor: pointer;
  }
  .filters button.on {
    background: var(--ink);
    color: var(--bg);
    border-color: var(--ink);
    font-weight: 600;
  }
  .path-legend {
    margin-bottom: 14px;
  }
  .path-legend-item {
    background: none;
    border: 1px solid transparent;
    border-radius: 999px;
    padding: 4px 10px;
    color: var(--ink-muted);
    font-size: 12px;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
  }
  .path-legend-item.active {
    border-color: var(--card-border);
    color: var(--ink);
    background: var(--surface);
  }
  .path-scroll {
    overflow-x: auto;
    padding-bottom: 6px;
  }
  .path-grid {
    display: flex;
    gap: 4px;
    width: max-content;
  }
  .path-col {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .path-stone {
    width: 16px;
    height: 16px;
    border-radius: 3px;
    transition: opacity 0.15s ease;
  }
  .path-stone.path-gap {
    background: var(--surface);
    border: 1px solid var(--card-border);
  }
  .path-stone.path-dim {
    opacity: 0.15;
  }
  .path-stone.path-milestone-streak {
    outline: 2px solid var(--craft);
    outline-offset: 1px;
  }
  .path-stone.path-milestone-points {
    outline: 2px solid var(--ink);
    outline-offset: 1px;
  }
</style>
