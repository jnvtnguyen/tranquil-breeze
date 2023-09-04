<script lang="ts" context="module">
    import { writable, type Writable } from "svelte/store";

    export interface TabContext {
        selected: Writable<HTMLElement>;
        button: Writable<HTMLElement>;
    }
</script>

<script lang="ts">
    import { setContext } from "svelte";

    const context: TabContext = {
        selected: writable<HTMLElement>(),
        button: writable<HTMLElement>()
    }

    setContext("context", context);

    const init = {
        content: (node: HTMLElement) => {
            const destroy = context.selected.subscribe((e: HTMLElement) => {
                if(e) node.replaceChildren(e);
            });

            return { destroy };
        },
        indicator: (node: HTMLElement) => {
            const destroy = context.button.subscribe((e: HTMLElement) => {
                if(e) {
                    const { left, width } = e.getBoundingClientRect();
                    const parent = node.parentElement.getBoundingClientRect();
                    node.style.left = `${left - parent.left}px`;
                    node.style.width = `${width}px`;
                }
            });

            return { destroy };
        },
    }
</script>

<div class="container">
    <div class="scroller">
        <ul class="tabs">
            <slot />
        </ul>
        <span class="indicator" use:init.indicator />
    </div>
</div>
<div class="content" role="tabpanel" use:init.content />

<style lang="scss">
    @use "$lib/css/variables" as *;

    .container {
        position: relative;
        display: flex;
        border-bottom: 1px solid $color-border;
    }

    .scoller {
        position: relative;
        display: inline-block;
        flex: 1 1 auto;
        white-space: nowrap;
        width: 100%;
    }

    .tabs {
        display: flex;
        position: relative;
        flex-direction: row;
        list-style: none;
        margin: 0;
        padding: 0;
    }

    .indicator {
        position: absolute;
        height: 2px;
        bottom: 0;
        transition: all 300ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
        background: $color-primary;
    }
</style>