<script>
	import Calendar from '$lib/components/Calendar.svelte';
    import Modal from '$lib/components/Modal.svelte';
	import EvenementForm from '$lib/components/EvenementForm.svelte';
	import { invoke, convertFileSrc } from '@tauri-apps/api/core';
	import { TYPES_EVENEMENT, couleurType, couleurEvenement } from '$lib/evenementsColor.js';

	let openModalAjouterEvenement = $state(false);

	let lightboxOuvert = $state(false);
	let lightboxImages = $state([]);
	let lightboxIndex = $state(0);

	function ouvrirLightbox(images, index) {
		lightboxImages = images;
		lightboxIndex = index;
		lightboxOuvert = true;
	}

	function imageSuivante() {
		lightboxIndex = (lightboxIndex + 1) % lightboxImages.length;
	}

	function imagePrecedente() {
		lightboxIndex = (lightboxIndex - 1 + lightboxImages.length) % lightboxImages.length;
	}

	function surTouche(event) {
		if (!lightboxOuvert) return;
		if (event.key === 'ArrowRight') imageSuivante();
		else if (event.key === 'ArrowLeft') imagePrecedente();
	}

	let selected = $state(null);
	let evenements = $state([]);
	let tousEvenements = $state([]);
	let cultures = $state([]);

	let typesActifs = $state(new Set(TYPES_EVENEMENT));
	let cultureFiltreId = $state('');

	let evenementsFiltres = $derived(
		tousEvenements.filter(
			(e) =>
				typesActifs.has(e.type_evenement) &&
				(cultureFiltreId === '' || String(e.culture_id) === cultureFiltreId)
		)
	);

	function basculerType(type) {
		const nouveau = new Set(typesActifs);
		if (nouveau.has(type)) {
			nouveau.delete(type);
		} else {
			nouveau.add(type);
		}
		typesActifs = nouveau;
	}
	const formateurDate = new Intl.DateTimeFormat('fr-FR', {
		weekday: 'long',
		day: 'numeric',
		month: 'long',
		year: 'numeric'
	});

	function dateLocaleVersISO(d) {
		const annee = d.getFullYear();
		const mois = String(d.getMonth() + 1).padStart(2, '0');
		const jour = String(d.getDate()).padStart(2, '0');
		return `${annee}-${mois}-${jour}`;
	}

	function rafraichirEvenements() {
		if (selected) {
			invoke("lister_evenements_par_date_avec_culture_et_images_cmd", { date: dateLocaleVersISO(selected) })
				.then((result) => {
					evenements = result;
				})
				.catch((error) => {
					console.error("Erreur lors de la récupération des événements :", error);
				});
		}
	}

	function rafraichirTousEvenements() {
		invoke("lister_evenements_avec_culture_cmd")
			.then((result) => {
				tousEvenements = result;
			})
			.catch((error) => {
				console.error("Erreur lors de la récupération des événements du calendrier :", error);
			});
	}

	function rafraichirCultures() {
		invoke("lister_cultures_cmd")
			.then((result) => {
				cultures = result;
			})
			.catch((error) => {
				console.error("Erreur lors de la récupération des cultures :", error);
			});
	}

	function evenementAjoute() {
		openModalAjouterEvenement = false;
		rafraichirEvenements();
		rafraichirTousEvenements();
		rafraichirCultures();
	}

	$effect(() => {
		rafraichirEvenements();
	})

	$effect(() => {
		rafraichirTousEvenements();
	})

	$effect(() => {
		rafraichirCultures();
	})
</script>

