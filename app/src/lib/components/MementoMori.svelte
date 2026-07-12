<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../api/commands';
  import type { MementoMoriView } from '../api/types';

  let view = $state<MementoMoriView | null>(null);
  let birthInput = $state('');

  onMount(async () => {
    view = await api.getMementoMori();
  });

  async function submitBirthDate(e: Event) {
    e.preventDefault();
    if (!birthInput) return;
    view = await api.setBirthDate(birthInput);
  }

  const COLS = 52;
  const cell = 7;
  const gap = 1.6;

  function rows(total: number) {
    return Math.ceil(total / COLS);
  }
</script>

<div class="view mm-view">
  {#if !view}
    <p class="empty-state">&hellip;</p>
  {:else if view.weeks_lived == null}
    <div class="mm-setup">
      <h2>Memento Mori</h2>
      <p class="mm-intro">
        A life, in weeks. Not to alarm — to orient. Set the date you were born, once, and the
        grid becomes a quiet, honest map of where you stand.
      </p>
      <form onsubmit={submitBirthDate}>
        <input class="task-input" type="date" bind:value={birthInput} />
        <button class="btn primary" type="submit">Begin</button>
      </form>
    </div>
  {:else}
    {@const total = view.weeks_total}
    {@const lived = Math.min(view.weeks_lived, total)}
    {@const w = COLS * (cell + gap)}
    {@const h = rows(total) * (cell + gap)}
    <div class="mm-content">
      <h2>Memento Mori</h2>
      <p class="mm-stat-line">
        <b>{lived.toLocaleString()}</b> weeks lived &middot; <b>{(total - lived).toLocaleString()}</b>
        remain, if eighty years are given.
      </p>
      <svg viewBox={`0 0 ${w} ${h}`} width={w} height={h} class="mm-grid" role="img" aria-label="Weeks of a life">
        {#each { length: total } as _, i}
          {@const col = i % COLS}
          {@const row = Math.floor(i / COLS)}
          {@const x = col * (cell + gap)}
          {@const y = row * (cell + gap)}
          {@const isCurrent = i === lived}
          <rect
            {x}
            {y}
            width={cell}
            height={cell}
            rx="1"
            fill={i < lived ? 'var(--craft)' : 'none'}
            stroke={i < lived ? 'none' : 'var(--card-border)'}
            stroke-width={isCurrent ? 1.5 : 0.75}
            style={isCurrent ? 'stroke:var(--ink); stroke-width:1.5' : ''}
          />
        {/each}
      </svg>
      <blockquote class="mm-quote">
        &ldquo;You could leave life right now. Let that determine what you do and say and
        think.&rdquo;
        <cite>Marcus Aurelius &middot; Meditations, II.11</cite>
      </blockquote>
    </div>
  {/if}
</div>

<style>
  .mm-view {
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .mm-setup,
  .mm-content {
    max-width: 560px;
    text-align: center;
  }
  .mm-setup h2,
  .mm-content h2 {
    font-size: 15px;
    text-transform: uppercase;
    letter-spacing: 0.14em;
    color: var(--ink-muted);
    margin: 0 0 14px;
    font-weight: 600;
  }
  .mm-intro {
    color: var(--ink-muted);
    font-size: 14px;
    line-height: 1.6;
    margin-bottom: 20px;
  }
  .mm-setup form {
    display: flex;
    gap: 10px;
    justify-content: center;
  }
  .mm-stat-line {
    color: var(--ink-muted);
    font-size: 13px;
    margin-bottom: 20px;
  }
  .mm-stat-line b {
    color: var(--ink);
    font-variant-numeric: tabular-nums;
  }
  .mm-grid {
    display: block;
    margin: 0 auto 26px;
  }
  .mm-quote {
    margin: 0 auto;
    font-family: Georgia, serif;
    font-style: italic;
    font-size: 15px;
    line-height: 1.6;
    color: var(--ink-muted);
    max-width: 34em;
  }
  .mm-quote cite {
    display: block;
    margin-top: 10px;
    font-style: normal;
    font-size: 11px;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--ink-faint);
  }
</style>
