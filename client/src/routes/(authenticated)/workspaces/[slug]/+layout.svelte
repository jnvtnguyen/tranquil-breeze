<script lang="ts">
    import {Sidebar, Navbar} from "$lib/components/";
</script>

<div class="container">
    <Navbar />
    <div class="content">
        <Sidebar />
        <div class="main">
            <slot />
        </div>
    </div>
</div>

<style lang="scss">
    @use "$lib/css/variables" as *;
    .container {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
    }

    .content {
        display: flex;
        width: 100%;
        height: 100%;
    }

    .main {
        flex: 1 0 auto;
        height: 100%;
        background: $color-page;
        padding: $spacing-4;
    }
</style>