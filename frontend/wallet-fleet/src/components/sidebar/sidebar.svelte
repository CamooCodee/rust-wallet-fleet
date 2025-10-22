<script lang="ts">
  import SidebarButton from "./sidebar-button.svelte";
  import { goto } from "$app/navigation";

  interface Props {
    pathname: string;
  }

  let { pathname }: Props = $props();

  let buttons = [
    { title: "Home" },
    { title: "Wallets" },
    { title: "Settings" },
  ];

  function select(newTitle: string) {
    goto(`/${newTitle.toLowerCase()}`);
  }
</script>

<section class="layout">
  <div class="bar"></div>
  <h1 class="title">Wallet Fleet</h1>

  <div class="btn-layout">
    {#each buttons as btn}
      <SidebarButton
        title={btn.title}
        selected={`/${btn.title.toLowerCase()}` == pathname}
        onClick={() => {
          select(btn.title);
        }}
      />
    {/each}
  </div>
</section>

<style>
  .layout {
    box-sizing: border-box;
    padding-inline: 20px;
    position: relative;
    height: 100vh;
    width: 20vw;
    max-width: 300px;
    min-width: 200px;
    background-color: #16161a;
  }
  .btn-layout {
    margin-top: 20px;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: 10px;
  }
  .bar {
    position: absolute;
    right: 0;
    top: 0;
    background-color: gray;
    height: 100%;
    width: 1px;
  }
  .title {
    padding: 0;
  }
</style>
