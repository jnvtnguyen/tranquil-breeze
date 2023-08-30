<script lang="ts">
	import { PageMeta, Input } from '$lib/components';
</script>

<PageMeta title="Sign Up" />
<main class="main">
	<div class="container">
		<form>
			<Input name="name" label="Name" />
			<Input name="email" type="email" label="Email" />
			<Input name="password" type="password" label="Password" />
		</form>
	</div>
</main>

<style lang="scss">
	@use "$lib/css/spacing" as *;

	.main {
		display: flex;
		width: 100%;
		height: 100%;
		align-items: center;
		justify-content: center;
	}

	.container {
		width: 480px;
		padding-top: $spacing-8;
	}
</style>