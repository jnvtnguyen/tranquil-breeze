<script lang="ts">
	import { field, form, message } from "$lib/form";
	import { min, required } from "$lib/form/validators";
    import { Modal, Input, Button } from "$lib/components";
	import { goto } from "$app/navigation";

    export let open: boolean = false;

    let loading: boolean = false;
    let slug: string = "";
    let slugLoading: boolean = false;

    const format = (value: string) => {
        return value.replace(/\s+/g, '-').replace(/'/g, '').toLowerCase();
    }

    const duplicate = () => {
        return async (value: string) => {
            if(slugLoading) return;
            return await fetch("/api/workspaces/check-slug", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({ slug: format(value) })
            }).then((response) => response.json()).then(({ valid }) => {
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
            try {
                loading = true;
                let response = await fetch("/api/workspaces/create", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify({ workspace: { name: $_name.value, slug, image: "" } })
                });

                loading = false;

                if(response?.ok) {
                    open = false; 
                    goto(`/workspaces/${slug}`);
                }
            }
            catch(e) {
                console.error(e);
            }
        }
    }

    $: slug = format($_name.value);
    $: fullSlug = "http://localhost:3000/workspaces/" + slug;
</script>

<Modal title="Create a Workspace" size="sm" bind:open={open}>
    <form>
        <Input
            name="name"
            label="Name"
            placeholder="Name"
            required={true}
            error={message($_name, { error: "duplicate", message: "This name and slug is already taken."})}
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
            bind:value={fullSlug}
            disabled
        />
    </form>
    <div slot="footer">
        <Button type="button" text="Create" disabled={loading} loading={loading} on:click={handleSubmit} />
    </div>
</Modal>