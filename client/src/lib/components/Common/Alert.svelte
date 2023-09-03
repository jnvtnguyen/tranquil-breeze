<script lang="ts">
   export let text: string | null = null;
</script>

<div class="alert">
    <div class="text">
        {text}
    </div>
</div>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .alert {
        position: relative;
        display: flex;
        align-items: center;
        padding: $spacing-2;
        background-color: $color-white;
        color: $color-danger;
        border-left: 10px solid $color-danger;
        font-size: 14px;
        border-radius: $spacing-half;
        box-shadow: rgba(0, 0, 0, 0.12) 0px 1px 3px, rgba(0, 0, 0, 0.24) 0px 1px 2px;
    }
</style>