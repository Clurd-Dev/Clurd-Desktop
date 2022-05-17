<script lang="ts">
    import { onMount } from 'svelte';
    import { load_files } from './ts/io';
    import { get_config } from './ts/io';
    import CustomMenu from './context/CustomMenu.svelte';
    let items: Array<object> = [], only_file: string, current_file: string, path:string, start , end;
    export let url: string;
    onMount(async () => {
        console.log(url)
        items = await load_files(url);
        path = await get_config(url);
      
    });
</script>
<svelte:head>
  <!-- UIkit CSS -->
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/uikit@3.14.1/dist/css/uikit.min.css" />

<!-- UIkit JS -->
<script src="https://cdn.jsdelivr.net/npm/uikit@3.14.1/dist/js/uikit.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/uikit@3.14.1/dist/js/uikit-icons.min.js"></script>
</svelte:head>

<ul id="list" >
{#each  items as file_raw}
    <li >
      <CustomMenu></CustomMenu>
      <div class="uk-card uk-card-default uk-grid-collapse uk-child-width-1-2@s uk-margin" uk-grid>
        <div class="uk-card-media-left uk-cover-container">
            <img src="/images/file.png" alt="" width="64px">
           {file_raw.file}
        </div>
        <div>
            <div class="uk-card-body">
                
            </div>
        </div>
    </div>
    
</li>
{/each}
</ul>

<style>
li {

  padding: 0;
  list-style-type: none;
}
ul {
    padding: 0;
    list-style-type: none;
}
</style>

