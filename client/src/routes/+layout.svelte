<script lang="ts">
	import { onNavigate } from '$app/navigation';
	import '$lib/css/global.scss';

	export let data;

	onNavigate((navigation) => {
		//@ts-ignore
		if(!document.startViewTransition) return;

		return new Promise((resolve) => {
			//@ts-ignore
			document.startViewTransition(async () => {
				resolve();
				await navigation.complete;
			});
		})
	});
</script>

{#key data.pathname}
	<div class="page">
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