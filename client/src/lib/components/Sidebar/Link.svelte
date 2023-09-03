<script lang="ts">
    import { page } from "$app/stores";

    export let title: string;
    export let href: string;

    $: active = $page.url.pathname === href;
</script>

<a class="link" {href} class:active>
    {#if $$slots.icon}
        <div class="icon">
            <slot name="icon" />
        </div>
    {/if}
    <span class="title">{title}</span>
</a>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .link {
        display: flex;
        align-items: center;
        color: $color-faint-text;
        text-decoration: none;
        padding: $spacing-1-half;

        .icon {
            display: flex;
            align-items: center;
            padding: $spacing-half;
            border-radius: 25%;
            color: $color-inverse-text;
        }

        .title {
            margin-left: $spacing-1-half;
        }

        &.active {
            background: $color-light-grey;
            border-left: 2px solid $color-primary;

            .icon {
                background: $color-primary;
            }
        }

    }
</style>