<div class="flex h-[calc(100vh-49px)] overflow-hidden">
	<div class="w-1/2 border-r border-gray-200 p-6 flex flex-col h-full overflow-hidden">
		<div class="flex flex-wrap items-center gap-x-4 gap-y-2 mb-4">
			<div class="flex flex-wrap gap-x-3 gap-y-1.5">
				{#each TYPES_EVENEMENT as type}
					<label class="flex items-center gap-1.5 text-sm text-gray-700 cursor-pointer">
						<input
							type="checkbox"
							checked={typesActifs.has(type)}
							onchange={() => basculerType(type)}
							class="rounded"
							style="accent-color: {couleurType(type)}"
						/>
						{type}
					</label>
				{/each}
			</div>
			<select bind:value={cultureFiltreId} class="text-sm rounded border-gray-200 py-1">
				<option value="">Toutes les cultures</option>
				{#each cultures as culture}
					<option value={String(culture.id)}>
						{culture.nom}{culture.variete ? ` (${culture.variete})` : ''}
					</option>
				{/each}
			</select>
		</div>

		<div class="flex-1 min-h-0">
			<Calendar bind:selected evenements={evenementsFiltres} />
		</div>
	</div>

	<div class="w-1/2 p-6 h-full overflow-y-auto">
		{#if selected}
			<h2 class="text-lg font-semibold capitalize mb-4">{formateurDate.format(selected)}</h2>

			<button class="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700" onclick={() => {openModalAjouterEvenement = true}}>
				Ajouter un événement
			</button>

			{#if evenements.length > 0}
			<p class="text-gray-500 mb-4">Événements enregistrés pour cette date :</p>
			<ul class="space-y-2">
				{#each evenements as evenement}
					{@const style = couleurEvenement(evenement)}
					<li
						class="border border-l-4 border-gray-200 rounded p-4"
						style="border-left-color: {style.couleur}"
					>
						<p><strong>Type :</strong> {evenement.type_evenement}</p>
						{#if evenement.culture_nom}
							<p><strong>Culture :</strong> {evenement.culture_nom}</p>
						{/if}
						{#if evenement.notes}
							<p class="whitespace-pre-wrap break-words"><strong>Notes :</strong> {evenement.notes}</p>
						{/if}
						{#if evenement.images?.length > 0}
							<div class="flex flex-wrap gap-2 mt-2">
								{#each evenement.images as image, index}
									<button
										type="button"
										onclick={() => ouvrirLightbox(evenement.images.map((i) => convertFileSrc(i.chemin_fichier)), index)}
										class="w-16 h-16 rounded border border-gray-200 overflow-hidden"
									>
										<img
											src={convertFileSrc(image.chemin_fichier)}
											alt=""
											class="w-full h-full object-cover"
										/>
									</button>
								{/each}
							</div>
						{/if}
					</li>
				{/each}
			</ul>
			{:else}
			<p class="text-gray-500">Aucun événement enregistré pour cette date.</p>
			{/if}
		{:else}
			<p class="text-gray-500">Sélectionnez une date dans le calendrier.</p>
		{/if}
	</div>
</div>


<Modal open={openModalAjouterEvenement} onclose={() => openModalAjouterEvenement = false}>

	<EvenementForm
		date={selected}
		onsuccess={evenementAjoute}
		oncancel={() => openModalAjouterEvenement = false}
	/>

</Modal>

<svelte:window onkeydown={surTouche} />

<Modal open={lightboxOuvert} onclose={() => lightboxOuvert = false}>
	<div class="flex items-center gap-3">
		{#if lightboxImages.length > 1}
			<button
				type="button"
				onclick={imagePrecedente}
				class="flex-none px-2 py-1 text-2xl text-gray-500 hover:text-gray-800"
				aria-label="Photo précédente"
			>
				‹
			</button>
		{/if}

		<img src={lightboxImages[lightboxIndex]} alt="" class="max-w-[60vw] max-h-[70vh] object-contain rounded" />

		{#if lightboxImages.length > 1}
			<button
				type="button"
				onclick={imageSuivante}
				class="flex-none px-2 py-1 text-2xl text-gray-500 hover:text-gray-800"
				aria-label="Photo suivante"
			>
				›
			</button>
		{/if}
	</div>

	{#if lightboxImages.length > 1}
		<p class="text-center text-sm text-gray-500 mt-2">{lightboxIndex + 1} / {lightboxImages.length}</p>
	{/if}
</Modal>