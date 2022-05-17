import { invoke } from '@tauri-apps/api/tauri';
import { dialogs } from 'svelte-dialogs';
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

export async function remove_fs(path: string, refresh, absolute: string) {
	const ENDPOINT = 'http://localhost:8000/remove';
    try {
        return JSON.parse(await invoke('remove_fs', {url: ENDPOINT, absolute: absolute}));
    } catch (error) {
		console.log(error);
		dialogs.alert("Error during removing of path from server");
        return undefined;
    }
	// const xhr = new XMLHttpRequest();
	// xhr.open('POST', ENDPOINT, true);
	// xhr.onreadystatechange = async function () {
	// 	if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
	// 		checkresponse(this.responseText, refresh, absolute);
	// 	}
	// };
	// xhr.send(JSON.stringify({ folder: path }));
}
// async function checkresponse(response: string, refresh, absolute) {
// 	if (response == '1') {
// 		let confirm;
// 		do {
// 			confirm = await dialogs.alert('File deleted successfully');
// 		} while (confirm);
// 		refresh(absolute);
// 	} else {
// 		let confirm;
// 		do {
// 			confirm = await dialogs.alert(
// 				'File deleted unsuccessfully, check server log to see error and if you think this is a bug open an issue on github.'
// 			);
// 		} while (confirm);
// 	}
// }
// export async function copyfs(old_file: string, new_file: string) {
// 	const ENDPOINT = 'http://localhost:8000/copy';
// 	const xhr = new XMLHttpRequest();
// 	xhr.open('POST', ENDPOINT, true);
// 	xhr.onreadystatechange = async function () {
// 		if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
// 			if (this.responseText == '1')
// 				dialogs.alert('File successfully copied').then(() => location.reload());
// 			else dialogs.alert('File unsuccessfully copied, check the error in log of server or retry');
// 		}
// 	};
// 	xhr.send(JSON.stringify({ folder: old_file, new: new_file }));
// }

// export async function movefs(old_file: string, new_file: string) {
// 	const ENDPOINT = 'http://localhost:8000/move';
// 	const xhr = new XMLHttpRequest();
// 	xhr.open('POST', ENDPOINT, true);
// 	xhr.onreadystatechange = async function () {
// 		if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
// 			if (this.responseText == '1')
// 				dialogs.alert('File successfully moved').then(() => location.reload());
// 			else dialogs.alert('File unsuccessfully moved, check the error in log of server or retry');
// 		}
// 	};
// 	xhr.send(JSON.stringify({ folder: old_file, new: new_file }));
// }


