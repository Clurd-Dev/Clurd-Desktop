<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import Circle2 from 'svelte-loading-spinners/dist/ts/Circle2.svelte';
    import '../../public/js/reader_thread.js';
    import { dialogs } from 'svelte-dialogs';
    let loading: boolean = false, bytes: Array<number>;
    onMount(()=>{
        const input = document.querySelector('input');
        input.addEventListener('change', async (file) => {
        const file_object = file.target.files[0]
        loading = true
        var myWorker = new Worker('/js/reader_thread.js');
        myWorker.addEventListener('message', function(e) {
            dialogs.confirm("Are you sure to upload this file?",{}).then((msg) => {
                loading = false;
                if(msg == true){
                    bytes = e.data;
                    loading = false;
                    invoke('ftp_put', {url:"", filesToUpload: file_object.name, bytes: Array.from(bytes)}).then(()=> dialogs.alert("File successfully transfered"));
                }

            });
          
        }, false);
        myWorker.postMessage(file_object)
        })
    });
</script>
<svelte:head>
</svelte:head>
<main>
    <label for="myfile">Select files:</label>
    <input type="file" id="files" name="files" multiple><br><br>
    {#if loading == true}
    <div align="center">
        <Circle2 size=120 unit="px" duration="1s"></Circle2>
        <p>I'm processing the file</p>
    </div>
    {/if}
</main>