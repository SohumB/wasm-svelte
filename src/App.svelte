<script>
 import RAF from './RAF.svelte';
 import { raf } from './raw-store';
 import { raf as rust_raf } from './wasm-store';
</script>

<main>
  <p>Hello from Svelte!</p>
  <p><RAF title="Raw JS" timer={raf} /></p>
  {#await rust_raf() then rraf}
    <p><RAF title="WASM" timer={rraf} /></p>
  {/await}
</main>
