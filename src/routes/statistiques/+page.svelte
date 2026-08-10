<script>
import { onMount } from 'svelte';
import { invoke } from '@tauri-apps/api/core';
import Slider from '$lib/components/Slider.svelte';
import GraphiqueCultures from '$lib/components/GraphiqueCultures.svelte';
import GraphiqueTemperature from '$lib/components/GraphiqueTemperature.svelte';
import GraphiqueCamembertArrosage from '$lib/components/GraphiqueCamembertArrosage.svelte';
import GraphiqueDatesPremieres from '$lib/components/GraphiqueDatesPremieres.svelte';
import IconEvenement from '$lib/components/IconEvenement.svelte';
import SelectMoisMultiple from '$lib/components/SelectMoisMultiple.svelte';
import { TYPES_EVENEMENT, TYPES_COULEUR_CULTURE, couleurType, ANNEES, libelleCulture, cleNombre, COULEURS_PREMIERES_DATES_SAISON_EVENEMENTS } from '$lib/utils.js';

let typesCouleurCulture = $derived(TYPES_EVENEMENT.filter((t) => TYPES_COULEUR_CULTURE.has(t)));
let typesCouleurFixe = $derived(TYPES_EVENEMENT.filter((t) => !TYPES_COULEUR_CULTURE.has(t)));


let statsCultures = $state(false);

let allCultures = $state([]);
let allCulturesTriees = $derived(
	[...allCultures].sort((a, b) =>
		`${a.nom} ${a.variete ?? ''}`.localeCompare(`${b.nom} ${b.variete ?? ''}`, 'fr')
	)
);

let culturesSelectionnees = $state([]);
let culturesSelectionneesTriees = $derived(
	[...culturesSelectionnees].sort((a, b) =>
		`${a.nom} ${a.variete ?? ''}`.localeCompare(`${b.nom} ${b.variete ?? ''}`, 'fr')
	)
);

let evenementsParCulture = $state(new Map());
let idsEnChargement = $state(new Set());
let enChargementEvenements = $derived(idsEnChargement.size > 0);

const typesGraphiques = ['Récolte', 'Arrosage', 'Plantation', 'Semis'];


let anneeMin = $state('');
let anneeMax = $state('');
let anneeMinNombre = $derived(anneeMin === '' ? null : Number(anneeMin));
let anneeMaxNombre = $derived(anneeMax === '' ? null : Number(anneeMax));
let moisSelectionnesCultures = $state([]);

$effect(() => {
	for (const culture of culturesSelectionnees) {
		if (!evenementsParCulture.has(culture.id) && !idsEnChargement.has(culture.id)) {
			idsEnChargement = new Set(idsEnChargement).add(culture.id);
			invoke('lister_evenements_par_culture_cmd', { cultureId: culture.id })
				.then((result) => {
					const carte = new Map(evenementsParCulture);
					carte.set(culture.id, result);
					evenementsParCulture = carte;
				})
				.catch((error) => {
					console.error("Erreur lors de la récupération des événements de la culture :", error);
				})
				.finally(() => {
					const restants = new Set(idsEnChargement);
					restants.delete(culture.id);
					idsEnChargement = restants;
				});
		}

	}
});

function chargerCultures() {
	invoke("lister_cultures_cmd")
		.then((result) => {
			allCultures = result;
		})
		.catch((error) => {
			console.error("Erreur lors de la récupération des cultures :", error);
		});
}

function compterEvenements(e) {
	if (!e) return 0;
	let res = 0;
	for (const k in e) {
		res += e[k];
	}
	return res;
}

// La date d'une photo n'est pas stockée sur l'image elle-même : on récupère
// toutes les images puis on remonte à la date via l'événement associé, pour
// pouvoir filtrer "photos ajoutées" par la période sélectionnée.
let tousImages = $state([]);
let chargementImages = $state(true);

function chargerImages() {
	chargementImages = true;
	invoke("lister_images_cmd")
		.then((result) => {
			tousImages = result;
		})
		.catch((error) => {
			console.error("Erreur lors de la récupération des images :", error);
		})
		.finally(() => {
			chargementImages = false;
		});
}

// Chargés une seule fois : servent à la fois au graphique de température et
// à la répartition par type d'événement des statistiques générales, chacun
// filtrant ensuite selon sa propre période.
let tousEvenements = $state([]);
let chargementEvenements = $state(true);

function chargerEvenements() {
	chargementEvenements = true;
	invoke("lister_evenements_cmd")
		.then((result) => {
			tousEvenements = result;
		})
		.catch((error) => {
			console.error("Erreur lors de la récupération des événements :", error);
		})
		.finally(() => {
			chargementEvenements = false;
		});
}

