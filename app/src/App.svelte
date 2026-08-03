<script lang="ts">
  import { onMount } from 'svelte';
  import { today } from './lib/stores/today.svelte';
  import Today from './lib/components/Today.svelte';
  import QuietMode from './lib/components/QuietMode.svelte';
  import Record from './lib/components/Record.svelte';
  import Settings from './lib/components/Settings.svelte';

  // Four screens, not seven: the day, the record, the settings — and the
  // painted one you leave open. Everything the old nav listed separately
  // (Path, Memento Mori, Manage) now lives inside one of these.
  type Screen = 'quiet' | 'today' | 'record' | 'settings';
  const tabs: [Screen, string][] = [
    ['quiet', 'Sessiz'],
    ['today', 'Bugün'],
    ['record', 'Kayıt'],
    ['settings', 'Ayarlar'],
  ];

  let screen = $state<Screen>('quiet');

  onMount(() => {
    today.init();
  });
</script>

<div class="app-shell">
  <div class="top-nav">
    <h1>Via Ethos</h1>
    <div class="nav-switch">
      {#each tabs as [id, label] (id)}
        <button class:active={screen === id} onclick={() => (screen = id)}>{label}</button>
      {/each}
    </div>
    <span class="points">{today.view?.total_points.toLocaleString() ?? ''} puan</span>
  </div>

  {#if screen === 'quiet'}
    <div class="view qm-view"><QuietMode /></div>
  {:else if screen === 'today'}
    <Today />
  {:else if screen === 'record'}
    <Record />
  {:else}
    <Settings />
  {/if}
</div>
