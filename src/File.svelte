<script lang="ts">
    import VirtualList from 'svelte-tiny-virtual-list';
    import { onMount } from 'svelte';
    import { load_files } from './ts/io';
    import Contex from './contex.svelte';
    import { rightClick, hideMenu } from './ts/menu';
    let files: Array<object> = [];
    export let url: string;
    onMount(async () => {
        files = await load_files(url);
    });
    function contex(e) {
		let only_file = rightClick(e);
		//let current_file = 'http://localhost:8000' + (path.replace('.', '') + rightClick(e));
	}
</script>
<Contex />
<VirtualList
    width="100%"
    height={600}
    itemCount={files.length}
    itemSize={64}>
  <div slot="item" let:index let:style {style}>
    {#if files[index].md5 != "dir"}
    <img src="/images/file.png" alt="file" width="64px"/>
    {:else}
    <img src="/images/folder.png" alt="folder" width="64px"/>
    {/if}
    {files[index].file}
  </div>
</VirtualList>