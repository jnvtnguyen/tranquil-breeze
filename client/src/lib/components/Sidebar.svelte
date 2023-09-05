<script lang="ts">
    import { page } from "$app/stores";
    import { LINKS } from "$lib/links"
    import { Header, Link } from "./Sidebar/";

    const linkWith = (href: string) => {
        return `/workspaces/${$page.data.workspace.slug}${href}`;
    }
</script>

<div class="sidebar">
    <Header title="Workspace" />
    {#each LINKS as link}
        <Link title={link.title} href={linkWith(link.href)}>
            <svelte:component this={link.icon} slot="icon" />
        </Link>
    {/each}
</div>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .sidebar {
        display: flex;
        flex-direction: column;
        width: 240px;
        height: 100%;
        background: $color-sidebar;
        box-shadow: rgba(50, 50, 93, 0.25) 0px 6px 12px -2px, rgba(0, 0, 0, 0.3) 0px 3px 7px -3px;
        z-index: 1;
        padding-top: $spacing-4;
    }
</style>