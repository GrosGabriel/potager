<script>
	import { onMount } from 'svelte';
	import Calendar from '$lib/components/Calendar.svelte';
    import Modal from '$lib/components/Modal.svelte';
	import EvenementForm from '$lib/components/EvenementForm.svelte';
	import IconEvenement from '$lib/components/IconEvenement.svelte';
	import SelectTypesEvenement from '$lib/components/SelectTypesEvenement.svelte';
	import { invoke, convertFileSrc } from '@tauri-apps/api/core';
	import { TYPES_EVENEMENT, TYPES_EVENEMENTS_TRI, couleurEvenement, libelleCulture } from '$lib/utils.js';

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
	let culturesTriees = $derived(
		[...cultures].sort((a, b) =>
			`${a.nom} ${a.variete ?? ''}`.localeCompare(`${b.nom} ${b.variete ?? ''}`, 'fr')
		)
	);

	let typesActifs = $state(new Set(TYPES_EVENEMENT));
	let cultureFiltreId = $state('');
	let cultureASupprimer = $state(null);
	let cultureFiltree = $derived(cultures.find((c) => String(c.id) === cultureFiltreId));

	let evenementsFiltres = $derived(
		tousEvenements.filter(
			(e) =>
				typesActifs.has(e.type_evenement) &&
				(cultureFiltreId === '' || String(e.culture_id) === cultureFiltreId)
		)
	);

	let evenementsSorted = $derived(
		[...evenements].sort((a,b) => {
			return TYPES_EVENEMENTS_TRI[a.type_evenement] - TYPES_EVENEMENTS_TRI[b.type_evenement];
		})
	)

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

	function modifierCouleurCulture(cultureId, couleur) {
		invoke("modifier_couleur_culture_cmd", { cultureId: parseInt(cultureId), couleur })
			.then(() => {
				rafraichirCultures();
				rafraichirTousEvenements();
				rafraichirEvenements();
			})
			.catch((error) => {
				console.error("Erreur lors de la modification de la couleur de la culture :", error);
			});
	}

	function evenementAjoute() {
		openModalAjouterEvenement = false;
		rafraichirEvenements();
		rafraichirTousEvenements();
		rafraichirCultures();
	}

	let evenementASupprimer = $state(null);
	let suppressionEnCours = $state(false);

	async function confirmerSuppression() {
		if (!evenementASupprimer) return;
		suppressionEnCours = true;
		try {
			await invoke('supprimer_evenement_cmd', { evenementId: evenementASupprimer.id });
			evenementASupprimer = null;
			rafraichirEvenements();
			rafraichirTousEvenements();
		} catch (error) {
			console.error("Erreur lors de la suppression de l'événement :", error);
		} finally {
			suppressionEnCours = false;
		}
	}

	$effect(() => {
		rafraichirEvenements(); // est bind avec selected, donc se déclenche à chaque changement de date
	})

	onMount(() => {
		rafraichirTousEvenements();
	})

	onMount(() => {
		rafraichirCultures();
	})
</script>

