<script>

let { checked = $bindable(false) } = $props();

</script>

<label class="switch">
  <input type="checkbox" bind:checked={checked} />
  <span class="slider"></span>
</label>





<style>
  .switch {
    position: relative;
    display: inline-block;
    width: 68px;
    height: 36px;
  }

  .switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    inset: 0;
    background-color: #ccc;
    transition: 0.3s;
    border-radius: 36px;
  }

  .slider::before {
    content: "";
    position: absolute;
    height: 28px;
    width: 28px;
    left: 4px;
    bottom: 4px;
    background-color: white;
    transition: 0.3s;
    border-radius: 50%;
  }

  input:checked + .slider {
    background-color: #4caf50;
  }

  input:checked + .slider::before {
    transform: translateX(32px);
  }
</style>