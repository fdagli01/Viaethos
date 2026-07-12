<script lang="ts">
  import { onMount } from 'svelte';
  import { today } from './lib/stores/today.svelte';
  import Today from './lib/components/Today.svelte';
  import Ledger from './lib/components/Ledger.svelte';

  let screen = $state<'today' | 'ledger'>('today');

  onMount(() => {
    today.init();
  });
</script>

<div class="app-shell">
  <div class="top-nav">
    <h1>Via Ethos</h1>
    <div class="nav-switch">
      <button class:active={screen === 'today'} onclick={() => (screen = 'today')}>Today</button>
      <button class:active={screen === 'ledger'} onclick={() => (screen = 'ledger')}
        >The Ledger</button
      >
    </div>
    <span class="points">{today.view?.total_points.toLocaleString() ?? ''} pts</span>
  </div>

  {#if screen === 'today'}
    <Today />
  {:else}
    <Ledger />
  {/if}
</div>
