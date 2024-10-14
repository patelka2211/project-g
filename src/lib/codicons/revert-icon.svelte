<script lang="ts">
	interface Props {
		[key: string]: any;
	}

	let { ...props }: Props = $props();
</script>

<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" {...props}>
	<path
		fill="none"
		stroke="currentColor"
		stroke-width="2"
		d="M8 3L3 8l5 5m4 7h3a6 6 0 1 0 0-12H4"
	/>
</svg>
