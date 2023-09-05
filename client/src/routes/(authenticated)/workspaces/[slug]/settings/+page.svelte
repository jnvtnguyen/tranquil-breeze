<script lang="ts">
	import { PageMeta, Tabs, Tab } from "$lib/components/";
    import TeamTab from "$lib/components/Workspace/Settings/TeamTab.svelte";
</script>

<PageMeta title="Settings" />
<h2>Workspace Settings</h2>

<Tabs>
    <Tab open title="General">

    </Tab>
    <Tab title="Team">
        <TeamTab />
    </Tab>
</Tabs>