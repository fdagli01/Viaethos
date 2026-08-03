<script lang="ts">
  import { meals } from '../stores/meals.svelte';
  import { api } from '../api/commands';
  import type { FoodItem, MealPreset, TimeSlot } from '../api/types';

  const slots: [TimeSlot, string][] = [
    ['breakfast', 'Kahvaltı'],
    ['lunch', 'Öğle'],
    ['dinner', 'Akşam'],
    ['snack', 'Ara'],
  ];

  const slotLabels: Record<TimeSlot, string> = Object.fromEntries(slots) as Record<
    TimeSlot,
    string
  >;

  // The slot is read off the clock rather than asked for — at 08:00 it is
  // breakfast, and a one-tap repeat should not stop to confirm that.
  function currentSlot(): TimeSlot {
    const h = new Date().getHours();
    if (h < 11) return 'breakfast';
    if (h < 16) return 'lunch';
    if (h < 21) return 'dinner';
    return 'snack';
  }

  let adding = $state(false);
  let timeSlot = $state<TimeSlot>(currentSlot());
  let query = $state('');
  let results = $state<FoodItem[]>([]);
  let selected = $state<FoodItem | null>(null);
  let grams = $state(100);
  let manual = $state(false);
  let manualName = $state('');
  let manualKcal = $state(0);
  let searchTimer: ReturnType<typeof setTimeout>;

  $effect(() => {
    const q = query.trim();
    clearTimeout(searchTimer);
    if (q.length < 2) {
      results = [];
      return;
    }
    searchTimer = setTimeout(async () => {
      results = await api.searchFoodItems(q);
    }, 200);
  });

  function pick(item: FoodItem) {
    selected = item;
    query = item.name;
    results = [];
  }

  function scale(per100: number) {
    return (per100 * grams) / 100;
  }

  async function logPreset(preset: MealPreset) {
    await meals.logPreset(currentSlot(), preset);
  }

  async function logSelected() {
    if (!selected) return;
    await meals.log(
      timeSlot,
      selected.name,
      scale(selected.kcal_per_100g),
      scale(selected.protein_per_100g),
      scale(selected.carb_per_100g),
      scale(selected.fat_per_100g),
    );
    selected = null;
    query = '';
    grams = 100;
  }

  async function logManual(e: Event) {
    e.preventDefault();
    if (!manualName.trim() || manualKcal <= 0) return;
    await meals.log(timeSlot, manualName.trim(), manualKcal, 0, 0, 0);
    manualName = '';
    manualKcal = 0;
    manual = false;
  }

  const kcalTotal = $derived(meals.view?.kcal_total ?? 0);
  const kcalBudget = $derived(meals.view?.kcal_budget ?? 2000);
  const pct = $derived(Math.min((kcalTotal / kcalBudget) * 100, 100));
  const overBudget = $derived(kcalTotal > kcalBudget);
</script>

<section class="pillar-section">
  <div class="pillar-header">
    <h2>Sofra</h2>
    <div class="pillar-hairline">
      <div style={`width:${pct}%;background:${overBudget ? 'var(--body)' : 'var(--life)'}`}></div>
    </div>
    <span class="action-meta" class:overdue={overBudget}
      >{Math.round(kcalTotal)} / {Math.round(kcalBudget)} kcal</span
    >
    <button
      class="add-toggle"
      class:open={adding}
      style="margin-left:0"
      onclick={() => (adding = !adding)}
      aria-label="Öğün ekle">+</button
    >
  </div>

  {#if meals.presets.length > 0}
    <!-- Already-eaten meals, most frequent first: the usual breakfast is one
         tap, not a search and a gram count all over again. -->
    <div class="chip-row">
      {#each meals.presets as preset (preset.name)}
        <button class="chip" onclick={() => logPreset(preset)}>
          {preset.name} <span class="chip-kcal">{Math.round(preset.kcal)}</span>
        </button>
      {/each}
    </div>
  {/if}

  {#if adding}
    <div class="add-form">
      <select class="task-select" bind:value={timeSlot} aria-label="Öğün">
        {#each slots as [value, label] (value)}
          <option {value}>{label}</option>
        {/each}
      </select>

      {#if !manual}
        <div class="sofra-search">
          <input
            class="task-input"
            placeholder="Yiyecek ara…"
            bind:value={query}
            oninput={() => (selected = null)}
          />
          {#if results.length > 0}
            <div class="sofra-results">
              {#each results as item (item.id)}
                <button class="sofra-result" onclick={() => pick(item)}>
                  {item.name}
                  <span class="action-meta">{Math.round(item.kcal_per_100g)} kcal/100g</span>
                </button>
              {/each}
            </div>
          {/if}
        </div>
        {#if selected}
          <label class="task-today-toggle">
            gram <input class="num-input" type="number" min="1" bind:value={grams} />
          </label>
          <span class="action-meta">{Math.round(scale(selected.kcal_per_100g))} kcal</span>
          <button class="btn primary" onclick={logSelected}>Ekle</button>
        {/if}
        <button class="focus-btn" onclick={() => (manual = true)}>Elle gir</button>
      {:else}
        <form class="sofra-manual" onsubmit={logManual}>
          <input class="task-input" placeholder="Öğün adı" bind:value={manualName} />
          <input class="num-input" type="number" min="1" placeholder="kcal" bind:value={manualKcal} />
          <button class="btn primary" type="submit">Ekle</button>
          <button type="button" class="focus-btn" onclick={() => (manual = false)}>Ara</button>
        </form>
      {/if}
    </div>
  {/if}

  {#if !meals.view || meals.view.meals.length === 0}
    <p class="empty-state">Bugün henüz öğün girilmedi.</p>
  {:else}
    {#each meals.view.meals as meal (meal.id)}
      <div class="action-row">
        <span class="action-meta sofra-slot">{slotLabels[meal.time_slot]}</span>
        <span class="action-name">{meal.name}</span>
        <span class="action-meta">{Math.round(meal.kcal)} kcal</span>
        <button class="focus-btn" onclick={() => meals.remove(meal.id)}>Sil</button>
      </div>
    {/each}
  {/if}
</section>

<style>
  .sofra-search {
    position: relative;
    flex: 1;
    min-width: 160px;
  }
  .sofra-results {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    z-index: 5;
    background: var(--card);
    border: 1px solid var(--card-border);
    border-radius: var(--radius-sm);
    margin-top: 4px;
    max-height: 220px;
    overflow-y: auto;
  }
  .sofra-result {
    display: flex;
    justify-content: space-between;
    width: 100%;
    text-align: left;
    background: none;
    border: none;
    color: var(--ink);
    padding: 8px 12px;
    cursor: pointer;
    font-size: 13px;
  }
  .sofra-result:hover {
    background: var(--surface);
  }
  .sofra-manual {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    flex: 1;
  }
  .sofra-slot {
    text-transform: uppercase;
    font-size: 10.5px;
    letter-spacing: 0.06em;
    min-width: 70px;
  }
  .chip-kcal {
    color: var(--ink-faint);
    margin-left: 4px;
  }
</style>
