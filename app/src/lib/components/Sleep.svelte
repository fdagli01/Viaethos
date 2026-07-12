<script lang="ts">
  import { onMount } from 'svelte';
  import { sleep } from '../stores/sleep.svelte';

  let bedAt = $state('23:30');
  let wokeAt = $state('07:00');
  let quality = $state(3);

  onMount(() => {
    sleep.init();
  });

  function hoursBetween(bed: string, woke: string): number {
    const [bh, bm] = bed.split(':').map(Number);
    const [wh, wm] = woke.split(':').map(Number);
    let bedMin = bh * 60 + bm;
    let wokeMin = wh * 60 + wm;
    if (wokeMin <= bedMin) wokeMin += 24 * 60;
    return (wokeMin - bedMin) / 60;
  }

  const previewHours = $derived(hoursBetween(bedAt, wokeAt));

  async function submit(e: Event) {
    e.preventDefault();
    const today = new Date().toISOString().slice(0, 10);
    await sleep.log(today, bedAt, wokeAt, quality);
  }
</script>

<section class="pillar-section">
  <div class="pillar-header">
    <h2>Uyku</h2>
    {#if sleep.last}
      <span class="action-meta" style="margin-left:auto"
        >last: {hoursBetween(sleep.last.bed_at, sleep.last.woke_at).toFixed(1)}h &middot; {'★'.repeat(
          sleep.last.quality_1_5,
        )}{'☆'.repeat(5 - sleep.last.quality_1_5)}</span
      >
    {/if}
  </div>

  <form class="sleep-form" onsubmit={submit}>
    <label class="task-today-toggle">Bed <input class="sleep-time" type="time" bind:value={bedAt} /></label>
    <label class="task-today-toggle">Woke <input class="sleep-time" type="time" bind:value={wokeAt} /></label>
    <span class="action-meta">{previewHours.toFixed(1)}h</span>
    <div class="sleep-stars">
      {#each [1, 2, 3, 4, 5] as n}
        <button
          type="button"
          class="sleep-star"
          class:filled={n <= quality}
          onclick={() => (quality = n)}
          aria-label={`Quality ${n}`}>★</button
        >
      {/each}
    </div>
    <button class="btn primary" type="submit">Log last night</button>
  </form>
</section>

<style>
  .sleep-form {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 14px 18px;
    flex-wrap: wrap;
  }
  .sleep-time {
    background: var(--surface);
    border: 1px solid var(--card-border);
    border-radius: var(--radius-sm);
    color: var(--ink);
    padding: 6px 10px;
    font-size: 13px;
  }
  .sleep-stars {
    display: flex;
    gap: 2px;
  }
  .sleep-star {
    background: none;
    border: none;
    font-size: 18px;
    color: var(--ink-faint);
    cursor: pointer;
    line-height: 1;
  }
  .sleep-star.filled {
    color: var(--craft);
  }
</style>
