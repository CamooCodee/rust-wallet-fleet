<script>
  let { title, selected, onClick } = $props();
</script>

<button class="btn {selected ? 'selected' : ''}" onclick={() => onClick()}>
  {title}
</button>

<style>
  .btn {
    background-color: transparent;
    color: white;
    text-align: left;
    padding: 10px;
    border-radius: 12px;

    border: 1px solid gray;
    cursor: pointer;
  }
  .btn:hover {
    background-color: rgba(255, 255, 255, 0.05);
  }
  .btn:active {
    background-color: rgba(255, 255, 255, 0.1);
  }

  .selected {
    border-color: orangered;
  }
</style>
