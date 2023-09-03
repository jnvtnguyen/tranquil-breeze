<script lang="ts">
    import type { Workspace as WorkspaceType } from "$lib/types";
	import Workspace from "./Navbar/Workspace.svelte";
    import Breadcrumbs from "./Breadcrumbs.svelte";

    export let workspace: WorkspaceType;
</script>

<div class="navbar">
    <Workspace workspace={workspace} />
    <Breadcrumbs workspace={workspace} />
</div>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .navbar {
        background: $color-white;
        width: 100%;
        height: 84px;
        box-shadow: rgba(99, 99, 99, 0.2) 0px 2px 8px 0px;
        z-index: 2;
        display: flex;
        align-items: center;
        padding-left: $spacing-2;
        padding-right: $spacing-2;
        .breadcrumbs-wrapper {
            flex: 1;
            padding-left: $spacing-2;
        }
    }

</style>