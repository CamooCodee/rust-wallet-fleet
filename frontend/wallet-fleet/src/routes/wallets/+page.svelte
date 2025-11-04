<script lang="ts">
  import {
    copyWallet,
    lamportsToSol,
    postApi,
    shortenPubkey,
  } from "../../util/util";
  import { invalidate } from "$app/navigation";
  import FundingModal from "../../components/funding-modal.svelte";
  import type { ApiResponse } from "../../util/api";
  import { toastRes } from "../../util/toast";
  import CollectingModal from "../../components/collecting-modal.svelte";

  let { data } = $props();

  let loadingWallets: boolean = $state(false);

  async function createWallets() {
    loadingWallets = true;
    const response = await postApi("/wallets/create", { count: 1 });
    const data = (await response.json()) as ApiResponse;
    toastRes(response, data);

    await invalidate((url) => {
      return url.pathname == "/wallets/list";
    });
    loadingWallets = false;
  }

  let fundingModalOpen: boolean = $state(false);
  let loadingFunding: boolean = $state(false);
  let solToFund: number | null = $state(null);
  let fundingWallet: string | null = $state(null);

  interface InitiateFundingResponse extends ApiResponse {
    job: {
      funding_wallet_pubkey: string;
      total_funding_lamports: string;
    };
  }

  async function initiateFunding(solPerWallet: number) {
    loadingFunding = true;
    const lamportsPerWallet =
      (BigInt(solPerWallet * 1000) * 1_000_000_000n) / 1000n;
    const response = await postApi("/funding/initiate", {
      lamports_per_wallet: lamportsPerWallet.toString(),
    });
    const data = (await response.json()) as InitiateFundingResponse;
    toastRes(response, data);
    loadingFunding = false;

    if (response.status > 300) {
      return;
    }

    solToFund = lamportsToSol(data.job.total_funding_lamports);
    fundingWallet = data.job.funding_wallet_pubkey;
  }
  async function completeFunding() {
    loadingFunding = true;
    const response = await postApi("/funding/complete", null);
    const data = await response.json();
    toastRes(response, data);

    loadingFunding = false;

    if (response.status > 300) {
      return;
    }

    fundingWallet = null;
    solToFund = null;
    fundingModalOpen = false;
    loadingWallets = true;
    await invalidate((url) => {
      return url.pathname == "/wallets/list";
    });
    loadingWallets = false;
  }

  let collectModalOpen = $state(false);

  async function collectSol(
    solToCollect: number,
    pubkeys: string[],
    destination: string
  ) {
    const lamports = (solToCollect * 1_000_000_000).toString();
    const body = {
      lamports: lamports,
      source_pubkeys: pubkeys,
      destination: destination,
    };
    const response = await postApi("/collect", body);
    const data = await response.json();
    toastRes(response, data);

    if (response.status > 300) {
      return;
    }

    collectModalOpen = false;
  }
</script>

<div class="header-layout">
  <h1>Wallets</h1>
  <button
    class="action-button"
    onclick={() => {
      createWallets();
    }}
    disabled={loadingWallets}>Create</button
  >
  <button
    class="secondary-button"
    onclick={() => {
      fundingModalOpen = true;
    }}>Fund</button
  >
  <button
    class="secondary-button"
    onclick={() => {
      collectModalOpen = true;
    }}>Collect</button
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

<svelte:window
  onkeydown={(e) => {
    if (e.key === "Escape") {
      fundingModalOpen = false;
    }
  }}
/>

{#if fundingModalOpen}
  <div class="modal-container">
    <FundingModal
      onStartFunding={initiateFunding}
      onCompleteFunding={completeFunding}
      onClose={() => {
        fundingModalOpen = false;
      }}
      loading={loadingFunding}
      sol={solToFund}
      {fundingWallet}
    />
  </div>
{/if}

{#if collectModalOpen}
  <div class="modal-container">
    <CollectingModal
      wallets={data.wallets}
      onClose={() => {
        collectModalOpen = false;
      }}
      onCollect={(solToCollect, pubkeys, destination) => {
        collectSol(solToCollect, pubkeys, destination);
      }}
    />
  </div>
{/if}

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
</style>
