<script lang="ts">
  import { copyWallet, shortenPubkey } from "../util/util";

  interface FundingModalProps {
    fundingWallet: string | null;
    sol: number | null;
    onClose?: () => void;
    onStartFunding?: (solPerWallet: number) => void;
    onCompleteFunding?: () => void;
    loading: boolean;
  }

  let {
    fundingWallet,
    sol,
    onClose,
    onStartFunding,
    onCompleteFunding,
    loading,
  }: FundingModalProps = $props();

  let fundingPerWallet = $state(0.1);
</script>

<div class="modal">
  <div class="funding-modal-header">
    <h1 class="modal-title">Funding...</h1>
    <button
      onclick={() => {
        onClose?.();
      }}
      class="close-button"
    >
      X
    </button>
  </div>
  <div class="funding-modal-layout">
    {#if fundingWallet !== null && sol !== null}
      <div style="display: flex; align-items: center; gap: 20px;">
        {shortenPubkey(fundingWallet)}
        <button
          class="secondary-button copy"
          onclick={() => copyWallet(fundingWallet)}>Copy</button
        >
      </div>
      <div>{(sol + 0.0001).toPrecision(5)} SOL</div>
      <button
        class="action-button"
        onclick={() => {
          onCompleteFunding?.();
        }}
        disabled={loading}>Complete Funding</button
      >
    {:else}
      Amount (SOL):
      <input
        name="amount"
        type="number"
        step="0.1"
        bind:value={fundingPerWallet}
        min="0"
        max="100"
      />
      <button
        class="action-button"
        onclick={() => {
          onStartFunding?.(fundingPerWallet);
        }}
        disabled={loading}>Start Funding</button
      >
    {/if}
  </div>
</div>

<style>
  .funding-modal-header {
    display: flex;
    justify-content: space-between;
  }
  .funding-modal-layout {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 20px;
  }
</style>
