<script lang="ts">
	import { rename_fs } from '../ts/io';
	export let file: string, url: string, baseurl: string;
	import './contex.css';
	import { remove_fs, rename_fs } from '../ts/io';
	import { dialogs } from 'svelte-dialogs';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

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
	function rename(){
		let full_path = "./" + url.replace(baseurl + "/", "");
		dialogs.prompt("What is the new name of " + file + "? IMPORTANT: Include the extension of file").then((msg) => {
			if(msg[0] == undefined){
				dialogs.alert("Please write a correct name for this file.");
			}else{
				let new_path = full_path.replace(file, msg[0]);
				rename_fs(full_path, baseurl + "/rename", new_path).then(() => {
					dialogs.alert("The file is successfully renamed").then(()=>{
						dispatch('rename', {
								text: ''
							});
					})
				});
			}
		});
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
			<a href="#0" on:click={rename(file)}><i class="fa fa-rename" aria-hidden="true" /> Rename</a>
		</li>
	</ul>
</div>
