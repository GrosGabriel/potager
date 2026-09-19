<script>
	import { NOMS_MOIS } from '$lib/utils.js';

	let { mois = $bindable([]) } = $props();

	let ouvert = $state(false);
	let racine = $state(null);

	let tousLesMoisActifs = $derived(mois.length === NOMS_MOIS.length);

	function basculer(numero) {
		mois = mois.includes(numero) ? mois.filter((m) => m !== numero) : [...mois, numero].sort((a, b) => a - b);
	}

	function toutCocher() {
		mois = NOMS_MOIS.map((_, i) => i + 1);
	}

	function toutDecocher() {
		mois = [];
	}

	let resume = $derived(
		tousLesMoisActifs
			? 'Tous les mois'
			: mois.length === 0
				? 'Aucun mois'
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
		class="cursor-pointer select-none rounded border border-gray-200 px-2.5 py-1 text-lg text-gray-700 transition-colors hover:border-b-green-600 hover:bg-gray-50 focus:outline-none focus:ring-0 w-40 truncate text-left"
	>
		{resume}
	</button>
	{#if ouvert}
		<div class="absolute z-10 mt-1 rounded border border-gray-200 bg-white p-2 shadow-md w-max">
			<div class="flex gap-2 border-b border-gray-100 pb-2 mb-2">
				<button type="button" onclick={toutCocher} class="text-xs text-green-700 hover:underline">Tout cocher</button>
				<button type="button" onclick={toutDecocher} class="text-xs text-gray-600 hover:underline">Tout décocher</button>
			</div>
			<div class="grid grid-cols-2 gap-x-3 gap-y-1">
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
		</div>
	{/if}
</div>
