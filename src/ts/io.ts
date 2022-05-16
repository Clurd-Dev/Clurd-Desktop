import { invoke } from '@tauri-apps/api/tauri';
export async function parse_config(){
    try {
        return JSON.parse(await invoke('parse_config'));
    } catch (error) {
        return undefined
    } 
};

export async function load_files(url:string){
    return JSON.parse(await invoke('getfiles',{ invokeMessage: url + '/getfiles', path: "./" }));
}