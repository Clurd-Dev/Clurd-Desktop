<script lang="ts">

    import { onMount } from 'svelte';
    import { load_files } from './ts/io';
    import { get_config } from './ts/io';
    import { rightClick, hideMenu } from './ts/menu';
	import Contex from './context/Context.svelte';
	import { dialogs } from 'svelte-dialogs';

	export let url: string;
	let items: Array<object> = [], only_file: string, current_file: string, path:string;

    function contex(e) {
		only_file = rightClick(e);
		current_file = url + (path.replace('.', '') + rightClick(e));
	}

	async function change_folder(path_dst:string){
		path = path + path_dst;
		items = await load_files(url, path);
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
				items = await load_files(url, path);
			}
		}
	}
	
    onMount(async () => {
        console.log(url)
        path = await get_config(url);
		items = await load_files(url, path);
        document.onclick = hideMenu;
    });
</script>

<Contex file={only_file} url={current_file} baseurl={url} on:rename={async () => items = await load_files(url,path)} on:remove={async () => items = await load_files(url,path)}/>

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
	<!-- <input type="file" name="dummyname" id="inputdata" />
	<button on:click={upload}>Upload</button> -->
	<hr />
</section>

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

</style>

