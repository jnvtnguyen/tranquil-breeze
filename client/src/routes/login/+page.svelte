<script lang="ts">
	import { PageMeta, Input, Button } from '$lib/components/';
	import { form, field, message } from '$lib/form/';
	import { required } from '$lib/form/validators/';

	const _email = field("email", "", "Email", [required()]);
	const _password = field("password", "", "Password", [required()]);

	const _form = form(_email, _password);

	const handleSubmit = async (e: Event) => {
		e.preventDefault();
		await _form.validate();

		if($_form.valid) {
			console.log($_form.summary)
		}
	}
</script>

<PageMeta title="Sign Up" />
<main class="main">
	<div class="container">
		<h1>Login</h1>

		<form>
			<Input 
				name="email" 
				type="email" 
				label="Email" 
				placeholder="Email" 
				required={true} 
				error={message($_email)}
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
				bind:value={$_password.value} 
				on:blur={() => _password.validate()}
			/>

			<div class="button">
				<Button type="button" text="Login" on:click={handleSubmit} />
			</div>
		</form>
	</div>
</main>

<style lang="scss">
	@import "../shared/auth/auth.scss";
</style>