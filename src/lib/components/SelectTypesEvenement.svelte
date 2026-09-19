<script>
	import { TYPES_EVENEMENT, couleurType } from '$lib/utils.js';

	let { typesActifs = $bindable(new Set()) } = $props();

	let ouvert = $state(false);
	let racine = $state(null);

	let tousLesTypesActifs = $derived(typesActifs.size === TYPES_EVENEMENT.length);
	let resume = $derived(
		tousLesTypesActifs
			? 'Tous les événements'
			: typesActifs.size === 0
				? 'Aucun événement'
			: typesActifs.size === 1
				? [...typesActifs][0]
				: `${typesActifs.size} types sélectionnés`
	);

	function basculer(type) {
		const selection = new Set(typesActifs);
		if (selection.has(type)) {
			selection.delete(type);
		} else {
			selection.add(type);
		}
		typesActifs = selection;
	}

	function toutCocher() {
		typesActifs = new Set(TYPES_EVENEMENT);
	}

	function toutDecocher() {
		typesActifs = new Set();
	}

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
		class="cursor-pointer select-none rounded border border-gray-200 px-2.5 py-1 text-sm text-gray-700 transition-colors hover:border-b-green-600 hover:bg-gray-50 focus:outline-none focus:ring-0 min-w-40 truncate text-left"
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
			{#each TYPES_EVENEMENT as type}
				<label class="flex items-center gap-1.5 whitespace-nowrap text-sm text-gray-700 cursor-pointer">
					<input
						type="checkbox"
						checked={tousLesTypesActifs || typesActifs.has(type)}
						onchange={() => basculer(type)}
						class="rounded"
						style="accent-color: {couleurType(type)}"
					/>
					{type}
				</label>
			{/each}
			</div>
		</div>
	{/if}
</div>
