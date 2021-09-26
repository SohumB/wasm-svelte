import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import rust from "@wasm-tool/rollup-plugin-rust";

// we create rust closures that javascript needs to memory-manage
// specifically, the unsubscribe functions
// handing off memory to JS depends on the experimental js weakref feature
// but modern browsers seem to be fine https://caniuse.com/?search=weakref
// in other browsers we will simply leak some memory
// and yes, this is the easiest way to get this flag to wasm-bindgen
process.env['WASM_BINDGEN_WEAKREF'] = 1;

export default defineConfig({
  plugins: [
    svelte({
      /* plugin options */
    }),
    rust(),
  ]
});
