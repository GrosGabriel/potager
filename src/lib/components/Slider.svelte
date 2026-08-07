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
    width: 46px;
    height: 24px;
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
    border-radius: 24px;
  }

  .slider::before {
    content: "";
    position: absolute;
    height: 18px;
    width: 18px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    transition: 0.3s;
    border-radius: 50%;
  }

  input:checked + .slider {
    background-color: #4caf50;
  }

  input:checked + .slider::before {
    transform: translateX(22px);
  }
</style>