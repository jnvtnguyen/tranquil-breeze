<script lang="ts">
    import { scale } from "svelte/transition";
	import { field, form, message } from "$lib/form";
	import { min, required } from "$lib/form/validators";
    import { Modal, Input, Button } from "$lib/components";
	import { goto } from "$app/navigation";
	import { PUBLIC_APP_HOST } from "$env/static/public";

    export let open: boolean = false;

    let loading: {
        form: boolean,
        slug: boolean
    } = {
        form: false,
        slug: false
    }
    let slug: string = "";

    const format = (value: string) => {
        return value.replace(/\s+/g, '-').replace(/'/g, '').toLowerCase();
    }

    const duplicate = () => {
        return async (value: string) => {
            if(loading.slug) return;
            loading.slug = true;
            return await fetch("/api/workspaces/check-slug", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({ slug: format(value) })
            }).then((response) => response.json()).then(({ valid }) => {
                loading.slug = false;
                return { valid, name: "duplicate" };
            });
        }
    }

    const _name = field("name", "", "Name", [required(), min(5), duplicate()]);
    const _form = form(_name);

    const handleSubmit = async (e: Event) => {
        e.preventDefault();
        await _form.validate();

        if($_form.valid) {
            loading.form = true;
            let response = await fetch("/api/workspaces/create", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({ workspace: { name: $_name.value, slug } })
            });

            loading.form = false;

            if(response?.ok) {
                open = false; 
                goto(`/workspaces/${slug}`);
            }
        }
    }

    $: slug = format($_name.value);
    $: full = `${PUBLIC_APP_HOST}/workspaces/` + slug;
</script>

<Modal title="Create a Workspace" size="sm" transition={scale} bind:open={open}>
    <form>
        <Input
            name="name"
            label="Name"
            placeholder="Name"
            required={true}
            error={message($_name, { error: "duplicate", message: "This name  slug is already taken."})}
            loading={loading.slug}
            {...$_name.meta}
            autocomplete="off"
            autofocus
            max={20}
            bind:value={$_name.value}
            on:blur={() => _name.validate()}
        />

        <Input
            name="slug"
            label="Slug"
            placeholder="Slug"
            required={true}
            bind:value={full}
            disabled
        />
    </form>
    <div slot="footer">
        <Button type="button" text="Create" disabled={loading.form} loading={loading.form} on:click={handleSubmit} />
    </div>
</Modal>