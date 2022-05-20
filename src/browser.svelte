<script lang="ts">
	import { copy } from './ts/copy';
	import { DialogContent } from 'svelte-dialogs';
    import { Circle2 } from 'svelte-loading-spinners';
	import { copy_fs, move_fs } from './ts/io';
	const ENDPOINT = 'http://localhost:8000/getfiles';
	export let ls: Array<object>, path: string, current_name: string, url: string;
    let spinner: number;
    let old_name: string = path + current_name;
    let only_name = current_name;
	function getfile(path: string) {
        spinner = 1;
		const xhr = new XMLHttpRequest();
		xhr.open('POST', ENDPOINT, true);
		xhr.onreadystatechange = function () {
			if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
				ls = JSON.parse(this.response);
                spinner = 0;
			}
		};
		xhr.send(JSON.stringify({ folder: path }));
	}
	async function test(e: string) {
        spinner = 1;
		path = path + e;
		current_name = e;
		getfile(path);
	}
</script>

<DialogContent>
	<h1 slot="header">Select the path where you want to move file</h1>
	<svelte:fragment slot="body">
        {#if spinner == 1}
            <div align="center">
                <Circle2 size="128" unit="px"/>
                <p>I'm getting file from server, please wait</p>
            </div>
        {:else}
            {#each ls as folder}
                {#if folder.md5 == 'dir' && folder.file.split('.')[1] == null}
                    <p on:click={() => test(folder.file + '/')}>{folder.file}</p>
                {/if}
            {/each}
            <p>
                <button on:click={copy_fs(old_name, path + only_name, "http://localhost:8000/copy")}
                    >Copy the file here</button
                >
            </p>
            <p>
                <button on:click={move_fs(old_name, path + only_name, "http://localhost:8000/move")}
                    >Move the file here</button
                >
            </p>
    {/if}
	</svelte:fragment>
</DialogContent>

<style>
	button {
		color: red;
	}
</style>
