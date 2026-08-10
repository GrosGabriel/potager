<script>
	import { invoke } from '@tauri-apps/api/core';
	import { TYPES_EVENEMENT as typesEvenement, libelleCulture } from '$lib/utils.js';

	let { date = null, onsuccess = () => {}, oncancel = () => {} } = $props();


	function dateLocaleVersISO(d) {
		const annee = d.getFullYear();
		const mois = String(d.getMonth() + 1).padStart(2, '0');
		const jour = String(d.getDate()).padStart(2, '0');
		return `${annee}-${mois}-${jour}`;
	}

	function dateVersISO(d) {
		if (!d) return dateLocaleVersISO(new Date());
		return d instanceof Date ? dateLocaleVersISO(d) : d;
	}

	function isoVersAffichage(iso) {
		if (!iso) return '';
		const [annee, mois, jour] = iso.split('-');
		return `${jour}/${mois}/${annee}`;
	}

	const regexDateAffichee = /^(\d{2})\/(\d{2})\/(\d{4})$/;

	function affichageVersISO(texte) {
		const correspondance = regexDateAffichee.exec(texte.trim());
		if (!correspondance) return null;
		const [, jour, mois, annee] = correspondance;
		const j = Number(jour);
		const m = Number(mois);
		const a = Number(annee);
		const d = new Date(a, m - 1, j);
		if (d.getFullYear() !== a || d.getMonth() !== m - 1 || d.getDate() !== j) return null;
		return `${annee}-${mois}-${jour}`;
	}

	let cultures = $state([]);
	let culturesTriees = $derived(
		[...cultures].sort((a, b) =>
			`${a.nom} ${a.variete ?? ''}`.localeCompare(`${b.nom} ${b.variete ?? ''}`, 'fr')
		)
	);
	let chargementCultures = $state(true);
	let erreurCultures = $state('');

	let cultureId = $state('');
	let typeEvenement = $state(typesEvenement[0]);
	let dateTexte = $derived(isoVersAffichage(dateVersISO(date)));
	let notes = $state('');
	let temperatureInt = $state(null);
	let temperatureExt = $state(null);
	let tempsArrosage = $state(null);
	let arrosageAutomatique = $state(false);
	let periodeArrosage = $state(null);

    let newCultureNom = $state('');
    let newCultureVariete = $state('');
    let newCultureCouleur = $state('#00ff00');

	let envoiEnCours = $state(false);
	let erreurEnvoi = $state('');

	let imagesSelectionnees = $state([]);
	let previsualisations = $derived(imagesSelectionnees.map((fichier) => URL.createObjectURL(fichier)));

	$effect(() => {
		const urls = previsualisations;
		return () => {
			urls.forEach((url) => URL.revokeObjectURL(url));
		};
	});

	function surChangementImages(event) {
		imagesSelectionnees = [...imagesSelectionnees, ...Array.from(event.target.files)];
		event.target.value = '';
	}

	function retirerImage(index) {
		imagesSelectionnees = imagesSelectionnees.filter((_, i) => i !== index);
	}

	function lireFichierEnDataURL(fichier) {
		return new Promise((resolve, reject) => {
			const lecteur = new FileReader();
			lecteur.onload = () => resolve(lecteur.result);
			lecteur.onerror = () => reject(lecteur.error);
			lecteur.readAsDataURL(fichier);
		});
	}



	let dateEvenementISO = $derived(affichageVersISO(dateTexte));
	let dateInvalide = $derived(dateTexte.trim() !== '' && dateEvenementISO === null);

	$effect(() => {
		invoke('lister_cultures_cmd')
			.then((result) => {
				cultures = result;
				if (result.length > 0) {
					cultureId = String(result[0].id);
				}
			})
			.catch((error) => {
				console.error('Erreur lors de la récupération des cultures :', error);
				erreurCultures = "Impossible de charger les cultures." + error;
			})
			.finally(() => {
				chargementCultures = false;
			});
	});

	let formValide = $derived((cultureId !== '' || typeEvenement == "Journal" || typeEvenement == "Température" ) && typeEvenement !== '' && dateEvenementISO !== null);

	async function soumettre(event) {
		event.preventDefault();
		if (!formValide || envoiEnCours) return;

		envoiEnCours = true;
		erreurEnvoi = '';

		try {

            if (typeEvenement !== "Journal" && typeEvenement !== "Température" && cultureId === "new_culture") {
                await invoke('ajouter_culture_cmd', {
                    nom: newCultureNom.trim(),
                    variete: newCultureVariete.trim() === '' ? null : newCultureVariete.trim(),
                    couleur: newCultureCouleur
                });
                // Récupérer la nouvelle culture pour obtenir son ID
                const culturesMisesAJour = await invoke('lister_cultures_cmd');
                cultures = culturesMisesAJour;
                const nouvelleCulture = culturesMisesAJour.find(c => c.nom === newCultureNom.trim() && c.variete === (newCultureVariete.trim() === '' ? null : newCultureVariete.trim()));
                if (nouvelleCulture) {
                    cultureId = String(nouvelleCulture.id);
                } else {
                    throw new Error("Impossible de récupérer l'ID de la nouvelle culture.");
                }
            };

            const parametresEvenement = {
				cultureId: typeEvenement === "Journal" || typeEvenement === "Température" ? null : Number(cultureId),
				typeEvent: typeEvenement,
				date: dateEvenementISO,
				notes: notes.trim() === '' ? null : notes.trim(),
				temperatureInt: temperatureInt !== null ? Number(temperatureInt) : null,
				temperatureExt: temperatureExt !== null ? Number(temperatureExt) : null,
				tempsArrosage: tempsArrosage !== null ? Number(tempsArrosage) : null,
			};

			// Arrosage automatique : le premier jour et tous les jours suivants
			// sont créés en une seule transaction côté Rust (voir
			// ajouter_evenement_avec_repetition) : soit tout est enregistré,
			// soit rien ne l'est en cas d'erreur en cours de route.
			const repeterArrosage =
				typeEvenement === "Arrosage" && arrosageAutomatique && periodeArrosage !== null && periodeArrosage > 0;

			const evenementId = repeterArrosage
				? await invoke('ajouter_evenement_avec_repetition_cmd', { ...parametresEvenement, nombreJours: periodeArrosage })
				: await invoke('ajouter_evenement_cmd', parametresEvenement);

			for (const fichier of imagesSelectionnees) {
				const donnees = await lireFichierEnDataURL(fichier);
				await invoke('ajouter_image_cmd', {
					evenementId,
					nomFichier: fichier.name,
					donnees
				});
			}

			imagesSelectionnees = [];
			notes = '';
			typeEvenement = typesEvenement[0];
			cultureId = '';
			onsuccess();
		} catch (error) {
			console.error("Erreur lors de l'ajout de l'événement :", error);
			erreurEnvoi = "Impossible d'enregistrer l'événement. Veuillez réessayer.";
		} finally {
			envoiEnCours = false;
		}
	}

