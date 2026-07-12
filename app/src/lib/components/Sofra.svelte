<script lang="ts">
  import { meals } from '../stores/meals.svelte';
  import { api } from '../api/commands';
  import type { FoodItem, TimeSlot } from '../api/types';

  const slots: TimeSlot[] = ['breakfast', 'lunch', 'dinner', 'snack'];

  function defaultSlot(): TimeSlot {
    const h = new Date().getHours();
    if (h < 11) return 'breakfast';
    if (h < 16) return 'lunch';
    if (h < 21) return 'dinner';
    return 'snack';
  }

  let timeSlot = $state<TimeSlot>(defaultSlot());
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
  </div>

  <div class="sofra-add">
    <select class="task-select" bind:value={timeSlot}>
      {#each slots as s}
        <option value={s}>{s}</option>
      {/each}
    </select>

    {#if !manual}
      <div class="sofra-search">
        <input
          class="task-input"
          placeholder="Search a food…"
          bind:value={query}
          oninput={() => (selected = null)}
        />
        {#if results.length > 0}
          <div class="sofra-results">
            {#each results as item (item.id)}
              <button class="sofra-result" onclick={() => pick(item)}>
                {item.name} <span class="action-meta">{Math.round(item.kcal_per_100g)} kcal/100g</span>
              </button>
            {/each}
          </div>
        {/if}
      </div>
      {#if selected}
        <label class="task-today-toggle">
          grams <input class="sofra-grams" type="number" min="1" bind:value={grams} />
        </label>
        <span class="action-meta">{Math.round(scale(selected.kcal_per_100g))} kcal</span>
        <button class="btn primary" onclick={logSelected}>Add</button>
      {/if}
      <button class="focus-btn" onclick={() => (manual = true)}>Manual entry</button>
    {:else}
      <form class="sofra-manual" onsubmit={logManual}>
        <input class="task-input" placeholder="Meal name" bind:value={manualName} />
        <input class="sofra-grams" type="number" min="1" placeholder="kcal" bind:value={manualKcal} />
        <button class="btn primary" type="submit">Add</button>
        <button type="button" class="focus-btn" onclick={() => (manual = false)}>Search instead</button>
      </form>
    {/if}
  </div>

  {#if !meals.view || meals.view.meals.length === 0}
    <p class="empty-state">No meals logged yet today.</p>
  {:else}
    {#each meals.view.meals as meal (meal.id)}
      <div class="action-row">
        <span class="action-meta sofra-slot">{meal.time_slot}</span>
        <span class="action-name">{meal.name}</span>
        <span class="action-meta">{Math.round(meal.kcal)} kcal</span>
        <button class="focus-btn" onclick={() => meals.remove(meal.id)}>Remove</button>
      </div>
    {/each}
  {/if}
</section>

<style>
  .sofra-add {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    padding: 12px 18px;
    border-bottom: 1px solid var(--card-border);
    flex-wrap: wrap;
  }
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
  .sofra-grams {
    width: 64px;
    background: var(--surface);
    border: 1px solid var(--card-border);
    border-radius: var(--radius-sm);
    color: var(--ink);
    padding: 8px 10px;
    font-size: 13px;
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
</style>
