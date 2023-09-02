<script lang="ts">
    import { fly } from "svelte/transition";
    //@ts-ignore
    import MenuIcon from "~icons/la/angle-down";
    //@ts-ignore
    import MagnifyIcon from "~icons/mdi/magnify";
	import Dropdown from "../Dropdown.svelte";

    let open: boolean = false;

    const handleClick = () => {
        open = !open;
    }
</script>

<div>
    <div class="workspace" on:click={handleClick}>
        <div class="content">
            <img class="image" src="https://picsum.photos/200/200" />
            <div class="information">
                <span class="sub">Workspace</span>
                <span class="name">Justin's Workspace</span>
            </div>
        </div>
        <div class="icon">
            <MenuIcon />
        </div>
        
    </div>

    <Dropdown {open} transition={fly} offset={4} placement="bottom-start">
        <div class="dropdown">
            <div class="search">
                <div class="icon">
                    <MagnifyIcon />
                </div>
                <input name="search" class="input" placeholder="Search for a workspace" />
            </div>

            <div class="workspaces-list">
                {#each Array(5) as _}
                    <div class="list-workspace active">
                        <img class="image" src="https://picsum.photos/200/200" />
                        <div class="list-workspace-information">
                            <span class="name">Justin's Workspace</span>
                        </div>
                    </div>
                {/each}
            </div>

            <div class="workspaces-actions">
                <span class="text">Create New Workspace</span>
            </div>
        </div>
    </Dropdown>
</div>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .workspace {
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 100%;
        max-height: 80px;
        border-radius: $spacing-1;
        padding: $spacing-3;
        background: $color-page;
        border: 1px solid $color-light-border;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
        user-select: none;

        &:hover {
            border-color: $color-light-border-hover;
        }

        .content {
            display: flex;
            align-items: center;
        }

        .image {
            width: 40px;
            height: 40px;
            border-radius: 25%;
        }

        .information {
            display: flex;
            flex-direction: column;
            height: 100%;
            padding-left: $spacing-2-half;
            padding-right: $spacing-2-half;
            padding-top: $spacing-half;
            padding-bottom: $spacing-half;
            font-weight: 500;

            .sub {
                color: $color-faint-text;
                font-size: 14px;
            }

            .name {
                display: block;
                font-size: 16px;
                margin-top: $spacing-half;
                text-overflow: ellipsis;
                overflow: hidden;
                white-space: nowrap;
                max-width: 100px;
            }
        }

        .icon {
            display: flex;
            font-size: 20px;
        }
    }

    .dropdown {
        min-width: 360px;

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
                padding-top: $spacing-2;
                padding-bottom: $spacing-2;
                padding-left: $spacing-2;
                width: 100%;
                border-top-right-radius: $spacing-2;
                border-bottom-right-radius: $spacing-2;
                cursor: pointer;

                .image {
                    width: 35px;
                    height: 35px;
                    border-radius: 25%;
                }

                .name {
                    font-size: 15px;
                    padding-left: $spacing-1-half;
                }
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