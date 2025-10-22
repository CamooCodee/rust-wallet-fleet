<script>
  import { postApi, shortenPubkey } from "../../util/util";
  import { invalidate } from "$app/navigation";

  let { data } = $props();
  console.log(data);

  // TODO: Add loading state that disables the button
  async function createWallets() {
    const response = await postApi("/wallets/create", { count: 1 });
    console.log(response.status);
    await invalidate((url) => {
      console.log(url);
      return url.pathname == "/wallets/list";
    });
  }
</script>

<div class="header-layout">
  <h1>Wallets</h1>
  <button
    class="action-button"
    onclick={() => {
      createWallets();
    }}>Create</button
  >
</div>

<ul class="wallet-list">
  {#each data.pubkeys as pubkey}
    <li class="wallet-list-element">
      {shortenPubkey(pubkey)}
      <div class="wallet-list-separator"></div>
    </li>
  {/each}
</ul>

<style>
  .header-layout {
    display: flex;
    align-items: center;
    gap: 20px;
  }
  .wallet-list {
    padding: 0;
    list-style-type: none;
  }
  .wallet-list-element {
    padding-block: 9px;
    padding-inline: 9px;
    position: relative;
  }
  .wallet-list-separator {
    width: 100%;
    height: 1px;
    position: absolute;
    bottom: 0;
    left: 0;

    background-color: gray;
  }
</style>