let evenementsTemperature = $derived(tousEvenements.filter((e) => e.type_evenement === 'Température'));

let anneeMinGen = $state('');
let anneeMaxGen = $state('');
let anneeMinGenNombre = $derived(anneeMinGen === '' ? null : Number(anneeMinGen));
let anneeMaxGenNombre = $derived(anneeMaxGen === '' ? null : Number(anneeMaxGen));
let moisSelectionnesGen = $state([]);

function dansPeriodeGen(dateISO) {
	const annee = Number(dateISO.slice(0, 4));
	const mois = Number(dateISO.slice(5, 7));
	if (anneeMinGenNombre !== null && annee < anneeMinGenNombre) return false;
	if (anneeMaxGenNombre !== null && annee > anneeMaxGenNombre) return false;
	if (moisSelectionnesGen.length > 0 && !moisSelectionnesGen.includes(mois)) return false;
	return true;
}

// Nombre d'événements par type sur la période sélectionnée pour les
// statistiques générales (toutes les clés à 0 par défaut pour que
// l'affichage n'ait jamais de trou même sans événement sur la période).
let nombreEvenementsParTypeFiltre = $derived.by(() => {
	const compte = Object.fromEntries(TYPES_EVENEMENT.map((type) => [cleNombre(type), 0]));
	for (const evenement of tousEvenements) {
		if (dansPeriodeGen(evenement.date)) {
			const cle = cleNombre(evenement.type_evenement);
			compte[cle] = (compte[cle] ?? 0) + 1;
		}
	}
	return compte;
});

let periodeGenActive = $derived(
	anneeMinGenNombre !== null || anneeMaxGenNombre !== null || moisSelectionnesGen.length > 0
);

let dateParEvenementId = $derived(new Map(tousEvenements.map((e) => [e.id, e.date])));

// Une culture est "suivie" sur la période si au moins un de ses événements
// tombe dedans. Sans période sélectionnée, on retombe sur le total (une
// culture tout juste ajoutée sans encore aucun événement compte quand même).
let cultureSuiviesFiltre = $derived.by(() => {
	if (!periodeGenActive) return allCultures.length;
	const ids = new Set();
	for (const evenement of tousEvenements) {
		if (evenement.culture_id != null && dansPeriodeGen(evenement.date)) {
			ids.add(evenement.culture_id);
		}
	}
	return ids.size;
});

let photosAjouteesFiltre = $derived.by(() => {
	if (!periodeGenActive) return tousImages.length;
	let compte = 0;
	for (const image of tousImages) {
		const date = dateParEvenementId.get(image.evenement_id);
		if (date && dansPeriodeGen(date)) compte++;
	}
	return compte;
});

$effect(() => {
	if (!statsCultures) {
		culturesSelectionnees = [];
	}
})

onMount(() => {
	chargerCultures();
	chargerEvenements();
	chargerImages();
});

</script>


<div class="p-6">
	<h3 class="text-4xl font-semibold text-gray-800 mb-5 text-center">
		{statsCultures ? 'Statistiques par culture' : 'Statistiques générales'}
	</h3>

	<div class="flex items-center justify-center gap-3 mb-6">
		<span
			class="text-md font-medium transition-colors"
			class:text-green-700={!statsCultures}
			class:text-gray-400={statsCultures}
		>
			Statistiques générales
		</span>
		<Slider bind:checked={statsCultures} />
		<span
			class="text-md font-medium transition-colors"
			class:text-green-700={statsCultures}
			class:text-gray-400={!statsCultures}
		>
			Statistiques par culture
		</span>
	</div>
</div>


