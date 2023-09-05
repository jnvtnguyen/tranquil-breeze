<script lang="ts">
	import { fade, slide } from "svelte/transition";
	import { PageMeta, Input, Button, Link, Alert } from '$lib/components/';
	import { form, field, message } from '$lib/form/';
	import { required } from '$lib/form/validators/';

	let loading: boolean = false;
	let error: string | null = null;

	const _email = field("email", "", "Email", [required()]);
	const _password = field("password", "", "Password", [required()]);

	const _form = form(_email, _password);

	const handleSubmit = async (e: Event) => {
		e.preventDefault();
		await _form.validate();

		if($_form.valid) {
			loading = true;
			error = null;

			let response = await fetch("/api/users/login", {
				method: "POST",
				headers: {
					"Content-Type": "application/json"
				},
				body: JSON.stringify({
					user: $_form.summary
				})
			});
			loading = false

			if(response?.ok) return;

			if (response.status === 401) {
				error = "The email or password you entered is incorrect.";
			}
			else {
				error = "An error occurred. Please try again.";
			}
		}
	}
</script>

<PageMeta title="Log in" />

<h1>Log in</h1>

{#if error}
	<div class="alert" in:fade out:slide>
		<Alert text={error} />
	</div>
{/if}

<form>
	<Input 
		name="email" 
		type="email" 
		label="Email" 
		placeholder="Email" 
		required={true} 
		error={message($_email)}
		{...$_email.meta}
		autofocus
		bind:value={$_email.value} 
		on:blur={() => _email.validate()}
	/>
	<Input 
		name="password" 
		type="password" 
		label="Password" 
		placeholder="Password" 
		required={true} 
		error={message($_password)}
		{...$_password.meta}
		bind:value={$_password.value} 
		on:blur={() => _password.validate()}
	/>

	<div class="button">
		<Button type="button" text="Log in" disabled={loading} loading={loading} on:click={handleSubmit} />
	</div>

	<p class="link">Don't have an account? <Link href="/signup" text="Sign up" /></p>
</form>

<style lang='scss'>
	@use "$lib/css/variables" as *;
	@import "../auth.scss";

	.alert {
		margin-bottom: $spacing-2;
	}
</style>