<script lang="ts">
	import type { Workspace } from "$lib/types";
    import { navigating, page } from "$app/stores";
    //@ts-ignore
    import MagnifyIcon from "~icons/mdi/magnify";
    //@ts-ignore
    import MenuDownIcon from "~icons/mdi/menu-down";
    //@ts-ignore
    import MenuUpIcon from "~icons/mdi/menu-up";
    import { Dropdown, Spinner } from "$lib/components";
	import WorkspaceModal from "./WorkspaceModal.svelte";

    export let workspace: Workspace;

    const fetchWorkspaces = async () => {
        return await fetch("/api/workspaces")
            .then((response) => response.json())
            .then((data) => {
                return data.workspaces;
            })
    }

    const handleOpen = () => {
        modal = true;
    }

    let openWorkspace: boolean = false;
    let modal: boolean = false;
</script>

<div>
    <div class="workspace">
        <div class="content">
            <img class="image" src="https://picsum.photos/200/200" />
            <span class="name">{workspace.name}</span>
        </div>
        <div class="icon">
            {#if !$navigating}
                {#if openWorkspace}
                    <MenuUpIcon />
                {:else}
                    <MenuDownIcon />
                {/if}
            {/if}
        </div>
    </div>

    <Dropdown bind:open={openWorkspace} offset={4} placement="bottom-start">
        <div class="dropdown">
            <div class="search">
                <div class="icon">
                    <MagnifyIcon />
                </div>
                <input autocomplete="off" name="search" class="input" placeholder="Search for a workspace" />
            </div>

            <div class="workspaces-list">
                {#await fetchWorkspaces()}
                    <div class="loading">
                        <Spinner />
                    </div>
                {:then workspaces}
                    {#each workspaces as workspace}
                        <a class="list-workspace" class:active={$page.url.pathname.includes(workspace.slug)}  href="/workspaces/{workspace.slug}">
                            <img class="image" src="https://picsum.photos/200/200" />
                            <div class="list-workspace-information">
                                <span class="name">{workspace.name}</span>
                            </div>
                        </a>
                    {/each}
                {:catch error}
                {/await}
            </div>

            <div class="workspaces-actions">
                <span class="text" on:click={handleOpen}>Create New Workspace</span>
            </div>

            <WorkspaceModal bind:open={modal} />
        </div>
    </Dropdown>
</div>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .workspace {
        display: flex;
        align-items: center;
        justify-content: space-between;
        background: $color-primary;
        padding: $spacing-1;
        border-radius: $spacing-1;
        cursor: pointer;
        width: 208px;
        color: $color-inverse-text;
        box-shadow: rgba(60, 64, 67, 0.3) 0px 1px 2px 0px, rgba(60, 64, 67, 0.15) 0px 2px 6px 2px;

        .content {
            display: flex;
            align-items: center;

            .image {
                width: 30px;
                height: 30px;
                border-radius: 25%;
                box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;
            }

            .name {
                display: block;
                font-size: 15px;
                padding-left: $spacing-1-half;
                text-overflow: ellipsis;
                white-space: nowrap;
                overflow: hidden;
                width: 125px;
            }
        }

        .icon {
            display: flex;
            align-items: center;
            padding-left: $spacing-1-half;
            font-size: 18px;
        }
    }

    .dropdown {
        min-width: 360px;

        .loading {
            display: flex;
            align-items: center;
            justify-content: center;
            padding: $spacing-2;
        }

        .search {
            display: flex;
            align-items: center;
            border-bottom: 1px solid $color-light-border;
            color: $color-faint-text;

            .input {
                flex: 1;
                border: 0;
                width: 100%;
                height: 100%;
                padding: $spacing-2;
                padding-left: $spacing-5;
                font-size: 14px;
                outline: none;
                background: transparent;
            }

            .icon {
                position: absolute;
                display: flex;
                align-items: center;
                font-size: 18px;
                padding-left: $spacing-1;
            }
        }

        .workspaces-list {
            display: flex;
            flex-direction: column;
            padding-left: 0;

            .list-workspace {
                display: flex;
                align-items: center;
                padding-top: $spacing-1;
                padding-bottom: $spacing-1;
                padding-left: $spacing-2;
                width: 100%;
                cursor: pointer;
                border-top: 1px solid $color-white;
                border-bottom: 1px solid $color-white;
                text-decoration: none;
                color: $color-text;

                &.active {
                    background: $color-primary;
                    border-top: 1px solid $color-border;
                    border-bottom: 1px solid $color-border;
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
                    border-top: 1px solid $color-border;
                    border-bottom: 1px solid $color-border;
                }
            }
            .error {
                padding-left: $spacing-2;
            }
        }

        .workspaces-actions {
            border-top: 1px solid $color-light-border;
            padding: $spacing-2;
            font-size: 14px;
            
            .text {
                cursor: pointer;
            }
        }
    }
</style>