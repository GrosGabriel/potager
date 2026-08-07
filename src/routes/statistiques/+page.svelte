<script>
import { onMount } from 'svelte';
import { invoke } from '@tauri-apps/api/core';
import Slider from '$lib/components/Slider.svelte';
import GraphiqueCultures from '$lib/components/GraphiqueCultures.svelte';
import GraphiqueTemperature from '$lib/components/GraphiqueTemperature.svelte';
import IconEvenement from '$lib/components/IconEvenement.svelte';
import SelectMoisMultiple from '$lib/components/SelectMoisMultiple.svelte';
import { couleurType, ANNEES } from '$lib/evenementsColor.js';


let statsCultures = $state(false);

let allCultures = $state([]);

let culturesSelectionnees = $state([]);

let evenementsParCulture = $state(new Map());
let idsEnChargement = $state(new Set());
let enChargementEvenements = $derived(idsEnChargement.size > 0);

let dureeTotaleArrosage = $state(null);

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

function dansPeriode(dateISO) {
	const annee = Number(dateISO.slice(0, 4));
	const mois = Number(dateISO.slice(5, 7));
	if (anneeMinNombre !== null && annee < anneeMinNombre) return false;
	if (anneeMaxNombre !== null && annee > anneeMaxNombre) return false;
	if (moisSelectionnesCultures.length > 0 && !moisSelectionnesCultures.includes(mois)) return false;
	return true;
}

$effect(() => {
	let total = 0;
	for (const culture of culturesSelectionnees) {
		const evenements = evenementsParCulture.get(culture.id) ?? [];
		for (const evenement of evenements) {
			if (evenement.type_evenement === 'Arrosage' && evenement.temps_arrosage && dansPeriode(evenement.date)) {
				total += evenement.temps_arrosage;
			}
		}
	}
	dureeTotaleArrosage = total;
});

let nombreEvenementsParType = $state(null);

let nombreImages = $state(null);

function chargerCultures() {
	invoke("lister_cultures_cmd")
		.then((result) => {
			allCultures = result;
		})
		.catch((error) => {
			console.error("Erreur lors de la récupération des cultures :", error);
		});
}

function chargerEvenements() {
	invoke("nombre_evenements_par_type_cmd")
		.then((result) => {
			nombreEvenementsParType = result;
		})
		.catch((error) => {
			console.error("Erreur lors de la récupération du nombre d'événements par type :", error);
		})
}

function compterEvenements(e) {
	if (!e) return 0;
	let res = 0;
	for (const k in e) {
		res += e[k];
	}
	return res;
}

function chargerNombreImages() {
	invoke("nombre_images_cmd")
		.then((result) => {
			nombreImages = result;
		})
		.catch((error) => {
			console.error("Erreur lors de la récupération du nombre d'images :", error);
		});
}

let evenementsTemperature = $state([]);
let chargementTemperatures = $state(true);

function chargerTemperatures() {
	chargementTemperatures = true;
	invoke("lister_evenements_cmd")
		.then((result) => {
			evenementsTemperature = result.filter((e) => e.type_evenement === 'Température');
		})
		.catch((error) => {
			console.error("Erreur lors de la récupération des températures :", error);
		})
		.finally(() => {
			chargementTemperatures = false;
		});
}

let anneeMinTemp = $state('');
let anneeMaxTemp = $state('');
let anneeMinTempNombre = $derived(anneeMinTemp === '' ? null : Number(anneeMinTemp));
let anneeMaxTempNombre = $derived(anneeMaxTemp === '' ? null : Number(anneeMaxTemp));
let moisSelectionnesTemp = $state([]);



$effect(() => {
	if (!statsCultures) {
		culturesSelectionnees = [];
	}
})

onMount(() => {
	chargerCultures();
	chargerEvenements();
	chargerNombreImages();
	chargerTemperatures();
});

</script>


<div class="p-6">
	<h1 class="text-lg font-semibold mb-6">Statistiques</h1>

	<div class="flex items-center justify-center gap-3 mb-6">
		<span
			class="text-sm font-medium transition-colors"
			class:text-green-700={!statsCultures}
			class:text-gray-400={statsCultures}
		>
			Statistiques générales
		</span>
		<Slider bind:checked={statsCultures} />
		<span
			class="text-sm font-medium transition-colors"
			class:text-green-700={statsCultures}
			class:text-gray-400={!statsCultures}
		>
			Statistiques par culture
		</span>
	</div>
</div>


