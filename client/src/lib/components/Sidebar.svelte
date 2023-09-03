<script lang="ts">
    import type { Workspace } from "$lib/types";
    //@ts-ignore
    import HomeIcon from "~icons/bytesize/home";
    import Link from "./Sidebar/Link.svelte";

    export let workspace: Workspace;

    const link = (href: string) => {
        return `/workspaces/${workspace.slug}${href}`;
    }
</script>

<div class="sidebar">
    <Link title="Projects Home" href={link("")}>
        <HomeIcon slot="icon" />
    </Link>
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