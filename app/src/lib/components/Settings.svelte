<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../api/commands';
  import { today } from '../stores/today.svelte';
  import type { ActionAdminView, SettingsView } from '../api/types';

  let settings = $state<SettingsView | null>(null);
  let dayBoundaryHour = $state(0);
  let calorieBudget = $state(2000);
  let weatherLat = $state(41.0082);
  let weatherLon = $state(28.9784);
  let saved = $state(false);

  let hasAiApiKey = $state(false);
  let aiApiKeyInput = $state('');
  let aiKeySaved = $state(false);

  let actions = $state<ActionAdminView[]>([]);
  let editingId = $state<string | null>(null);
  let addingAction = $state(false);
  let newName = $state('');
  let newPillarId = $state('');
  let newKind = $state<'focus' | 'tick'>('tick');

  let confirmingReset = $state(false);
  let resetting = $state(false);
  let resetDone = $state(false);

  // Coordinates are a setting nobody wants to type. The city list covers the
  // realistic cases; the raw lat/lon inputs are gone.
  const cities: [string, number, number][] = [
    ['İstanbul', 41.0082, 28.9784],
    ['Ankara', 39.9334, 32.8597],
    ['İzmir', 38.4237, 27.1428],
    ['Londra', 51.5072, -0.1276],
    ['New York', 40.7128, -74.006],
  ];

  const schedules: [ActionAdminView['schedule_type'], string][] = [
    ['daily', 'Her gün'],
    ['weekdays', 'Hafta içi'],
    ['times_per_week', 'Haftada n'],
  ];

  function rhythmLabel(a: ActionAdminView): string {
    if (a.schedule_type === 'times_per_week') return `haftada ${a.times_per_week ?? 3}`;
    return a.schedule_type === 'weekdays' ? 'hafta içi' : 'her gün';
  }

  onMount(async () => {
    await loadSettings();
    actions = await api.getManageActions();
  });

  async function loadSettings() {
    settings = await api.getSettings();
    dayBoundaryHour = settings.day_boundary_hour;
    calorieBudget = settings.calorie_budget;
    weatherLat = settings.weather_lat;
    weatherLon = settings.weather_lon;
    hasAiApiKey = settings.has_ai_api_key;
  }

  const activeCity = $derived(
    cities.find(([, lat, lon]) => lat === weatherLat && lon === weatherLon)?.[0] ?? null,
  );

  function pickCity(lat: number, lon: number) {
    weatherLat = lat;
    weatherLon = lon;
  }

  async function save(e: Event) {
    e.preventDefault();
    settings = await api.updateSettings(dayBoundaryHour, calorieBudget, weatherLat, weatherLon);
    saved = true;
    setTimeout(() => (saved = false), 2000);
  }

  async function saveAiKey(e: Event) {
    e.preventDefault();
    if (!aiApiKeyInput.trim()) return;
    await api.setAiApiKey(aiApiKeyInput.trim());
    hasAiApiKey = true;
    aiApiKeyInput = '';
    aiKeySaved = true;
    setTimeout(() => (aiKeySaved = false), 2000);
  }

  async function savePillar(pillarId: string, name: string, colorToken: string) {
    await api.updatePillar(pillarId, name, colorToken);
    await today.refresh();
  }

  async function saveAction(a: ActionAdminView) {
    actions = await api.updateAction(
      a.id,
      a.name,
      a.kind === 'focus' ? a.default_minutes : null,
      a.schedule_type,
      a.schedule_type === 'times_per_week' ? a.times_per_week : null,
      a.target_per_day,
    );
    editingId = null;
    await today.refresh();
  }

  async function toggleArchived(a: ActionAdminView) {
    actions = await api.setActionArchived(a.id, !a.archived);
    await today.refresh();
  }

  // A new habit only asks for what it cannot guess: a name, a pillar, and
  // whether it is a session or a checkbox. Daily, 25 minutes and once a day
  // are the defaults — all of them editable afterwards.
  async function submitNewAction(e: Event) {
    e.preventDefault();
    if (!newName.trim() || !newPillarId) return;
    actions = await api.addAction(
      newPillarId,
      newName.trim(),
      newKind,
      newKind === 'focus' ? 25 : null,
      'daily',
      null,
      1,
    );
    newName = '';
    addingAction = false;
    await today.refresh();
  }

  async function doReset() {
    resetting = true;
    try {
      await api.resetAllData();
      await loadSettings();
      actions = await api.getManageActions();
      await today.refresh();
      confirmingReset = false;
      resetDone = true;
      setTimeout(() => (resetDone = false), 4000);
    } finally {
      resetting = false;
    }
  }

  const actionsByPillar = $derived(
    today.view
      ? today.view.pillars.map((p) => ({
          pillar: p,
          actions: actions.filter((a) => a.pillar_id === p.id),
        }))
      : [],
  );
