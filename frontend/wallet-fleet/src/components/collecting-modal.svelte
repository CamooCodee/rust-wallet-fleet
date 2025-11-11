<script lang="ts">
  import { copyWallet, lamportsToSol, shortenPubkey } from "../util/util";

  interface CollectingModalProps {
    wallets: {
      pubkey: string;
      sol_lamports: string;
    }[];
    onClose: () => void;
    onCollect: (
      amountSol: number,
      pubkeys: string[],
      destination: string
    ) => void;
    isLoading: boolean;
  }

  let props: CollectingModalProps = $props();
  let solToCollect = $state(1);
  let destination = $state("");

  let rows = $derived(
    props.wallets.map((w) => {
      let difference = solToCollect / props.wallets.length;
      return {
        pubkey: w.pubkey,
        sol: lamportsToSol(w.sol_lamports),
        difference: difference,
        newBalance: lamportsToSol(w.sol_lamports) - difference,
      };
    })
  );

  let notEnoughSol = $derived(
    props.wallets.find(
      (w) =>
        lamportsToSol(w.sol_lamports) - solToCollect / props.wallets.length < 0
    ) !== undefined
  );

  let validCollectAmount = $derived(solToCollect > 0);
</script>

<div class="modal">
  <div class="collecting-modal-header">
    <h1 class="modal-title">Collect</h1>
    <button
      onclick={() => {
        props.onClose?.();
      }}
      class="close-button"
    >
      X
    </button>
  </div>
  <div class="collecting-modal-layout">
    <div class="input-area">
      <div>
        Collect:
        <input
          class="sol-input"
          name="to_collect"
          type="number"
          bind:value={solToCollect}
          min="0"
        />
        SOL
      </div>
      <div>
        Destination:
        <input
          class="destination-input"
          name="destination"
          bind:value={destination}
        />
      </div>
    </div>
    <div class="wallet-table">
      <table cellspacing="0" cellpadding="5" width="100%">
        <thead class="wallet-table-header">
          <tr>
            <th class="wallet-table-header-cell"> Wallet </th>
            <th class="wallet-table-header-cell"> Balance </th>
            <th class="wallet-table-header-cell"> Difference </th>
            <th class="wallet-table-header-cell"> New Balance</th>
          </tr>
        </thead>
        <tbody>
          {#each rows as row}
            <tr>
              <td>
                {shortenPubkey(row.pubkey)}
                <button
                  class="secondary-button copy"
                  style="margin-left: 15px;"
                  onclick={() => copyWallet(row.pubkey)}>Copy</button
                >
              </td>
              <td>{row.sol} SOL</td>
              <td>{-row.difference.toPrecision(2)} SOL</td>
              <td class={row.newBalance < 0 ? "invalid-balance" : ""}
                >{row.newBalance.toPrecision(2)} SOL</td
              >
              <!-- <div class="wallet-list-separator"></div> -->
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    <div class="button-layout">
      <button
        class="action-button"
        disabled={notEnoughSol || !validCollectAmount || props.isLoading}
        onclick={() => {
          props.onCollect(
            solToCollect,
            props.wallets.map((w) => w.pubkey),
            destination
          );
        }}
      >
        Collect
      </button>
    </div>
    <div class="invalid-balance">
      {#if notEnoughSol}
        At least one wallet doesn't have sufficient SOL.
      {:else if !validCollectAmount}
        You need to collect something.
      {/if}
    </div>
  </div>
</div>

<style>
  .collecting-modal-header {
    display: flex;
    justify-content: space-between;
  }
  .collecting-modal-layout {
    padding-block: 20px;
    height: 100%;
    width: 100%;
    gap: 20px;
  }
  .input-area {
    padding-bottom: 20px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .sol-input {
    width: 80px;
  }
  .destination-input {
    width: 400px;
  }
  .wallet-table {
    width: 100%;
    height: 200px;

    overflow: auto;

    scrollbar-color: rgba(255, 255, 255, 0.3) rgba(255, 255, 255, 0.05);
  }
  .wallet-table-header {
    position: sticky;
    top: 0;
    z-index: 1;

    text-align: left;

    color: var(--accent);
    background-color: var(--gray-accent);
  }
  .wallet-table-header-cell {
    padding-block: 10px;
  }
  .invalid-balance {
    color: red;
  }
  .button-layout {
    padding-top: 15px;
    padding-bottom: 10px;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
