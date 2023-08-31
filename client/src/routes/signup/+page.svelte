<script lang="ts">
	import type { Validator } from '$lib/form/types';
	import { PageMeta, Input, Button } from '$lib/components/';
	import { form, field, message } from '$lib/form/';
	import { required, email, min } from '$lib/form/validators/';

	let loading: boolean = false;

	const duplicate = (): Validator => {
		return async (value: string) => {
			const response = await fetch("/api/users/check-email", {
				method: "POST",
				headers: {
					"Content-Type": "application/json"
				},
				body: JSON.stringify({ email: value })
			});

			const { valid } = await response.json();

			return { valid, name: "duplicate" };
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
			try {
				loading = true;
				let response = await fetch("/api/users/signup", {
					method: "POST",
					headers: {
						"Content-Type": "application/json"
					},
					body: JSON.stringify({
						user: $_form.summary
					})
				}).then((response) => {
					return response;
				});

				if(response?.ok) {
					loading = false
				}
			}
			catch(e) {
				console.error(e);
			}
		}
	}
</script>

<PageMeta title="Sign Up" />
<main class="main">
	<div class="container">
		<h1>Sign Up</h1>

		<form>
			<Input 
				name="name" 
				label="Name" 
				placeholder="Name" 
				required={true} 
				error={message($_name)}
				{...$_name.meta}
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
				<Button type="button" text="Sign Up" disabled={loading} on:click={handleSubmit} />
			</div>
		</form>
	</div>
</main>

<style lang="scss">
	@import "../shared/auth/auth.scss";
</style>