<script lang="ts">
	import type { HTMLInputTypeAttribute } from "svelte/elements";
    //@ts-ignore
    import AlertIcon from "~icons/mdi/alert";
    //@ts-ignore
    import CheckIcon from "~icons/mdi/check-circle";
    import { slide, crossfade } from "svelte/transition";
    import { quintOut } from "svelte/easing";
    import { createEventDispatcher } from "svelte";
    //@ts-ignore
    import { v4 as uuid } from "uuid";

    export let value: string | null = null;
    export let label: string | null = null;
    export let error: string | null = null;
    export let dirty: boolean | null = null;
    export let valid: boolean | null = null;
    export let required: boolean = false;
    export let type: HTMLInputTypeAttribute = "text";
    export let name: string;
    export let id: string = uuid();

    let input: HTMLInputElement;
    let focused: boolean = false;

    const dispatch = createEventDispatcher();

    const typeAction = (node: HTMLInputElement) => {
        node.type = type;
    }

    const handleFocus = () => {
        focused = true;
        dispatch("focus");
    }

    const handleBlur = () => {
        focused = false;
        dispatch("blur");
    }

    const handleClick = () => {
        input.focus();
    }

    const [send, recieve] = crossfade({
        duration: 500,
        easing: quintOut
    });
</script>

<div class="wrapper">
    {#if label}
        <label class="label" for={id}>
            {label}
            {#if required}
                <span class="required">*</span>
            {/if}
        </label>
    {/if}
    <div class="input-wrapper" class:error class:focused on:click={handleClick}>
        <input 
            {...$$restProps}
            class="input"
            id={id}
            name={name}
            bind:value 
            bind:this={input}
            on:focus={handleFocus}
            on:blur={handleBlur}
            use:typeAction
        />
        {#if error}
            <span class="suffix-icon" class:error in:send={{ key: "valid" }} out:recieve={{ key: "error"}}>
                <AlertIcon />
            </span>
        {:else if dirty}
            <span class="suffix-icon" class:valid in:send={{ key: "error" }} out:recieve={{ key: "valid" }}>
                <CheckIcon />
            </span>
        {/if}
    </div>
    {#if error}
        <p class="error-message" transition:slide>
            {error}
        </p>
    {/if}
</div>

<style lang="scss">
    @use "$lib/css/variables" as *;
    .wrapper {
        position: relative;
        margin-bottom: $spacing-2;
    } 

    .label { 
        position: relative; 
        left: 0;
        right: 0;
        top: 0;
        padding: 0;
        text-align: left;
        font-size: 12px;
        color: $color-faint-text;
        cursor: text;
    }

    .input-wrapper {
        display: flex;
        align-items: center;
        margin-top: $spacing-1; 
        height: 46px;
        width: 100%;
        margin-bottom: 0;
        padding-top: $spacing-2;
        padding-bottom: $spacing-2;
        padding-left: $spacing-1-half;
        padding-right: $spacing-1-half;
        border: 1px solid $color-border;
        border-radius: $spacing-half;
        background-color: #efefef3d;
        cursor: text;
        transition: all 0.2s;

        &:hover {
            border-color: $color-border-hover;
        }

        &.focused:not(.error) {
            border-color: $color-border-focus;
        }

        &.error {
            border-color: $color-danger;
        }
    }

    .input {
        flex: 1;
        font-size: 14px;
        border: 0;
        outline: none;
        background: transparent;
        user-select: none;

        &::placeholder {
            color: $color-faint-text;
            opacity: 0.7;
        }
    }

    .required {
        font-size: 16px;
        color: $color-danger;
    }

    .error-message {
        height: auto;
        margin-top: $spacing-1;
        margin-bottom: 0;
        font-size: 14px;
        color: $color-danger;
    }

    .suffix-icon {
        display: flex;
        align-items: center;
        &.error {
            color: $color-danger;
        }

        &.valid {
            color: $color-success;
        }
    }
</style>