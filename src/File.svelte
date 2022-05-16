<script lang="ts">
    import { load_files } from './ts/io';
    import { onMount } from 'svelte';
    let files: Array<object> = [];
    export let url: string;
    onMount(async () => {
        files = await load_files(url);
    });
</script>

{#each files as file_info}
    <p>{file_info.file}</p>
{/each}