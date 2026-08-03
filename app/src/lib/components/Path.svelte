<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../api/commands';
  import type { PathHistoryView } from '../api/types';

  let view = $state<PathHistoryView | null>(null);
  let rangeDays = $state(90);

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
    const cols: (typeof days)[] = [];
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

  function stoneStyle(colors: string[]): string {
    if (colors.length === 0) return '';
    if (colors.length === 1) return `background:${colors[0]}`;
    return `background: linear-gradient(135deg, ${colors[0]} 0 50%, ${colors[1]} 50% 100%)`;
  }

  const recentMilestones = $derived(
    view ? [...view.milestones].sort((a, b) => (a.date < b.date ? 1 : -1)).slice(0, 6) : [],
  );
</script>

<div class="card">
  <div class="path-header">
    <h3>Yol</h3>
    <div class="chip-row path-ranges">
      <button class="chip" class:on={rangeDays === 30} onclick={() => setRange(30)}>30g</button>
      <button class="chip" class:on={rangeDays === 90} onclick={() => setRange(90)}>90g</button>
      <button class="chip" class:on={rangeDays === 365} onclick={() => setRange(365)}>1y</button>
    </div>
  </div>

  {#if !view}
    <p class="empty-state">Kayıt okunuyor&hellip;</p>
  {:else}
    <div class="path-scroll">
      <div class="path-grid">
        {#each columns as col (col[0].date)}
          <div class="path-col">
            {#each col as day (day.date)}
              {@const marks = milestonesByDate.get(day.date) ?? []}
              <div
                class="path-stone"
                class:path-gap={day.pillar_colors.length === 0}
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

    {#if recentMilestones.length > 0}
      <ul class="path-milestones">
        {#each recentMilestones as m (m.date + m.label)}
          <li>
            <span class="action-meta">{m.date}</span>
            {m.kind === 'streak' ? '◆' : '★'}
            {m.label}
          </li>
        {/each}
      </ul>
    {/if}
  {/if}
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
  .path-ranges {
    padding: 0;
    border-bottom: none;
  }
  .path-scroll {
    overflow-x: auto;
    padding: 14px 0 6px;
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
  }
  .path-stone.path-gap {
    background: var(--surface);
    border: 1px solid var(--card-border);
  }
  .path-stone.path-milestone-streak {
    outline: 2px solid var(--craft);
    outline-offset: 1px;
  }
  .path-stone.path-milestone-points {
    outline: 2px solid var(--ink);
    outline-offset: 1px;
  }
  .path-milestones {
    list-style: none;
    margin: 12px 0 0;
    padding: 0;
    font-size: 13px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .path-milestones .action-meta {
    margin-right: 8px;
  }
</style>
