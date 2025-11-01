<script lang="ts">
  import { onMount } from "svelte";
  import type { Toast } from "../util/toast";
  import { flip } from "svelte/animate";
  import { fade } from "svelte/transition";
  import { CircleCheck, CircleX } from "@lucide/svelte";

  interface MountedToast {
    id: string;
    toast: Toast;
  }

  let toasts = $state<MountedToast[]>([]);

  function removeToast(id: string) {
    toasts = toasts.filter((t) => t.id !== id);
  }

  onMount(() => {
    const add = (e: Event) => {
      let event = e as CustomEvent;
      const toast = event.detail as Toast;

      let newId = Math.floor(Math.random() * 100000).toString();
      while (toasts.find((t) => t.id === newId) !== undefined) {
        newId = Math.floor(Math.random() * 100000).toString();
      }

      const toastToMount = {
        id: newId,
        toast: toast,
      };
      toasts = [...toasts, toastToMount];
      window.setTimeout(() => removeToast(toastToMount.id), 5000);
    };
    window.addEventListener("toast:add", add);
    console.log("Toast mounted");
  });
</script>

<div class="toast-layout">
  {#each toasts as t (t.id)}
    <div
      class="toast"
      id={t.id}
      transition:fade={{ duration: 200 }}
      animate:flip={{ duration: 200 }}
    >
      {#if t.toast.success}
        <CircleCheck color="white" fill="#2ebf55" />
      {:else}
        <CircleX color="white" fill="#ff2b41" />
      {/if}
      {t.toast.message}
    </div>
  {/each}
</div>

<style>
  .toast {
    background-color: white;
    color: black;

    border-radius: 10px;
    padding: 10px;

    display: flex;
    align-items: center;
    gap: 5px;
  }
  .toast-layout {
    position: fixed;
    bottom: 10px;
    left: 50%;
    transform: translateX(-50%);

    width: 400px;
    height: 100px;

    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: end;
    gap: 6px;
  }
</style>
