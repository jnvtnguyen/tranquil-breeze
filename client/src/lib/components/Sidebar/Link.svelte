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
        position: relative;
        display: flex;
        align-items: center;
        color: $color-faint-text;
        text-decoration: none;
        padding: $spacing-1-half;
        padding-top: $spacing-1;
        padding-bottom: $spacing-1;
        border-left: 2px solid transparent;
        transition: all .2s;

        &:not(:first-child) {
            margin-top: $spacing-half;
        }

        &:hover:not(.active) {
            background: $color-light-grey;
            border-left-color: $color-primary;
        }

        .icon {
            display: flex;
            position: relative;
            align-items: center;
            padding: $spacing-half;
            color: $color-faint-text;
            border-radius: 25%;
        }

        .title {
            margin-left: $spacing-1-half;
            z-index: 1000;
        }

        &.active {
            border-left: 2px solid $color-primary;
            background: $color-light-grey;

            .icon {
                background: $color-primary;
                color: $color-inverse-text;
            }
        }
    }
</style>
