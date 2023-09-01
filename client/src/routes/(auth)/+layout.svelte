<script></script>

<main class="main">
    <div class="container">
        <slot />
    </div>
</main>

<style lang="scss">
    @use '$lib/css/spacing' as *;

    .main {
        display: flex;
        width: 100%;
        height: 100%;
        align-items: center;
        justify-content: center;
    }

    .container {
        width: 480px;
        padding-bottom: $spacing-10;
    }
</style>