<div class="flex h-[calc(100vh-49px)] overflow-hidden">
	<div class="w-1/2 border-r border-gray-200 p-6 flex flex-col h-full overflow-hidden">
		<div class="flex flex-wrap items-center gap-x-4 gap-y-2 mb-4">
			<div class="flex items-center gap-1.5">
				<span class="text-sm text-gray-600">Événements :</span>
				<SelectTypesEvenement bind:typesActifs />
			</div>
			<div class="flex items-center gap-1.5">
				<label for="culture-filtre" class="text-sm text-gray-600">Culture :</label>
				<select id="culture-filtre" bind:value={cultureFiltreId} class="text-sm rounded border border-gray-200 py-1 max-w-[15rem] flex-none transition-colors hover:border-b-green-600 hover:bg-gray-50 focus:outline-none focus:ring-0">
					<option value="">Toutes les cultures</option>
					{#each culturesTriees as culture}
						<option value={String(culture.id)}>
							{libelleCulture(culture, 13, 10)}
						</option>
					{/each}
				</select>

				{#if cultureFiltreId !== '' && cultureFiltree}
					<input
						type="color"
						value={cultureFiltree.couleur}
						onchange={(e) => modifierCouleurCulture(cultureFiltreId, e.currentTarget.value)}
						class="flex-none w-8 h-8 rounded border border-gray-300 p-0.5 cursor-pointer"
						aria-label="Changer la couleur de cette culture"
						title="Changer la couleur de cette culture"
					/>

					<button
						type="button"
						onclick={() => (cultureASupprimer = cultureFiltreId)}
						class="flex-none flex items-center justify-center w-8 h-8 rounded border border-gray-300 text-gray-400 hover:text-red-600 hover:border-red-300 hover:bg-red-50"
						aria-label="Supprimer cette culture"
						title="Supprimer cette culture"
					>
						<span class="w-4 h-4">
							<IconEvenement type="Supprimer" />
						</span>
					</button>
				{/if}
			</div>
		</div>

		<div class="flex-1 min-h-0">
			<Calendar bind:selected evenements={evenementsFiltres} />
		</div>
	</div>

	<div class="w-1/2 p-6 h-full overflow-y-auto">
		{#if selected}
			<h2 class="text-2xl font-semibold capitalize mb-4">{formateurDate.format(selected)}</h2>

			<button class="bg-green-600 text-white text-xl px-4 py-2 rounded hover:bg-green-700" onclick={() => {openModalAjouterEvenement = true}}>
				Ajouter un événement
			</button>

			{#if evenements.length > 0}
			<p class="text-gray-500 text-lg mb-4 pt-3">Événements enregistrés pour cette date :</p>
			<ul class="space-y-2">
				{#each evenementsSorted as evenement}
					{@const style = couleurEvenement(evenement)}
					<li
						class="border border-l-4 border-gray-200 rounded p-4"
						style="border-left-color: {style.couleur}"
					>
						<div class="flex items-start justify-between gap-2">
							<p class="text-lg"><strong>Type :</strong> {evenement.type_evenement}</p>
							<button
								type="button"
								onclick={() => (evenementASupprimer = evenement)}
								class="flex-none flex items-center justify-center w-10 h-10 rounded border border-gray-300 text-gray-400 hover:text-red-600 hover:border-red-300 hover:bg-red-50"
								aria-label="Supprimer cet événement"
								title="Supprimer cet événement"
							>
								<span class="w-6 h-6">
									<IconEvenement type="Supprimer" />
								</span>
							</button>
						</div>
						{#if evenement.culture_nom}
							<p class="text-lg"><strong>Culture :</strong> {evenement.culture_nom} {evenement.culture_variete ? `(${evenement.culture_variete})` : ''}</p>
						{/if}
						{#if evenement.type_evenement === "Arrosage"}
							<p class="text-lg"><strong>Durée d'arrosage :</strong> {evenement.temps_arrosage} minutes</p>
						{/if}
						{#if evenement.type_evenement === "Température"}
							<p class="text-lg"><strong>Température intérieure :</strong> {evenement.temperature_int} °C</p>
							<p class="text-lg"><strong>Température extérieure :</strong> {evenement.temperature_ext} °C</p>
						{/if}
						{#if evenement.notes}
							<p class="text-lg whitespace-pre-wrap break-words"><strong>Notes :</strong> {evenement.notes}</p>
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
			<p class="text-gray-500 text-lg pt-3">Aucun événement enregistré pour cette date.</p>
			{/if}
		{:else}
			<p class="text-gray-500 text-lg pt-3">Sélectionnez une date dans le calendrier.</p>
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

<Modal open={evenementASupprimer !== null} onclose={() => (evenementASupprimer = null)}>
	<p class="text-2xl mb-4">Voulez-vous vraiment supprimer cet événement ?</p>
	<div class="flex justify-end gap-2">
		<button
			type="button"
			onclick={() => (evenementASupprimer = null)}
			class="px-4 py-2 rounded text-md text-gray-600 hover:bg-gray-100"
		>
			Non
		</button>
		<button
			type="button"
			onclick={confirmerSuppression}
			disabled={suppressionEnCours}
			class="bg-red-600 text-md text-white px-4 py-2 rounded hover:bg-red-700 disabled:opacity-50"
		>
			{suppressionEnCours ? 'Suppression…' : 'Oui'}
		</button>
	</div>
</Modal>

<Modal open={cultureASupprimer !== null} onclose={() => (cultureASupprimer = null)}>
	<p class="text-2xl mb-1">Voulez-vous vraiment supprimer cette culture ?</p>
	<p class="text-lg text-gray-500 mb-4">Tous les événements liés à cette culture seront également supprimés.</p>
	<div class="flex justify-end gap-2">
		<button
			type="button"
			onclick={() => (cultureASupprimer = null)}
			class="px-4 py-2 rounded text-md text-gray-600 hover:bg-gray-100"
		>
			Non
		</button>
		<button
			type="button"
			onclick={async () => {
				try {
					await invoke('supprimer_culture_cmd', { cultureId: parseInt(cultureASupprimer) });
					cultureASupprimer = null;
					cultureFiltreId = '';
					rafraichirCultures();
					rafraichirTousEvenements();
				} catch (error) {
					console.error("Erreur lors de la suppression de la culture :", error);
				}
			}}
			class="bg-red-600 text-md text-white px-4 py-2 rounded hover:bg-red-700"
		>
			Oui
		</button>
	</div>
</Modal>