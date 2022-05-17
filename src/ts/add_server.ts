import { dialogs } from "svelte-dialogs";
import { invoke } from '@tauri-apps/api/tauri';
export function add(){
    dialogs.prompt("Insert the Clurd Server URL").then((url_from_form) => {
        console.log(url_from_form);
        if (url_from_form != undefined){
            invoke('update_config', { invokeMessage: url_from_form[0] });
            return  url_from_form[0]
        }
    });
    return "0"
}
