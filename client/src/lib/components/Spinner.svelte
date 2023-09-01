<script lang="ts">
    export let size: string = "16px";

    $: style = `width: ${size}; height: ${size}`;
</script>

<span class="loader" {style}></span>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .loader {
        display: inline-block;
        border: 2px solid $color-grey; 
        border-bottom-color: $color-white;
        border-radius: 50%;
        animation: rotation 1s linear infinite;
    }

    @keyframes rotation {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
</style>