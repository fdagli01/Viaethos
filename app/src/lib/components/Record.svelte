<script lang="ts">
  import Path from './Path.svelte';
  import Ledger from './Ledger.svelte';
  import MementoMori from './MementoMori.svelte';
</script>

<!-- One record screen instead of three: the walked path, the numbers behind
     it, and the horizon it all sits inside. -->
<div class="view">
  <Path />
  <Ledger />
  <MementoMori />
</div>
