import { readable } from 'svelte/store';

export const raf = readable(performance.now(), update => {
  function onTick(timestamp) {
    update(timestamp);
    requestAnimationFrame(onTick);
  }

  onTick();
});
