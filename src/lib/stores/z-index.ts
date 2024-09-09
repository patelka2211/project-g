import { get, writable } from "svelte/store";

export const zIndex = (() => {
  const MINIMUM_VALUE = 100;

  let store = writable(MINIMUM_VALUE);

  function getNext() {
    let zIndex = get(store);

    store.set(zIndex + 1);

    return zIndex;
  }

  function decrease() {
    let zIndex = get(store);

    if (zIndex - 1 < MINIMUM_VALUE) return zIndex;

    store.set(zIndex - 1);

    return zIndex - 1;
  }

  return {
    ...store,
    getNext,
    decrease,
  };
})();
