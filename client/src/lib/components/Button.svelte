<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import Spinner from "./Spinner.svelte";

    export let text: string | null = null;
    export let type: "button" | "submit" | "reset" = "button";
    export let loading: boolean = false;

    const dispatch = createEventDispatcher();

    const handleClick = () => {
        dispatch("click");
    }
</script>

<button {...$$restProps} class="button" {type} on:click={handleClick}>
    <span class="text">
        {#if loading}
            <Spinner size="16px" />
        {:else}
            {text}
        {/if}
    </span>
</button>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .button {
        position: relative;
        display: inline-grid;
        justify-content: center;
        cursor: pointer;
        width: 100%;
        padding-top: $spacing-2;
        padding-bottom: $spacing-2;
        padding-left: $spacing-3;
        padding-right: $spacing-3;
        font-size: 15px;
        color: $color-inverse-text;
        border: 0;
        outline: 0;
        background-color: $color-primary;
        border-color: $color-primary;
        column-gap: $spacing-3;
        align-items: center;
        border-radius: $spacing-half;
        transition: background-color .2s;

        &:hover:not(:disabled) {
            background-color: $color-primary-hover;
            border-color: $color-primary-hover;
        }

        &:disabled {
            opacity: 0.7;
            cursor: not-allowed;
            transition: none;
        }
    }

    .text {
        display: flex;
        align-items: center;
        justify-content: center;
    }
</style>