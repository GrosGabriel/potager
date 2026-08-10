<script>
	import { NOMS_MOIS } from '$lib/utils.js';

	let { mois = $bindable([]) } = $props();

	let ouvert = $state(false);
	let racine = $state(null);

	function basculer(numero) {
		mois = mois.includes(numero) ? mois.filter((m) => m !== numero) : [...mois, numero].sort((a, b) => a - b);
	}

	let resume = $derived(
		mois.length === 0
			? 'Tous les mois'
			: mois.length <= 3
				? mois.map((m) => NOMS_MOIS[m - 1]).join(', ')
				: `${mois.length} mois sélectionnés`
	);

	function surClicDocument(event) {
		if (ouvert && racine && !racine.contains(event.target)) {
			ouvert = false;
		}
	}
</script>

<svelte:window onclick={surClicDocument} />

<div class="relative inline-block" bind:this={racine}>
	<button
		type="button"
		onclick={() => (ouvert = !ouvert)}
		class="cursor-pointer select-none rounded border border-gray-200 px-2.5 py-1 text-lg text-gray-700 hover:bg-gray-50 w-40 truncate text-left"
	>
		{resume}
	</button>
	{#if ouvert}
		<div class="absolute z-10 mt-1 grid grid-cols-2 gap-x-3 gap-y-1 rounded border border-gray-200 bg-white p-2 shadow-md w-max">
			{#each NOMS_MOIS as nom, i}
				<label class="flex items-center gap-1.5 whitespace-nowrap text-lg text-gray-700 cursor-pointer">
					<input
						type="checkbox"
						checked={mois.includes(i + 1)}
						onchange={() => basculer(i + 1)}
						class="rounded"
					/>
					{nom}
				</label>
			{/each}
		</div>
	{/if}
</div>