<div>
	{#if statsCultures}
		<h3 class="text-sm font-semibold text-gray-700 mb-2">Cultures sélectionnées</h3>
		{#if allCultures.length > 0}
			<div class="flex flex-wrap gap-2">
				{#each allCultures as culture}
					<label
						class="flex items-center gap-1.5 px-2.5 py-1 rounded-full border border-gray-200 text-sm text-gray-700 cursor-pointer hover:bg-gray-50"
					>
						<input
							type="checkbox"
							bind:group={culturesSelectionnees}
							value={culture}
							class="rounded"
							style="accent-color: {culture.couleur}"
						/>
						<span class="inline-block w-2.5 h-2.5 rounded-full flex-none" style="background-color: {culture.couleur}"></span>
						{culture.nom}{culture.variete ? ` (${culture.variete})` : ''}
					</label>
				{/each}
			</div>
		{:else}
			<p class="text-gray-500 text-sm">Ajoutez des cultures pour voir leurs statistiques.</p>
		{/if}

		
		{#if culturesSelectionnees.length > 0}
			<h4 class="text-md text-center font-semibold text-gray-700 mt-6 mb-2">Par type d'événement</h4>

			<div class="flex items-center justify-center gap-2 mb-3 text-sm text-gray-600">
				<span>Période de</span>
				
				<select bind:value={anneeMin} class="rounded border-gray-200 py-1 text-sm">
					<option value="">Toutes les années</option>
					{#each ANNEES as annee}
						<option value={String(annee)}>{annee}</option>
					{/each}
				</select>
				<span>à</span>
				<select bind:value={anneeMax} class="rounded border-gray-200 py-1 text-sm">
					<option value="">Toutes les années</option>
					{#each ANNEES as annee}
						<option value={String(annee)}>{annee}</option>
					{/each}
				</select>
			</div>

			<div class="flex items-center justify-center gap-2 mb-3 text-sm text-gray-600">
				<span>Uniquement les mois de</span>
				<SelectMoisMultiple bind:mois={moisSelectionnesCultures} />
			</div>

			<div class="flex flex-wrap justify-center gap-x-4 gap-y-1.5 mb-3">
				{#each culturesSelectionnees as culture}
					<span class="flex items-center gap-1.5 text-sm text-gray-600">
						<span class="inline-block w-4 h-1" style="background-color: {culture.couleur}"></span>
						{culture.nom}{culture.variete ? ` (${culture.variete})` : ''}
					</span>
				{/each}
			</div>

			<p class="text-center text-sm text-gray-600 mb-4">
				Durée totale d'arrosage : <span class="font-semibold text-gray-800">{dureeTotaleArrosage} minutes</span>
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
							<p class="text-xs font-semibold text-gray-700">{type}</p>
						</div>
						<GraphiqueCultures
							cultures={culturesSelectionnees}
							evenementsParCulture={evenementsParCulture}
							enChargement={enChargementEvenements}
							type={type}
							anneeMin={anneeMinNombre}
							anneeMax={anneeMaxNombre}
							moisSelectionnes={moisSelectionnesCultures}
							compact
						/>
					</div>
				{/each}
			</div>

		{/if}

	{:else}
		<h3 class="text-lg font-semibold text-gray-800">Statistiques générales</h3>
		{#if nombreEvenementsParType != null && nombreImages !== null}
			<p>Nombre total de cultures : {allCultures.length}</p>
			<p>Nombre total d'événements : {compterEvenements(nombreEvenementsParType)}</p>
			<p>Nombre total d'événements journal : {nombreEvenementsParType.journal}</p>
			<p>Nombre total d'événements arrosage : {nombreEvenementsParType.arrosage}</p>
			<p>Nombre total d'événements récolte : {nombreEvenementsParType.recolte}</p>
			<p>Nombre total d'événements plantation : {nombreEvenementsParType.plantation}</p>
			<p>Nombre total d'événements semis : {nombreEvenementsParType.semis}</p>
			<p>Nombre total d'événements retrait : {nombreEvenementsParType.retrait}</p>
			<p>Nombre total d'événements température : {nombreEvenementsParType.temperature}</p>
			<p>Nombre total d'images : {nombreImages ? nombreImages : 0}</p>
		{:else}
			<p>Chargement des statistiques...</p>
		{/if}

		<h4 class="text-md text-center font-semibold text-gray-700 mt-6 mb-2">Température</h4>

		<div class="flex items-center justify-center gap-2 mb-3 text-sm text-gray-600">
			<span>Période de</span>
			<select bind:value={anneeMinTemp} class="rounded border-gray-200 py-1 text-sm">
				<option value="">Toutes les années</option>
				{#each ANNEES as annee}
					<option value={String(annee)}>{annee}</option>
				{/each}
			</select>
			<span>à</span>
			<select bind:value={anneeMaxTemp} class="rounded border-gray-200 py-1 text-sm">
				<option value="">Toutes les années</option>
				{#each ANNEES as annee}
					<option value={String(annee)}>{annee}</option>
				{/each}
			</select>
		</div>

		<div class="flex items-center justify-center gap-2 mb-3 text-sm text-gray-600">
			<span>Uniquement les mois de</span>
			<SelectMoisMultiple bind:mois={moisSelectionnesTemp} />
		</div>

		<GraphiqueTemperature
			evenements={evenementsTemperature}
			anneeMin={anneeMinTempNombre}
			anneeMax={anneeMaxTempNombre}
			moisSelectionnes={moisSelectionnesTemp}
			enChargement={chargementTemperatures}
		/>

	{/if}
</div>
