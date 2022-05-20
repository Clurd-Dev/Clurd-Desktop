<script lang="ts">
	import { Router, Link, Route } from "svelte-navigator";
	import { Stretch } from 'svelte-loading-spinners'
    import { onMount } from 'svelte';
    import { load_files } from './ts/io';
    import { get_config } from './ts/io';
    import { rightClick, hideMenu } from './ts/menu';
	import Contex from './context/Context.svelte';
	import { dialogs } from 'svelte-dialogs';
	import { invoke } from '@tauri-apps/api/tauri';
	import "uikit/dist/css/uikit.css"
	export let url: string;
	let items: Array<object> = [], only_file: string, current_file: string, path:string, available: string = "", total: string = "", spinner: number = 1;

    function contex(e) {
		only_file = rightClick(e);
		current_file = url + (path.replace('.', '') + rightClick(e));
	}

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
		available = (parseFloat((parseInt(temp.total) / 1000000000).toFixed(3)) -parseFloat((parseInt(temp.available) / 1000000000).toFixed(3))).toFixed(3);
		total = (parseInt(temp.total) / 1000000000).toFixed(3);
        document.onclick = hideMenu;
    });
</script>
<Contex file={only_file} url={current_file} baseurl={url} ls={items} current_path={path} on:rename={async () => items = await load_files(url,path)} on:remove={async () => items = await load_files(url,path)}/>
{#if spinner == 1}
	<div align="center" class="spinner">
		<Stretch size="128" color="#FF3E00" unit="px" duration="2s"/>
		<p>I'm getting file from server, please wait</p>
	</div>
{:else}
<section>
	<img src="/images/back.png" alt="back" on:click={goback}/>
	<img src="/images/refresh.png" alt="refresh" on:click={async () => items = await load_files(url,path)}/>
	<div class="grid-container" on:contextmenu={contex} align="center">
		{#each items as lsraw}
			{#if lsraw.md5 == 'dir'}
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
					on:contextmenu={contex}
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
<footer>
	<div align="center">
		<p>Space in use: {available} GB of {total} GB</p>
	</div>
	<div class="space">
		<progress class="uk-progress" value={available} max={total} />
	</div>
</footer>
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
