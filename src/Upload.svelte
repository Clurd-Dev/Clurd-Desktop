<script lang="ts">
	import { Stretch } from 'svelte-loading-spinners';
    import { onMount } from 'svelte';
    import { load_files } from './ts/io';
    import { get_config } from './ts/io';
	import Uploader from './Uploader/Uploader.svelte';
	import { dialogs } from 'svelte-dialogs';
	import { invoke } from '@tauri-apps/api/tauri';
	import "uikit/dist/css/uikit.css";
	import "uikit/dist/js/uikit.js";
	import Downloader from './Downloader/Downloader.svelte';
	export let url: string;
	let items: Array<object> = [], path:string, spinner: number = 1;
	async function change_folder(path_dst:string){
		path = path + path_dst;
		spinner = 1;
		load_files(url, path).then((resp)=>{
			spinner = 0;
			items = resp;
		});
	}

	async function goback() {
		if (path == './') {
			dialogs.alert("Can't go back through home");
		} else {
			let tempath = path.split('/');
			tempath.pop();
			path = tempath.join('/');
			if (path == '.') {
				path += '/';
				load_files(url, path).then((resp) => {
					items = resp;
				});
			}else{
				spinner = 1;
				load_files(url, path).then((resp)=>{
					items = resp;
					spinner = 0;
				});
			}
		}
	}

    onMount(async () => {
        path = await get_config(url);
		load_files(url, path).then((response) => {
			items = response;
			spinner = 0;
		});
		let temp = JSON.parse(await invoke('get_space', {url: url + '/space', path: path}));
    });
</script>


{#if spinner == 1}
	<div align="center" class="spinner">
		<Stretch size="128" color="#FF3E00" unit="px" duration="2s"/>
		<p>I'm getting file from server, please wait</p>
	</div>
{:else}
<hr/>
<div align="center">
    <h1>Sync files</h1>
    <a class="uk-button uk-button-primary" href="#0" on:click={()=> dialogs.modal(Downloader, {})}>Sync files from this directory</a>
    <hr/>
    <a class="uk-button uk-button-primary" href="#0" on:click={() => dialogs.modal(Uploader, { name: "world" })}>Upload files here</a>
</div>
<hr/>
<section>
	<img src="/images/back.png" alt="back" on:click={goback}/>
	<div class="grid-container" align="center">
		{#each items as lsraw}
			{#if lsraw.dir == true}
				<div
					class="grid-item"
					on:click={() => change_folder(lsraw.file + '/')}
					id={lsraw.file}
					align="center"
				>
					<div id={lsraw.file}>
						<img src="/images/folder.png" class="icon" alt="folder" id={lsraw.file} />
					</div>
					<p>{lsraw.file}</p>
				</div>
			{:else}
				<div
					class="grid-item"
					id={lsraw.file}
					align="center"
				>
					<div id={lsraw.file}>
						{#if lsraw.image}
							<img src="/images/image.png" alt="fileimg" class="icon" id={lsraw.file}/>
						{:else if lsraw.video}
							<img src="/images/video.png" alt="filevideo" class="icon" id={lsraw.file}/>
						{:else if lsraw.audio}
							<img src="/images/audio.png" alt="fileaudio" class="icon" id={lsraw.file}/>
						{:else if lsraw.file.split('.')[1] == 'pdf'}
							<img src="/images/pdf.png" alt="filepdf" class="icon" id={lsraw.file}/>
						{:else}
							<img src="/images/file.png" alt="file" class="icon" id={lsraw.file}/>
						{/if}
						<p id={lsraw.file}>{lsraw.file}</p>
					</div>
				</div>
			{/if}
		{/each}
	</div>
	<br />
</section>
<br />
{/if}



<style>
  .grid-container {
	display: grid;
	grid-template-columns: auto auto auto;
	padding: 10px;
}
.grid-item {
	border: 1px solid rgba(0, 0, 0, 0.8);
	padding: 20px;
	font-size: 30px;
	text-align: center;
}
.icon {
	width: 128px;
}

.space {
		margin: 20px;
	}

.spinner{
	padding: 70px 0;
	text-align: center;
}
</style>
