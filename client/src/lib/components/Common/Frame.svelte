<script lang="ts">
    import type { Action } from "svelte/action";
    import type { TransitionConfig } from "svelte/transition";

    const null_transition = () => ({ duration: 0 });
    const noop = () => {};

    type TransitionFunc = (node: HTMLElement, params: any) => TransitionConfig;

    export let tag: string = 'div';
    export let role: string | undefined = undefined;

    export let transition: TransitionFunc = null_transition;
    export let params: object = {};

    export let node: HTMLElement | undefined = undefined;
    export let use: Action<HTMLElement, any> = noop;
    export let options = {};
</script>

<svelte:element {...$$restProps} class="frame" this={tag} use:use={options} bind:this={node} transition:transition={params} on:click on:mouseenter on:mouseleave on:focusin on:focusout {role}>
    <slot />
</svelte:element>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .frame {
        background: $color-white;
        box-shadow: rgba(0, 0, 0, 0.16) 0px 10px 36px 0px, rgba(0, 0, 0, 0.06) 0px 0px 0px 1px;
        border-radius: $spacing-1-half;
    }
</style>