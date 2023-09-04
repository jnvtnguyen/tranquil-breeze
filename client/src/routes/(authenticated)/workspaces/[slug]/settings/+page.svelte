<script lang="ts">
    import type { PageData } from "./$types";
	import { PageMeta, Tabs, Tab } from "$lib/components/";
    import TeamTab from "$lib/components/Workspace/Settings/TeamTab.svelte";

    export let data: PageData;
</script>

<PageMeta title="Settings" />
<h2>Workspace Settings</h2>

<Tabs>
    <Tab open title="General">

    </Tab>
    <Tab title="Team">
        <TeamTab workspace={data.workspace} />
    </Tab>
</Tabs>