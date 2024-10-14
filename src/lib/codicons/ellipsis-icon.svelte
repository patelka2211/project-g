<script lang="ts">
	interface Props {
		[key: string]: any;
	}

	let { ...props }: Props = $props();
</script>

<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" {...props}>
	<path
		fill="currentColor"
		d="M4 8a1 1 0 1 1-2 0a1 1 0 0 1 2 0m5 0a1 1 0 1 1-2 0a1 1 0 0 1 2 0m5 0a1 1 0 1 1-2 0a1 1 0 0 1 2 0"
	/>
</svg>
