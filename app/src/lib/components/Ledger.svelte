<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../api/commands';
  import type { LedgerStats } from '../api/types';

  let stats = $state<LedgerStats | null>(null);
  let hovered = $state<number | null>(null);

  onMount(async () => {
    stats = await api.getLedgerStats();
  });

  const pillarColors: Record<string, string> = {
    Mind: 'var(--mind)',
    Body: 'var(--body)',
    Craft: 'var(--craft)',
    Life: 'var(--life)',
  };

  const weeklyFocusMinutes = $derived(
    stats
      ? stats.focus_by_pillar_day.reduce(
          (sum, day) => sum + day.by_pillar.reduce((s, [, m]) => s + m, 0),
          0,
        )
      : 0,
  );

  const bestStreak = $derived(
    stats ? stats.streaks.reduce((m, s) => Math.max(m, s.best), 0) : 0,
  );

  const maxFocusMinutes = $derived(
    stats
      ? Math.max(
          1,
          ...stats.focus_by_pillar_day.map((d) => d.by_pillar.reduce((s, [, m]) => s + m, 0)),
        )
      : 1,
  );

  const maxPoints = $derived(
    stats ? Math.max(1, ...stats.points_trend.map((p) => p.points)) : 1,
  );

  function linePath(): string {
    if (!stats) return '';
    const w = 600;
    const h = 120;
    const n = stats.points_trend.length;
    return stats.points_trend
      .map((p, i) => {
        const x = (i / (n - 1)) * w;
        const y = h - (p.points / maxPoints) * (h - 10) - 5;
        return `${i === 0 ? 'M' : 'L'}${x.toFixed(1)},${y.toFixed(1)}`;
      })
      .join(' ');
  }
</script>

<div class="view">
  {#if !stats}
    <p class="empty-state">Reading the ledger&hellip;</p>
  {:else}
    <div class="ledger-tiles">
      <div class="tile">
        <div class="label">Ethos Points</div>
        <div class="value">{stats.total_points.toLocaleString()}</div>
      </div>
      <div class="tile">
        <div class="label">Best Streak</div>
        <div class="value">{bestStreak}d</div>
      </div>
      <div class="tile">
        <div class="label">Focus, last 7d</div>
        <div class="value">{(weeklyFocusMinutes / 60).toFixed(1)}h</div>
      </div>
      <div class="tile">
        <div class="label">Active Habits</div>
        <div class="value">{stats.streaks.length}</div>
      </div>
    </div>

    <div class="card">
      <h3>Focus Hours by Pillar &middot; Last 7 Days</h3>
      <div class="legend">
        {#each Object.entries(pillarColors) as [name, color]}
          <span><span class="swatch" style={`background:${color}`}></span>{name}</span>
        {/each}
      </div>
      <svg viewBox="0 0 600 140" width="100%" height="140" role="img" aria-label="Focus hours by pillar per day">
        {#each stats.focus_by_pillar_day as day, i}
          {@const barW = 600 / stats.focus_by_pillar_day.length - 8}
          {@const x = i * (600 / stats.focus_by_pillar_day.length) + 4}
          {#each day.by_pillar as [pillarName, minutes], pi}
            {@const totalBefore = day.by_pillar.slice(0, pi).reduce((s, [, m]) => s + m, 0)}
            {@const barH = (minutes / maxFocusMinutes) * 100}
            {@const yBefore = (totalBefore / maxFocusMinutes) * 100}
            {#if minutes > 0}
              <rect
                x={x}
                y={120 - yBefore - barH}
                width={barW}
                height={Math.max(barH - 2, 1.5)}
                fill={pillarColors[pillarName] ?? '#888'}
                rx="2"
              />
            {/if}
          {/each}
          <text x={x + barW / 2} y="134" font-size="9" fill="var(--ink-faint)" text-anchor="middle">
            {day.date.slice(5)}
          </text>
        {/each}
      </svg>
    </div>

    <div class="card">
      <h3>Ethos Points &middot; Last 30 Days</h3>
      <svg
        viewBox="0 0 600 120"
        width="100%"
        height="120"
        role="img"
        aria-label="Ethos points over 30 days"
        onmousemove={(e) => {
          const rect = (e.target as SVGElement).closest('svg')!.getBoundingClientRect();
          const ratio = (e.clientX - rect.left) / rect.width;
          hovered = Math.round(ratio * (stats!.points_trend.length - 1));
        }}
        onmouseleave={() => (hovered = null)}
      >
        <path d={linePath()} fill="none" stroke={pillarColors.Mind} stroke-width="2" />
        {#if hovered !== null && stats.points_trend[hovered]}
          {@const n = stats.points_trend.length}
          {@const x = (hovered / (n - 1)) * 600}
          {@const y = 120 - (stats.points_trend[hovered].points / maxPoints) * 110 - 5}
          <line x1={x} y1="0" x2={x} y2="120" stroke="var(--card-border)" stroke-width="1" />
          <circle cx={x} cy={y} r="4" fill={pillarColors.Mind} />
          <text x={Math.min(x + 8, 540)} y={Math.max(y, 12)} font-size="11" fill="var(--ink)">
            {stats.points_trend[hovered].date.slice(5)} &middot; {stats.points_trend[hovered].points}pt
          </text>
        {/if}
      </svg>
    </div>

    <div class="card">
      <h3>Streaks</h3>
      {#if stats.streaks.length === 0}
        <p class="empty-state">No actions yet.</p>
      {:else}
        <table class="streak-table">
          <thead>
            <tr>
              <th>Action</th>
              <th>Pillar</th>
              <th>Kind</th>
              <th>Current</th>
              <th>Best</th>
            </tr>
          </thead>
          <tbody>
            {#each stats.streaks as row}
              <tr>
                <td>{row.action_name}</td>
                <td>
                  <span class="pillar-tag">
                    <span class="swatch" style={`background:${row.color_token}`}></span>
                    {row.pillar_name}
                  </span>
                </td>
                <td>{row.kind}</td>
                <td>{row.current}</td>
                <td>{row.best}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  {/if}
</div>
