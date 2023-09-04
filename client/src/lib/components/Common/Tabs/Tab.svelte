<script lang="ts">
    import type { TabContext } from "./Tabs.svelte";
    import { getContext } from "svelte";
    import { writable } from "svelte/store";

    export let open: boolean = false;
    export let title: string = "Title";

    let buttonElement: HTMLButtonElement;

    const context = getContext<TabContext>("context") ?? {};

    //@ts-ignore
    const selected = context.selected ?? writable<HTMLElement>();
    //@ts-ignore
    const button = context.button ?? writable<HTMLElement>();

    const init = {
        content: (node: HTMLElement) => {
            selected.set(node);

            const destroy = selected.subscribe((e: HTMLElement) => {
                button.set(buttonElement);

                if(e !== node) {
                    open = false;
                }
            });

            return { destroy };
        },
        button: (node: HTMLElement) => {
            if(open) {
                button.set(node);
            }
        }
    }
</script>

<li class="tab">
    <button type="button" class="button" class:open on:click={() => (open = true)} use:init.button bind:this={buttonElement} on:click>
        <slot name="title">{title}</slot>
    </button>

    {#if open}
        <div class="content">
            <div use:init.content>
                <slot />
            </div>
        </div>
    {/if}
</li>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .button {
        background: transparent;
        display: inline-flex;
        border: 0;
        outline: 0;
        max-width: 360px;
        min-width: 48px;
        flex-shrink: 0;
        padding: $spacing-1-half $spacing-2;
        justify-content: center;
        text-align: center;
        color: $color-faint-text;
        cursor: pointer;

        &.open {
            color: $color-primary;
        }
    }
</style>