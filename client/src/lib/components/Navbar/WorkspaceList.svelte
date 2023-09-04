<script lang="ts">
    import { Svroller } from "svrollbar";
    import { page } from "$app/stores";
    import { Spinner } from "$lib/components";

    const fetchWorkspaces = async () => {
        return await fetch("/api/workspaces")
            .then((response) => response.json())
            .then((data) => {
                return data.workspaces;
            })
    }

    $: current = /\/[^/]*\/[^/]*/.exec($page.url.pathname)[0];
</script>

<div class="workspace-list">
    {#await fetchWorkspaces()}
        <div class="loading">
            <Spinner />
        </div>
    {:then workspaces}
        <Svroller width="100%" height="320px">
            <div class="inner-workspaces">
                {#each workspaces as workspace, i (workspace)}
                    <a class="list-workspace" class:active={current === `/workspaces/${workspace.slug}`}  href="/workspaces/{workspace.slug}">
                        <img class="image" src="https://picsum.photos/200/200" />
                        <div class="list-workspace-information">
                            <span class="name">{workspace.name}</span>
                        </div>
                    </a>
                {/each}
            </div>
        </Svroller>
    {:catch error}
    {/await}
</div>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .workspace-list {
        display: flex;
        flex-direction: column;
        border-top: 1px solid $color-border;
        border-bottom: 1px solid $color-border;
    }

    .list-workspace {
        display: flex;
        align-items: center;
        padding-top: $spacing-1;
        padding-bottom: $spacing-1;
        padding-left: $spacing-2;
        width: 100%;
        cursor: pointer;
        border-left: 0;
        border-left: 0;
        text-decoration: none;
        color: $color-text;
        transition: all .2s;
        &.active {
            background: $color-primary;
            color: $color-inverse-text;
        }

        .image {
            width: 35px;
            height: 35px;
            border-radius: 25%;
        }

        .name {
            font-size: 15px;
            padding-left: $spacing-1-half;
        }


        &:hover:not(.active) {
            background: $color-light-grey;
        }
    }

    .error {
        padding-left: $spacing-2;
    }
</style>