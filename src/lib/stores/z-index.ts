import { get, writable } from 'svelte/store';

export const zIndex = (() => {
	const MINIMUM_VALUE = 100;

	const store = writable(MINIMUM_VALUE);

	function getNext() {
		const zIndex = get(store);

		store.set(zIndex + 1);

		return zIndex;
	}

	function decrease() {
		const zIndex = get(store);

		if (zIndex - 1 < MINIMUM_VALUE) return zIndex;

		store.set(zIndex - 1);

		return zIndex - 1;
	}

	return {
		...store,
		getNext,
		decrease
	};
})();