</script>

<div class="view">
  <div class="card">
    <h3>Genel</h3>
    <form class="settings-form" onsubmit={save}>
      <label class="task-today-toggle">
        Gün kaçta biter (0&ndash;23)
        <input class="num-input" type="number" min="0" max="23" bind:value={dayBoundaryHour} />
      </label>
      <p class="settings-hint">
        Gece kuşuysan bunu geceyarısının ötesine çekebilirsin &mdash; 4 dersen, saat 02:00&rsquo;de
        biten bir seans hâlâ düne yazılır.
      </p>

      <label class="task-today-toggle">
        Günlük kalori bütçesi
        <input class="num-input" type="number" min="0" step="50" bind:value={calorieBudget} />
      </label>

      <div class="settings-cities">
        <span class="action-meta">Hava durumu</span>
        {#each cities as [name, lat, lon] (name)}
          <button
            type="button"
            class="chip"
            class:on={activeCity === name}
            onclick={() => pickCity(lat, lon)}>{name}</button
          >
        {/each}
      </div>

      <div class="settings-actions">
        <button class="btn primary" type="submit">Kaydet</button>
        {#if saved}<span class="action-meta">Kaydedildi.</span>{/if}
      </div>
    </form>
  </div>

  <div class="card">
    <h3>Sütunlar</h3>
    {#if today.view}
      {#each today.view.pillars as pillar (pillar.id)}
        <div class="manage-row">
          <input
            class="task-input manage-name"
            value={pillar.name}
            onchange={(e) =>
              savePillar(pillar.id, (e.target as HTMLInputElement).value, pillar.color_token)}
          />
          <input
            type="color"
            aria-label={`${pillar.name} rengi`}
            value={pillar.color_token}
            onchange={(e) =>
              savePillar(pillar.id, pillar.name, (e.target as HTMLInputElement).value)}
          />
        </div>
      {/each}
    {/if}
  </div>

  <div class="card">
    <div class="card-head">
      <h3>Alışkanlıklar</h3>
      <button
        class="add-toggle"
        class:open={addingAction}
        onclick={() => (addingAction = !addingAction)}
        aria-label="Alışkanlık ekle">+</button
      >
    </div>

    {#if addingAction}
      <form class="manage-row" onsubmit={submitNewAction}>
        <input class="task-input manage-name" placeholder="Adı" bind:value={newName} />
        {#if today.view}
          <select class="task-select" bind:value={newPillarId} aria-label="Sütun">
            <option value="">Sütun…</option>
            {#each today.view.pillars as pillar (pillar.id)}
              <option value={pillar.id}>{pillar.name}</option>
            {/each}
          </select>
        {/if}
        <button
          type="button"
          class="chip"
          class:on={newKind === 'tick'}
          onclick={() => (newKind = 'tick')}>İşaret</button
        >
        <button
          type="button"
          class="chip"
          class:on={newKind === 'focus'}
          onclick={() => (newKind = 'focus')}>Seans</button
        >
        <button class="btn primary" type="submit">Ekle</button>
      </form>
    {/if}

    {#each actionsByPillar as group (group.pillar.id)}
      {#each group.actions as a (a.id)}
        <div class="manage-row" class:manage-archived={a.archived}>
          <span class="pillar-dot" style={`background:${group.pillar.color_token}`}></span>
          {#if editingId === a.id}
            <input class="task-input manage-name" bind:value={a.name} />
            <select class="task-select" bind:value={a.schedule_type} aria-label="Ritim">
              {#each schedules as [value, label] (value)}
                <option {value}>{label}</option>
              {/each}
            </select>
            {#if a.schedule_type === 'times_per_week'}
              <input class="num-input" type="number" min="1" max="7" bind:value={a.times_per_week} />
            {/if}
            {#if a.kind === 'focus'}
              <label class="task-today-toggle">
                dk <input class="num-input" type="number" min="5" bind:value={a.default_minutes} />
              </label>
            {:else}
              <label class="task-today-toggle">
                &times;/gün
                <input class="num-input" type="number" min="1" bind:value={a.target_per_day} />
              </label>
            {/if}
            <button class="focus-btn" onclick={() => saveAction(a)}>Kaydet</button>
          {:else}
            <span class="action-name">{a.name}</span>
            <span class="action-meta"
              >{a.kind === 'focus' ? 'seans' : 'işaret'} &middot; {rhythmLabel(a)}</span
            >
            <button class="focus-btn" onclick={() => (editingId = a.id)}>Düzenle</button>
          {/if}
          <button class="focus-btn" onclick={() => toggleArchived(a)}
            >{a.archived ? 'Geri al' : 'Arşivle'}</button
          >
        </div>
      {/each}
    {/each}
  </div>

  <div class="card">
    <h3>AI program önerisi</h3>
    <p class="settings-hint">
      Program ekranındaki &ldquo;AI ile öner&rdquo; düğmesi bir Anthropic API anahtarı gerektirir.
      Anahtar yalnızca bu bilgisayardaki yerel veritabanında durur, başka hiçbir yere gönderilmez.
    </p>
    <form class="settings-form" onsubmit={saveAiKey}>
      <input
        class="task-input"
        type="password"
        aria-label="API anahtarı"
        placeholder={hasAiApiKey ? 'Ayarlandı — değiştirmek için yeni bir anahtar gir' : 'sk-ant-…'}
        bind:value={aiApiKeyInput}
      />
      <div class="settings-actions">
        <button class="btn primary" type="submit">Kaydet</button>
        {#if aiKeySaved}<span class="action-meta">Kaydedildi.</span>{/if}
      </div>
    </form>
  </div>

  <div class="card">
    <h3>Her şeyi sıfırla</h3>
    <p class="settings-hint">
      Girilen her şeyi siler &mdash; seanslar, işaretler, puanlar, görevler, öğünler, uyku
      kayıtları, dersler, program blokları ve ayarlar. Uygulama ilk kurulumdaki hâline döner. Geri
      alınamaz.
    </p>
    {#if resetDone}
      <p class="action-meta">Sıfırlandı.</p>
    {:else if !confirmingReset}
      <button class="btn danger" onclick={() => (confirmingReset = true)}>Sıfırla</button>
    {:else}
      <div class="settings-actions">
        <span class="action-meta">Emin misin? Bütün kayıtlar kalıcı olarak silinecek.</span>
        <button class="btn danger" onclick={doReset} disabled={resetting}>
          {resetting ? 'Siliniyor…' : 'Evet, hepsini sil'}
        </button>
        <button class="btn ghost" onclick={() => (confirmingReset = false)}>Vazgeç</button>
      </div>
    {/if}
  </div>
</div>

<style>
  .settings-hint {
    color: var(--ink-muted);
    font-size: 13px;
    line-height: 1.6;
    margin: 0 0 16px;
  }
  .settings-form {
    display: flex;
    flex-direction: column;
    gap: 12px;
    align-items: flex-start;
  }
  .settings-form .settings-hint {
    margin: -4px 0 4px;
  }
  .settings-cities {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    align-items: center;
  }
  .settings-actions {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-top: 8px;
    flex-wrap: wrap;
  }
  .card-head {
    display: flex;
    align-items: center;
    margin-bottom: 14px;
  }
  .card-head h3 {
    margin: 0;
  }
  .manage-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 0;
    border-bottom: 1px solid var(--card-border);
    flex-wrap: wrap;
  }
  .manage-row:last-child {
    border-bottom: none;
  }
  .manage-name {
    max-width: 220px;
  }
  .manage-archived {
    opacity: 0.5;
  }
</style>
