<script lang="ts">
	export let file: string, url: string, baseurl: string, ls: Array<object>, current_path: string;
	import './contex.css';
	import { remove_fs, rename_fs } from '../ts/io';
	import { dialogs } from 'svelte-dialogs';
	import { createEventDispatcher } from 'svelte';
	import Details from '../details.svelte';
	import FSBrowser from '../browser.svelte';
	const dispatch = createEventDispatcher();
	const opts = {};
	async function rename(url: string, baseurl: string, file: string){
	    let full_path = "./" + url.replace(baseurl + "/", "");
	    dialogs.prompt("What is the new name of " + file + "? IMPORTANT: Include the extension of file").then((msg)=> {
	        if(msg[0] == undefined){
	            dialogs.alert("Please write a correct name for this file.");
	        }else{
	            let new_path = full_path.replace(file, msg[0]);
	            rename_fs(full_path, baseurl + "/rename", new_path).then(()=> {
	                dialogs.alert("The file is successfully renamed").then(()=>{
	                    dispatch('rename', {text: ''})
	                })
	            })
	        }
	    });
	}

	async function remove(absolute: string){
		dialogs.confirm("Are you sure to delete file " + file + "?").then((msg)=> {
			console.log(msg);
			if (msg == true){
				remove_fs(absolute);
				dialogs.alert("File successfully removed.").then(()=>{
					dispatch('remove');
				})
			}else{
				dialogs.alert("Operation cancelled");
			}

		})
	}
	async function download(){
		fetch(url)
		.then(resp => resp.blob())
		.then(blob => {
			const url = window.URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.style.display = 'none';
			a.href = url;
			a.download = file;
			document.body.appendChild(a);
			a.click();
			window.URL.revokeObjectURL(url);
			dialogs.alert('your file has downloaded!'); 
		})
		.catch(() => dialogs.alert("Can't download the file."));
	}
</script>

<svelte:head>
	<link
	rel="stylesheet"
	href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css"
/>
</svelte:head>
<div id="contextMenu" class="context-menu" style="display: none">
	<ul class="menu">
		<li class="remove">
			<a href="#0" on:click={remove(file)}><i class="fa fa-trash" aria-hidden="true" /> Remove</a>
		</li>
		<li class="rename">
			<a href="#0" on:click={rename(url, baseurl, file)}><i class="fa fa-rename" aria-hidden="true" /> Rename</a>
		</li>
		<li class="details">
			<a href="#0" on:click={dialogs.modal(Details, {ls: ls, name_file: url, path: "./" + url.replace(baseurl + "/", "") ,file_name: file})}><i class="fa fa-info"/>Details</a>
		</li>
		<li class="copy">
			<a href="#0" on:click={() =>
					dialogs.modal(FSBrowser, {
						ls: ls,
						path: current_path,
						current_name: file,
						url: url
					})}
					>
				<i class="fa fa-copy" aria-hidden="true" /> Copy/Move to
			</a>
		</li>
		<li class="download">
			<a href={url} download>
				<i class="fa fa-download" aria-hidden="true" /> Download this file
			</a>
		</li>
	</ul>
</div>
