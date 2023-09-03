<script lang="ts">
    export let href: string | null = null;
    export let text: string | null = null;
</script>

<a class="link" href={href}>{text}</a>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .link {
        color: $color-text;
        text-decoration: none;

        &:hover {
            text-decoration: underline;
        }
    }
</style>