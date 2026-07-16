<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '../api/commands';
  import type { SettingsView } from '../api/types';

  let settings = $state<SettingsView | null>(null);
  let dayBoundaryHour = $state(0);
  let calorieBudget = $state(2000);
  let weatherLat = $state(41.0082);
  let weatherLon = $state(28.9784);
  let saved = $state(false);
  let hasAiApiKey = $state(false);
  let aiApiKeyInput = $state('');
  let aiKeySaved = $state(false);

  const cities: [string, number, number][] = [
    ['Istanbul', 41.0082, 28.9784],
    ['Ankara', 39.9334, 32.8597],
    ['Izmir', 38.4237, 27.1428],
    ['London', 51.5072, -0.1276],
    ['New York', 40.7128, -74.006],
  ];

  onMount(async () => {
    settings = await api.getSettings();
    dayBoundaryHour = settings.day_boundary_hour;
    calorieBudget = settings.calorie_budget;
    weatherLat = settings.weather_lat;
    weatherLon = settings.weather_lon;
    hasAiApiKey = settings.has_ai_api_key;
  });

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
</script>

<div class="view">
  <div class="card">
    <h3>Day Boundary</h3>
    <p class="settings-hint">
      What hour does your day end? Night owls can push this past midnight — a session logged at
      2am still counts toward yesterday if the boundary is set to, say, 4.
    </p>
    <form class="settings-form" onsubmit={save}>
      <label class="task-today-toggle">
        Hour (0&ndash;23)
        <input class="sofra-grams" type="number" min="0" max="23" bind:value={dayBoundaryHour} />
      </label>

      <h3 class="settings-subhead">Sofra</h3>
      <label class="task-today-toggle">
        Daily calorie budget
        <input class="sofra-grams" type="number" min="0" step="50" bind:value={calorieBudget} />
      </label>

      <h3 class="settings-subhead">Real Weather Location</h3>
      <div class="settings-cities">
        {#each cities as [name, lat, lon]}
          <button type="button" class="focus-btn" onclick={() => pickCity(lat, lon)}>{name}</button>
        {/each}
      </div>
      <label class="task-today-toggle">
        Latitude <input class="sofra-grams" type="number" step="0.0001" bind:value={weatherLat} />
      </label>
      <label class="task-today-toggle">
        Longitude <input class="sofra-grams" type="number" step="0.0001" bind:value={weatherLon} />
      </label>

      <div class="settings-actions">
        <button class="btn primary" type="submit">Save</button>
        {#if saved}<span class="action-meta">Saved.</span>{/if}
      </div>
    </form>
  </div>

  <div class="card">
    <h3>AI Program Önerisi</h3>
    <p class="settings-hint">
      Program ekranındaki "AI ile öner" düğmesi bir Anthropic API anahtarı gerektirir. Anahtar
      yalnızca bu bilgisayardaki yerel veritabanında durur, başka hiçbir yere gönderilmez.
    </p>
    <form class="settings-form" onsubmit={saveAiKey}>
      <label class="task-today-toggle">
        API anahtarı
        <input
          class="task-input"
          type="password"
          placeholder={hasAiApiKey ? 'Ayarlandı — değiştirmek için yeni bir anahtar gir' : 'sk-ant-…'}
          bind:value={aiApiKeyInput}
        />
      </label>
      <div class="settings-actions">
        <button class="btn primary" type="submit">Kaydet</button>
        {#if aiKeySaved}<span class="action-meta">Kaydedildi.</span>{/if}
      </div>
    </form>
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
  .settings-subhead {
    margin: 10px 0 0;
    font-size: 12px;
  }
  .settings-cities {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }
  .settings-actions {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-top: 8px;
  }
</style>
