<script lang="ts">
  import { onMount } from 'svelte';
  import { today } from './lib/stores/today.svelte';
  import Today from './lib/components/Today.svelte';
  import Ledger from './lib/components/Ledger.svelte';
  import QuietMode from './lib/components/QuietMode.svelte';

  let screen = $state<'quiet' | 'today' | 'ledger'>('quiet');

  onMount(() => {
    today.init();
  });
</script>

<div class="app-shell">
  <div class="top-nav">
    <h1>Via Ethos</h1>
    <div class="nav-switch">
      <button class:active={screen === 'quiet'} onclick={() => (screen = 'quiet')}
        >Quiet Mode</button
      >
      <button class:active={screen === 'today'} onclick={() => (screen = 'today')}>Today</button>
      <button class:active={screen === 'ledger'} onclick={() => (screen = 'ledger')}
        >The Ledger</button
      >
    </div>
    <span class="points">{today.view?.total_points.toLocaleString() ?? ''} pts</span>
  </div>

  {#if screen === 'quiet'}
    <div class="view qm-view"><QuietMode /></div>
  {:else if screen === 'today'}
    <Today />
  {:else}
    <Ledger />
  {/if}
</div>
