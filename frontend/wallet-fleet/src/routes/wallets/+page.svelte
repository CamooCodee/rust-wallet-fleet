<script lang="ts">
  import { lamportsToSol, postApi, shortenPubkey } from "../../util/util";
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

  async function copyWallet(pubkey: string) {
    await navigator.clipboard.writeText(pubkey);
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

<!--Make the colums aligned-->
<ul class="wallet-list">
  {#each data.wallets as wallet}
    <li class="wallet-list-element">
      {shortenPubkey(wallet.pubkey)}
      <button
        class="secondary-button copy"
        onclick={() => copyWallet(wallet.pubkey)}>Copy</button
      >
      {lamportsToSol(wallet.sol_lamports)} SOL
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

    display: flex;
    align-items: center;
    gap: 20px;
  }
  .wallet-list-separator {
    width: 100%;
    height: 1px;
    position: absolute;
    bottom: 0;
    left: 0;

    background-color: gray;
  }
  .copy.secondary-button {
    padding-inline: 10px;
    padding-block: 5px;
    font-size: x-small;
  }
</style>
