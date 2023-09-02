<script lang="ts">
    import {Sidebar} from "$lib/components/";
</script>

<div class="content">
    <Sidebar />
    <div class="main">
        <slot />
    </div>
</div>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .content {
        display: flex;
        width: 100%;
        height: 100%;
    }

    .main {
        flex: 1 0 auto;
        height: 100%;
        background: $color-page;
    }
</style>