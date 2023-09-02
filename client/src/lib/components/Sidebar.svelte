<script lang="ts">
	import Workspace from "./Sidebar/Workspace.svelte";
</script>

<div class="sidebar">
    <Workspace />
</div>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .sidebar {
        display: flex;
        padding: $spacing-4;
        width: 320px;
        height: 100%;
        background: $color-white;
        border-right: 1px solid $color-light-border;
    }
</style>