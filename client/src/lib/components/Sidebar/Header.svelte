<script lang="ts">
    export let title: string;
</script>

<div class="header">
    <span class="title">{title}</span>
</div>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .header {
        display: flex;
        align-items: center;
        padding: $spacing-1-half;
        margin-left: 2px;
        padding-bottom: $spacing-1;
        color: $color-faint-text;
        font-size: 0.8rem;
        font-weight: 600;
        text-transform: uppercase;
    }
</style>