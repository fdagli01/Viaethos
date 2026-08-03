<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../api/commands';
  import type { LedgerStats } from '../api/types';

  let stats = $state<LedgerStats | null>(null);

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

  const bestStreak = $derived(stats ? stats.streaks.reduce((m, s) => Math.max(m, s.best), 0) : 0);

  const maxFocusMinutes = $derived(
    stats
      ? Math.max(
          1,
          ...stats.focus_by_pillar_day.map((d) => d.by_pillar.reduce((s, [, m]) => s + m, 0)),
        )
      : 1,
  );

  const SLEEP_TARGET_HOURS = 8;
  const maxSleepHours = $derived(
    stats ? Math.max(SLEEP_TARGET_HOURS + 1, ...stats.sleep_by_day.map((d) => d.hours)) : 9,
  );
  const sleepRefY = $derived(120 - (SLEEP_TARGET_HOURS / maxSleepHours) * 110);
</script>

{#if !stats}
  <p class="empty-state">Kayıt okunuyor&hellip;</p>
{:else}
  <div class="ledger-tiles">
    <div class="tile">
      <div class="label">Ethos Puanı</div>
      <div class="value">{stats.total_points.toLocaleString()}</div>
    </div>
    <div class="tile">
      <div class="label">En uzun seri</div>
      <div class="value">{bestStreak}g</div>
    </div>
    <div class="tile">
      <div class="label">Odak, son 7 gün</div>
      <div class="value">{(weeklyFocusMinutes / 60).toFixed(1)}s</div>
    </div>
    <div class="tile">
      <div class="label">Aktif alışkanlık</div>
      <div class="value">{stats.streaks.length}</div>
    </div>
  </div>

  <div class="card">
    <h3>Odak saatleri &middot; son 7 gün</h3>
    <div class="legend">
      {#each Object.entries(pillarColors) as [name, color] (name)}
        <span><span class="swatch" style={`background:${color}`}></span>{name}</span>
      {/each}
    </div>
    <svg
      viewBox="0 0 600 140"
      width="100%"
      height="140"
      role="img"
      aria-label="Sütun başına günlük odak süresi"
    >
      {#each stats.focus_by_pillar_day as day, i (day.date)}
        {@const barW = 600 / stats.focus_by_pillar_day.length - 8}
        {@const x = i * (600 / stats.focus_by_pillar_day.length) + 4}
        {#each day.by_pillar as [pillarName, minutes], pi (pillarName)}
          {@const totalBefore = day.by_pillar.slice(0, pi).reduce((s, [, m]) => s + m, 0)}
          {@const barH = (minutes / maxFocusMinutes) * 100}
          {@const yBefore = (totalBefore / maxFocusMinutes) * 100}
          {#if minutes > 0}
            <rect
              {x}
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
    <h3>Uyku &middot; son 7 gün</h3>
    <svg
      viewBox="0 0 600 150"
      width="100%"
      height="150"
      role="img"
      aria-label="Uyku saatleri ve 8 saat hedefi"
    >
      <line
        x1="0"
        y1={sleepRefY}
        x2="600"
        y2={sleepRefY}
        stroke="var(--ink-faint)"
        stroke-dasharray="4 3"
        stroke-width="1"
      />
      <text x="596" y={sleepRefY - 4} font-size="9" fill="var(--ink-faint)" text-anchor="end"
        >8s hedef</text
      >
      {#each stats.sleep_by_day as day, i (day.date)}
        {@const barW = 600 / stats.sleep_by_day.length - 8}
        {@const x = i * (600 / stats.sleep_by_day.length) + 4}
        {@const barH = (day.hours / maxSleepHours) * 110}
        {#if day.hours > 0}
          <rect {x} y={120 - barH} width={barW} height={barH} fill="var(--life)" rx="2" />
        {/if}
        <text x={x + barW / 2} y="134" font-size="9" fill="var(--ink-faint)" text-anchor="middle">
          {day.date.slice(5)}
        </text>
        <text x={x + barW / 2} y="146" font-size="10" fill="var(--craft)" text-anchor="middle">
          {day.quality_1_5 ? '★'.repeat(day.quality_1_5) : '—'}
        </text>
      {/each}
    </svg>
  </div>

  <div class="card">
    <h3>Seriler</h3>
    {#if stats.streaks.length === 0}
      <p class="empty-state">Henüz alışkanlık yok.</p>
    {:else}
      <table class="streak-table">
        <thead>
          <tr>
            <th>Alışkanlık</th>
            <th>Sütun</th>
            <th>Şu an</th>
            <th>En iyi</th>
          </tr>
        </thead>
        <tbody>
          {#each stats.streaks as row (row.action_name + row.pillar_name)}
            <tr>
              <td>{row.action_name}</td>
              <td>
                <span class="pillar-tag">
                  <span class="swatch" style={`background:${row.color_token}`}></span>
                  {row.pillar_name}
                </span>
              </td>
              <td>{row.current}g</td>
              <td>{row.best}g</td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
{/if}
