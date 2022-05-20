import { invoke } from '@tauri-apps/api/tauri';
import { dialogs } from 'svelte-dialogs';
export async function parse_config(){
    try {
        return JSON.parse(await invoke('parse_config'));
    } catch (error) {
        return undefined
    } 
};

export async function load_files(url:string, path:string){
    return JSON.parse(await invoke('getfiles',{ invokeMessage: url + '/getfiles', path: path}));
}

export async function get_config(endpoint: string) {
	const ENDPOINT = endpoint + '/getconfig';
    try {
        return JSON.parse(await invoke('get_config', {url: ENDPOINT})).path;
    } catch (error) {
		console.log(error);
		dialogs.alert("Error during get path from server");
        return undefined;
    }
}

export async function get_information(endpoint: string) {
	const ENDPOINT = endpoint + 'getinfo';
	const request = await fetch(ENDPOINT);
	if (request.ok) {
		const json = await request.json();
		return json;
	} else {
		console.log('HTTP-Error: ' + request.status);
	}
}

export async function remove_fs( absolute: string) {
	const ENDPOINT = 'http://localhost:8000/remove';
    try {
        return JSON.parse(await invoke('remove_fs', {url: ENDPOINT, absolute: absolute}));
    } catch (error) {
		console.log(error);
		dialogs.alert("Error during removing of path from server");
        return undefined;
    }
}

export async function rename_fs(old: string, url: string, new_path: string){
	await invoke('rename_fs', {old: old, url: url, new:new_path});
}

export async function copy_fs(old_path: string, new_path: string, url: string){
    let response = await invoke('copy_fs', {url: url, old:old_path, new: new_path});
    if (response == 0) {
        dialogs.alert("Problem during copy of file");
        return 0;
    }else{
        dialogs.alert("File copied");
        return 1;
    }
}

export async function move_fs(old_path: string, new_path: string, url: string){
    let response = await invoke('move_fs', {url: url, old:old_path, new: new_path});
    if (response == 0) {
        dialogs.alert("Problem during copy of file");
        return 0;
    }else{
        dialogs.alert("File moved");
        return 1;
    }
}