<div>
	{#if statsCultures}
		<h3 class="text-2xl font-semibold text-gray-700 mb-2 text-center">Cultures sélectionnées</h3>
		{#if allCultures.length > 0}
			<div class="flex flex-wrap gap-2">
				{#each allCulturesTriees as culture}
					<label
						class="flex items-center gap-1.5 px-2.5 py-1 rounded-full border border-gray-200 text-lg text-gray-700 cursor-pointer hover:bg-gray-50"
					>
						<input
							type="checkbox"
							bind:group={culturesSelectionnees}
							value={culture}
							class="rounded"
							style="accent-color: {culture.couleur}"
						/>
						<span class="inline-block w-2.5 h-2.5 rounded-full flex-none" style="background-color: {culture.couleur}"></span>
						{libelleCulture(culture)}
					</label>
				{/each}
			</div>
		{:else}
			<p class="text-gray-500 text-sm">Ajoutez des cultures pour voir leurs statistiques.</p>
		{/if}

		
		

			<div class="flex gap-8 mb-5 pt-4">
				<div class="flex-1 flex flex-col justify-center space-y-3 items-center">
					<p class="text-2xl font-semibold text-gray-700 text-center">Selectionner la période</p>
					<div class="flex items-center justify-center gap-2 text-lg text-gray-600">
						<span>Période de</span>
						<select bind:value={anneeMin} class="rounded border-gray-200 py-1 text-lg">
							<option value="">Toutes les années</option>
							{#each ANNEES as annee}
								<option value={String(annee)}>{annee}</option>
							{/each}
						</select>
						<span>à</span>
						<select bind:value={anneeMax} class="rounded border-gray-200 py-1 text-lg">
							<option value="">Toutes les années</option>
							{#each ANNEES as annee}
								<option value={String(annee)}>{annee}</option>
							{/each}
						</select>
					</div>

					<div class="flex items-center justify-center gap-2 text-lg text-gray-600">
						<span>Uniquement les mois de</span>
						<SelectMoisMultiple bind:mois={moisSelectionnesCultures} />
					</div>
					<!--
					<div class="flex flex-wrap gap-x-4 gap-y-1.5">
						{#each culturesSelectionneesTriees as culture}
							<span class="flex items-center gap-1.5 text-sm text-gray-600">
								<span class="inline-block w-4 h-1" style="background-color: {culture.couleur}"></span>
								{libelleCulture(culture)}
							</span>
						{/each}
					</div> Pas sûr d'avoir besoin de cette légende vu qu'on voit bien les couleurs à d'autres endroits-->
				</div>

				<div class="flex-1">
					<p class="text-2xl text-center font-semibold text-gray-700 mb-2">Répartition du temps d'arrosage</p>
					{#if culturesSelectionnees.length > 0}
					<GraphiqueCamembertArrosage
						cultures={culturesSelectionnees}
						evenementsParCulture={evenementsParCulture}
						anneeMin={anneeMinNombre}
						anneeMax={anneeMaxNombre}
						moisSelectionnes={moisSelectionnesCultures}
						enChargement={enChargementEvenements}
					/>
					{:else}
						<div class="min-h-[22rem] flex items-center justify-center">
							<p class="text-gray-500 text-lg text-center">Sélectionnez au moins une culture pour voir la répartition du temps d'arrosage.</p>
						</div>
					{/if}
				</div>
			</div>

		{#if culturesSelectionnees.length > 0}
			<h4 class="text-2xl text-center font-semibold text-gray-700 mt-6 mb-2">Par type d'événement</h4>
			<p class="text-lg text-gray-500 text-center mb-2">
				Ces graphiques présentent le nombre d'événements enregistrés par type pour les cultures sélectionnées.
			</p>
			<div class="grid grid-cols-2 gap-5">
				
				{#each typesGraphiques as type}
					<div class="border border-gray-200 rounded-lg bg-gray-50 p-3">
						<div class="flex items-center gap-2 mb-2">
							<span
								class="w-6 h-6 rounded-full flex items-center justify-center flex-none"
								style="background-color: {couleurType(type)}"
							>
								<span class="w-3.5 h-3.5">
									<IconEvenement {type} />
								</span>
							</span>
							<p class="text-xl font-semibold text-gray-700">{type}</p>
						</div>
						<GraphiqueCultures
							cultures={culturesSelectionnees}
							evenementsParCulture={evenementsParCulture}
							enChargement={enChargementEvenements}
							type={type}
							anneeMin={anneeMinNombre}
							anneeMax={anneeMaxNombre}
							moisSelectionnes={moisSelectionnesCultures}
						/>
					</div>
				{/each}
			</div>

			<h4 class="text-2xl text-center font-semibold text-gray-700 mt-6 mb-2">Premières dates de la saison</h4>
			<p class="text-lg text-gray-500 text-center mb-2">
				Ce graphique présente la première date de semis, de plantation et de récolte de chaque année.
			</p>

			<div class="flex flex-wrap justify-center gap-x-4 gap-y-1.5 mb-3">
				{#each Object.entries(COULEURS_PREMIERES_DATES_SAISON_EVENEMENTS) as [type, couleur]}
					<span class="flex items-center gap-1.5 text-lg text-gray-600">
						<span class="inline-block w-2.5 h-2.5 rounded flex-none" style="background-color: {couleur}"></span>
						{type}
					</span>
				{/each}
			</div>

			<div class="grid grid-cols-2 gap-5">
				{#each culturesSelectionneesTriees as culture}
					<div class="border border-gray-200 rounded-lg bg-gray-50 p-3">
						<p class="text-lg font-semibold text-gray-700 mb-2">{libelleCulture(culture)}</p>
						<GraphiqueDatesPremieres
							{culture}
							evenements={evenementsParCulture.get(culture.id) ?? []}
							anneeMin={anneeMinNombre}
							anneeMax={anneeMaxNombre}
							enChargement={enChargementEvenements}
						/>
					</div>
				{/each}
			</div>
		{/if}


	{:else}
		{#if !chargementImages}
			<div class="grid grid-cols-3 gap-4 mb-8">
				<div class="border border-gray-200 rounded-lg bg-gray-50 p-5 text-center">
					<p class="text-4xl font-semibold text-gray-800">{compterEvenements(nombreEvenementsParTypeFiltre)}</p>
					<p class="text-lg text-gray-500 mt-1">événements enregistrés</p>
				</div>
				<div class="border border-gray-200 rounded-lg bg-gray-50 p-5 text-center">
					<p class="text-4xl font-semibold text-gray-800">{cultureSuiviesFiltre}</p>
					<p class="text-lg text-gray-500 mt-1">cultures suivies</p>
				</div>
				<div class="border border-gray-200 rounded-lg bg-gray-50 p-5 text-center">
					<p class="text-4xl font-semibold text-gray-800">{photosAjouteesFiltre}</p>
					<p class="text-lg text-gray-500 mt-1">photos ajoutées</p>
				</div>
			</div>

			<div class="flex gap-8 items-center">
				<div class="flex-1 flex flex-col justify-center items-center space-y-3">
					<p class="text-2xl font-semibold text-gray-700 text-center">Selectionner la période</p>
					<div class="flex items-center justify-center gap-2 text-lg text-gray-600">
						<span>Période de</span>
						<select bind:value={anneeMinGen} class="rounded border-gray-200 py-1 text-lg">
							<option value="">Toutes les années</option>
							{#each ANNEES as annee}
								<option value={String(annee)}>{annee}</option>
							{/each}
						</select>
						<span>à</span>
						<select bind:value={anneeMaxGen} class="rounded border-gray-200 py-1 text-lg">
							<option value="">Toutes les années</option>
							{#each ANNEES as annee}
								<option value={String(annee)}>{annee}</option>
							{/each}
						</select>
					</div>
					<div class="flex items-center justify-center gap-2 text-lg text-gray-600">
						<span>Uniquement les mois de</span>
						<SelectMoisMultiple bind:mois={moisSelectionnesGen} />
					</div>
				</div>

				<div class="flex-[2]">
					<p class="text-2xl font-semibold text-gray-700 mb-3 text-center">Répartition par type d'événement</p>
					<div class="grid grid-cols-2 gap-6 max-w-2xl mx-auto">
						<div class="space-y-3">
							{#each typesCouleurCulture as type}
								<div class="flex items-center gap-3 border border-gray-200 rounded-lg p-3">
									<span
										class="w-10 h-10 rounded-full flex items-center justify-center flex-none"
										style="background-color: {couleurType(type)}"
									>
										<span class="w-5 h-5">
											<IconEvenement {type} />
										</span>
									</span>
									<span class="text-lg text-gray-700 flex-1">{type}</span>
									<span class="text-xl font-semibold text-gray-800">{nombreEvenementsParTypeFiltre[cleNombre(type)]}</span>
								</div>
							{/each}
						</div>
						<div class="space-y-3">
							{#each typesCouleurFixe as type}
								<div class="flex items-center gap-3 border border-gray-200 rounded-lg p-3">
									<span
										class="w-10 h-10 rounded-full flex items-center justify-center flex-none"
										style="background-color: {couleurType(type)}"
									>
										<span class="w-5 h-5">
											<IconEvenement {type} />
										</span>
									</span>
									<span class="text-lg text-gray-700 flex-1">{type}</span>
									<span class="text-xl font-semibold text-gray-800">{nombreEvenementsParTypeFiltre[cleNombre(type)]}</span>
								</div>
							{/each}
						</div>
					</div>
				</div>
			</div>
		{:else}
			<p class="text-gray-500 text-center text-lg">Chargement des statistiques...</p>
		{/if}

		<h4 class="text-2xl text-center font-semibold text-gray-700 mt-6 mb-2">Température</h4>
		<p class="text-lg text-gray-500 text-center mb-2">
			Ce graphique présente les températures minimales et maximales intérieures et extérieures sur la période sélectionnée.
		</p>

		<GraphiqueTemperature
			evenements={evenementsTemperature}
			anneeMin={anneeMinGenNombre}
			anneeMax={anneeMaxGenNombre}
			moisSelectionnes={moisSelectionnesGen}
			enChargement={chargementEvenements}
		/>

	{/if}
</div>
