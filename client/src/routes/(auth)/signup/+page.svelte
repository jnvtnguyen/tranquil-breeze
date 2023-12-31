<script lang="ts">
	import type { Validator } from '$lib/form/types';
	import { PageMeta, Input, Button, Link } from '$lib/components/';
	import { form, field, message } from '$lib/form/';
	import { required, email, min } from '$lib/form/validators/';

	let loading: {
		form: boolean,
		email: boolean
	} = {
		form: false,
		email: false
	};

	const duplicate = (): Validator => {
		return async (value: string) => {
			if(loading.email) return;
			loading.email = true;
			return await fetch("/api/users/check-email", {
				method: "POST",
				headers: {
					"Content-Type": "application/json"
				},
				body: JSON.stringify({ email: value })
			}).then((response) => response.json()).then(({ valid }) => {
				loading.email = false;
				return { valid, name: "duplicate" };
			});
		}
	}

	const _name = field("name", "", "Name", [required()]);
	const _email = field("email", "", "Email", [required(), email(), duplicate()]);
	const _password = field("password", "", "Password", [required(), min(8)]);

	const _form = form(_name, _email, _password);

	const handleSubmit = async (e: Event) => {
		e.preventDefault();
		await _form.validate();

		if($_form.valid) {
			loading.form = true;
			let response = await fetch("/api/users/signup", {
				method: "POST",
				headers: {
					"Content-Type": "application/json"
				},
				body: JSON.stringify({
					user: $_form.summary
				})
			});

			loading.form = false;

			if(!response?.ok) return;
		}
	}
</script>

<PageMeta title="Sign up" />
<h1>Sign up</h1>

<form>
	<Input 
		name="name" 
		label="Name" 
		placeholder="Name" 
		required={true} 
		error={message($_name)}
		{...$_name.meta}
		autofocus
		bind:value={$_name.value}
		on:blur={() => _name.validate()}
	/>
	<Input 
		name="email" 
		type="email" 
		label="Email" 
		placeholder="Email" 
		required={true} 
		error={message($_email, { error: "duplicate", message: "This email is already in use." })}
		loading={loading.email}
		{...$_email.meta}
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
		<Button type="button" text="Sign up" disabled={loading.form} loading={loading.form} on:click={handleSubmit} />
	</div>

	<p class="link">Already have an account? <Link href="/login" text="Log in" /></p>
</form>

<style lang='scss'>
	@import "../auth.scss";
</style>