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

<div class="card mm-card">
  <h3>Memento Mori</h3>
  {#if !view}
    <p class="empty-state">&hellip;</p>
  {:else if view.weeks_lived == null}
    <p class="mm-intro">
      Bir ömür, haftalar hâlinde. Korkutmak için değil &mdash; yönünü bulman için. Doğum tarihini
      bir kez gir, ızgara nerede durduğunun sakin ve dürüst bir haritasına dönüşsün.
    </p>
    <form class="mm-form" onsubmit={submitBirthDate}>
      <input class="task-input" type="date" bind:value={birthInput} />
      <button class="btn primary" type="submit">Başla</button>
    </form>
  {:else}
    {@const total = view.weeks_total}
    {@const lived = Math.min(view.weeks_lived, total)}
    {@const w = COLS * (cell + gap)}
    {@const h = rows(total) * (cell + gap)}
    <p class="mm-stat-line">
      <b>{lived.toLocaleString()}</b> hafta yaşandı &middot;
      <b>{(total - lived).toLocaleString()}</b> hafta kaldı, seksen yıl verilirse.
    </p>
    <svg
      viewBox={`0 0 ${w} ${h}`}
      width={w}
      height={h}
      class="mm-grid"
      role="img"
      aria-label="Bir ömrün haftaları"
    >
      {#each { length: total } as _, i}
        {@const col = i % COLS}
        {@const row = Math.floor(i / COLS)}
        {@const isCurrent = i === lived}
        <rect
          x={col * (cell + gap)}
          y={row * (cell + gap)}
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
      &ldquo;Şu an hayattan ayrılabilirsin. Ne yaptığını, ne söylediğini ve ne düşündüğünü bu
      belirlesin.&rdquo;
      <cite>Marcus Aurelius &middot; Meditations, II.11</cite>
    </blockquote>
  {/if}
</div>

<style>
  .mm-card {
    text-align: center;
  }
  .mm-intro {
    color: var(--ink-muted);
    font-size: 14px;
    line-height: 1.6;
    margin: 0 auto 20px;
    max-width: 40em;
  }
  .mm-form {
    display: flex;
    gap: 10px;
    justify-content: center;
  }
  .mm-stat-line {
    color: var(--ink-muted);
    font-size: 13px;
    margin: 0 0 20px;
  }
  .mm-stat-line b {
    color: var(--ink);
    font-variant-numeric: tabular-nums;
  }
  .mm-grid {
    display: block;
    margin: 0 auto 26px;
    max-width: 100%;
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
