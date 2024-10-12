import { goto } from '$app/navigation';
import { get as _get, writable } from 'svelte/store';

export const repoPath = (() => {
	const store = writable(window.sessionStorage.getItem('current-repo'));

	function set(value: string) {
		window.sessionStorage.setItem('current-repo', value);
		store.set(value);
	}

	async function setAndBrowse(value: string) {
		set(value);
		await goto('/browse');
	}

	function clear() {
		window.sessionStorage.removeItem('current-repo');
		store.set(null);
	}

	function get() {
		return _get(store);
	}

	return {
		...store,
		set,
		setAndBrowse,
		clear,
		get
	};
})();
