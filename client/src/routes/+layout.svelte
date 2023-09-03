<script lang="ts">
	import '$lib/css/global.scss';
	import { fly } from 'svelte/transition';
	import { cubicIn, cubicOut } from 'svelte/easing';

	export let data;

	const duration = 200;
	const delay = duration + 100;
	const y = 10;

	const flyIn = { easing: cubicOut, y, duration, delay };
	const flyOut = { easing: cubicIn, y: -1, duration };
</script>

{#key data.pathname}
	<div class="page" in:fly={flyIn} out:fly={flyOut}>
		<slot />
	</div>
{/key}

<style lang="scss">
	@use "$lib/css/variables" as *;
	.page {
		width: 100vw;
		height: 100vh;
		overflow: hidden;
	}
</style>