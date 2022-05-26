<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { open } from '@tauri-apps/api/dialog';
    import Circle2 from 'svelte-loading-spinners/dist/ts/Circle2.svelte';
    import { dialogs } from 'svelte-dialogs';
    let loading: boolean = false;
    async function upload(){
        let filename = await open();
        loading = true;
        let temp = filename.split("/");
        invoke('ftp_put', {url:"", filesToUpload: filename, filename: temp[temp.length-1]}).then(()=>{
            dialogs.alert("File successfully transfered");
            loading = false
        })
    }
</script>
<svelte:head>
</svelte:head>
<main>
    <center>
        <button class="uk-button-primary" on:click={upload}>Select a file</button>
    </center>
    {#if loading == true}
    <div align="center">
        <Circle2 size=120 unit="px" duration="1s"></Circle2>
        <p>I'm uploading file</p>
    </div>
    {/if}
</main>