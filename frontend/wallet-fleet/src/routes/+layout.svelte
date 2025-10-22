<script lang="ts">
  import favicon from "$lib/assets/favicon.svg";
  import "../app.css";
  import Sidebar from "../components/sidebar/sidebar.svelte";
  import { page } from "$app/state";

  let { children } = $props();

  let pathname = $derived(page.url.pathname);
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

<div class="app">
  <Sidebar {pathname} />
  <main class="content">
    {@render children?.()}
  </main>
</div>

<style>
  .app {
    display: flex;
    width: 100%;
    height: 100%;
  }
  .content {
    flex: 1;
    min-width: 0;
    padding-inline: 15px;
  }
</style>
