<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { save } from '@tauri-apps/api/dialog';
</script>
<svelte:head>
</svelte:head>
<main>
    <p>Select the folder where you want to download files</p>
    <button on:click={() => save().then(msg => console.log(msg))}>Save file</button>
</main>