<!-- Modal.svelte -->
<script lang="ts">
  //import { onMount } from 'svelte'

  let { open, onclose, children } = $props()
  let dialog: HTMLDialogElement

  $effect(() => {
    if (open) {
      dialog.showModal()  // bloque les clics extérieurs nativement
    } else {
      dialog.close()
    }
  })
</script>

<!-- Clic sur le backdrop (hors modal) → ferme -->
<dialog bind:this={dialog} onclose={onclose} onclick={(e) => {
  if (e.target === dialog) onclose()
}}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="modal-content" role="button" tabindex="0" onclick={(e) => e.stopPropagation()}>
    {@render children()}
  </div>
</dialog>

<style>
  dialog {
    border: none;
    border-radius: 16px;
    padding: 0;
    max-width: 90vw;
    margin: auto;
    box-shadow: 0 8px 40px rgba(0,0,0,0.18);
  }

  /* Le backdrop natif du <dialog> */
  dialog::backdrop {
    background: rgba(0, 0, 0, 0.4);
    backdrop-filter: blur(2px);
  }

  .modal-content {
    
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem;
    min-width: 320px;
  }
</style>