$effect(() => {
	if (typeEvenement === "Journal" || typeEvenement === "Température") {
		cultureId = '';
		newCultureNom = '';
		newCultureVariete = '';
		newCultureCouleur = '#00ff00';
	}
	if (typeEvenement !== "Arrosage") {
		tempsArrosage = null;
		arrosageAutomatique = false;
		periodeArrosage = null;
	}
})

</script>

<form onsubmit={soumettre} class="w-[min(40rem,80vw)] max-h-[88vh] overflow-y-auto space-y-5 px-1">
    <h2 class="text-xl font-semibold text-gray-800">Ajouter un événement</h2>

    <div class="grid grid-cols-2 gap-4">
        <div>
            <label for="type_evenement" class="block text-base font-medium text-gray-700 mb-1">Type d'événement</label>
            <select id="type_evenement" bind:value={typeEvenement} required class="w-full rounded border-gray-200 text-base py-2">
                {#each typesEvenement as type}
                    <option value={type}>{type}</option>
                {/each}
            </select>
        </div>

        <div>
			<label for="date" class="block text-base font-medium text-gray-700 mb-1">Date</label>
			<input
				id="date"
				type="text"
				inputmode="numeric"
				placeholder="JJ/MM/AAAA"
				bind:value={dateTexte}
				required
				class="w-full rounded border-gray-200 text-base py-2"
				class:border-red-400={dateInvalide}
			/>
			{#if dateInvalide}
				<p class="text-sm text-red-600 mt-1">Format attendu : JJ/MM/AAAA</p>
			{/if}
		</div>
    </div>

    {#if typeEvenement !== "Journal" && typeEvenement !== "Température"}
        <div class="flex justify-center">
          <div class="w-fit">
            <label for="culture" class="block text-base font-medium text-gray-700 mb-1">Culture</label>
            <select id="culture" bind:value={cultureId} required class="rounded border-gray-200 text-base py-2">
                {#each culturesTriees as culture}
                    <option value={String(culture.id)}>
                        {libelleCulture(culture)}
                    </option>
                {/each}
                <option value={"new_culture"}>Ajouter une nouvelle culture</option>
            </select>
          </div>
        </div>

        {#if cultureId === "new_culture"}
            <div>
                <label for="new_culture_nom" class="block text-base font-medium text-gray-700 mb-1">Nom de la nouvelle culture</label>
                <input
                    id="new_culture_nom"
                    type="text"
                    bind:value={newCultureNom}
                    required
                    class="w-full rounded border-gray-200 text-base py-2"
                />
            </div>

            <div>
                <label for="new_culture_variete" class="block text-base font-medium text-gray-700 mb-1">Variété</label>
                <input
                    id="new_culture_variete"
                    type="text"
                    bind:value={newCultureVariete}
                    class="w-full rounded border-gray-200 text-base py-2"
                />
            </div>

            <div>
                <label for="new_culture_couleur" class="block text-base font-medium text-gray-700 mb-1">Couleur</label>
                <input
                    id="new_culture_couleur"
                    type="color"
                    bind:value={newCultureCouleur}
                    required
                    class="h-9 w-16 rounded border-gray-200"
                />
            </div>

        {/if}

    {/if}

	{#if typeEvenement === "Température"}
		<div class="grid grid-cols-2 gap-4">
			<div>
				<label for="temperature_int" class="block text-base font-medium text-gray-700 mb-1">Température intérieure (°C)</label>
				<input
					id="temperature_int"
					type="number"
					step="0.1"
					bind:value={temperatureInt}
					required
					class="w-full rounded border-gray-200 text-base py-2"
				/>
			</div>
			<div>
				<label for="temperature_ext" class="block text-base font-medium text-gray-700 mb-1">Température extérieure (°C)</label>
				<input
					id="temperature_ext"
					type="number"
					step="0.1"
					bind:value={temperatureExt}
					required
					class="w-full rounded border-gray-200 text-base py-2"
				/>
			</div>
		</div>
	{/if}

	{#if typeEvenement === "Arrosage"}
		<div class="flex items-center gap-6 flex-wrap">
			<div class="flex items-center gap-2 pl-4 pr-20 py-2.5">
				<input
					id="arrosage_automatique"
					type="checkbox"
					bind:checked={arrosageAutomatique}
					class="rounded"
				/>
				<label for="arrosage_automatique" class="text-base font-medium text-gray-700">Arrosage automatique</label>
			</div>
			{#if arrosageAutomatique}
				<div class="flex items-center gap-2">
					<!--Il faudrait un selecteur pour choisir plusieurs jours d'arrosages autour de la date choisie-->
					<label for="période_arrosage" class="text-base font-medium text-gray-700 whitespace-nowrap">Répéter (jours)</label>
					<input
						id="période_arrosage"
						type="number"
						step="1"
						min="2"
						bind:value={periodeArrosage}
						required
						class="w-20 rounded border-gray-200 text-base py-2"
					/>
				</div>
			{/if}
		</div>
		<div>
			<label for="temps_arrosage" class="block text-base font-medium text-gray-700 mb-1">Durée d'arrosage (minutes)</label>
			<input
				id="temps_arrosage"
				type="number"
				step="1"
				min="1"
				bind:value={tempsArrosage}
				required
				class="w-full rounded border-gray-200 text-base py-2"
			/>
		</div>
	{/if}


    <div>
        <label for="notes" class="block text-base font-medium text-gray-700 mb-1">Notes</label>
        <textarea
            id="notes"
            bind:value={notes}
            rows="3"
            placeholder="Détails, quantités, observations…"
            class="w-full rounded border-gray-200 text-base py-2"
        ></textarea>
    </div>

    <div>
        <p class="block text-base font-medium text-gray-700 mb-1">Photos</p>
        <div class="flex items-center gap-3">
            <label
                for="images"
                class="inline-flex items-center px-4 py-2 rounded border border-green-200 bg-green-50 text-base text-green-700 hover:bg-green-100 cursor-pointer"
            >
                Choisir des photos
            </label>
            <span class="text-base text-gray-500">
                {imagesSelectionnees.length > 0
                    ? `${imagesSelectionnees.length} photo${imagesSelectionnees.length > 1 ? 's' : ''} sélectionnée${imagesSelectionnees.length > 1 ? 's' : ''}`
                    : 'Aucune photo sélectionnée'}
            </span>
            <input
                id="images"
                type="file"
                accept="image/*"
                multiple
                onchange={surChangementImages}
                class="hidden"
            />
        </div>
        {#if imagesSelectionnees.length > 0}
            <div class="flex flex-wrap gap-2 mt-2">
                {#each previsualisations as url, index}
                    <div class="relative w-16 h-16">
                        <img src={url} alt="" class="w-full h-full object-cover rounded" />
                        <button
                            type="button"
                            onclick={() => retirerImage(index)}
                            class="absolute -top-1.5 -right-1.5 bg-white border border-gray-300 rounded-full w-5 h-5 flex items-center justify-center text-sm text-gray-600 hover:bg-gray-100"
                            aria-label="Retirer cette photo"
                        >
                            ×
                        </button>
                    </div>
                {/each}
            </div>
        {/if}
    </div>

    {#if erreurCultures}
		<p class="text-base text-red-600">{erreurCultures}</p>
	{/if}
	{#if erreurEnvoi}
		<p class="text-base text-red-600">{erreurEnvoi}</p>
	{/if}

	<div class="flex justify-end gap-3 pt-2">
		<button
			type="button"
			onclick={oncancel}
			class="px-5 py-2.5 text-base rounded text-gray-600 hover:bg-gray-100"
		>
			Annuler
		</button>
		<button
			type="submit"
			disabled={!formValide || envoiEnCours}
			class="bg-green-600 text-white px-5 py-2.5 text-base rounded hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed"
		>
			{envoiEnCours ? 'Enregistrement…' : "Enregistrer"}
		</button>
	</div>

</form>
