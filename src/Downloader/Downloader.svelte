<script lang="ts">
    import { BarLoader } from 'svelte-loading-spinners';
    import { invoke } from '@tauri-apps/api/tauri';
    import { save } from '@tauri-apps/api/dialog';
    import { createDir } from '@tauri-apps/api/fs';
    import { dialogs } from 'svelte-dialogs';
    import { load_files } from '../ts/io';
    export let url: string, files: Array<string>, path: string;
    async function sync(){
        dialogs.modal(BarLoader);
        let folder_stack = [];
        let folder_download: any = await save();
        if(folder_download != undefined){
            folder_download = folder_download.split("/");
            folder_download.pop();
            files.forEach(async(file) => {
                let response;
                if(file.dir == false)
                    response = await invoke('download', {url: url + "/" + file.file, path: file.file, savefolder: folder_download.join('/')});
                else{
                    folder_stack.push("/" + file.file);
                    createDir(folder_download.join('/') + "/" + file.file);
                }
            })
                for(let i = 0; i < folder_stack.length; i++) {
                    let files_new: Array<object>;
                    console.log(folder_stack[i]);
                    console.log(folder_stack.length);
                    let response: string;
                    files_new = await load_files(url, path + folder_stack[i]);
                    for(let k = 0; k < files_new.length; k++ ){
                        if(files_new[k].dir == false)
                            response = await invoke('download', {url: url + folder_stack[i] + "/" + files_new[k].file, path: files_new[k].file, savefolder: folder_download.join('/') + folder_stack[i]});
                        else{
                            createDir(folder_download.join('/') + folder_stack[i] + "/" + files_new[k].file);
                            folder_stack.push(folder_stack[i] + "/" + files_new[k].file);
                            console.log(folder_stack);
                        }
                       
                    } 
                };
            dialogs.alert("Folder synced successfuly");
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