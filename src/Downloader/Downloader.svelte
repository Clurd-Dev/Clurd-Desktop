<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { save } from '@tauri-apps/api/dialog';
    import { dialogs } from 'svelte-dialogs';
    export let current_path: string, url: string, files: Array<string>;
    async function sync(){
        console.log(current_path);
        let folder_download: any = await save();
        if(folder_download != undefined){
            folder_download = folder_download.split("/");
            folder_download.pop();
            files.forEach(async(file) => {
                console.log(url + "/" + file.file);
                let response = await invoke('download', {url: url + "/" + file.file, path: file.file, savefolder: folder_download.join('/')});
                console.log(response);
            })
        }else{
            dialogs.alert("Please select a correct folder to download");
        }
     
         
    }
</script>
<svelte:head>
</svelte:head>
<main>
    <p>Select the folder where you want to download files</p>
    <center>
        <button on:click={sync} class="uk-button-primary">Save file</button>
    </center>
</main>