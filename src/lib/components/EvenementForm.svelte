<script>
	import { invoke } from '@tauri-apps/api/core';
	import { TYPES_EVENEMENT as typesEvenement } from '$lib/evenementsColor.js';

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
	let chargementCultures = $state(true);
	let erreurCultures = $state('');

	let cultureId = $state('');
	let typeEvenement = $state(typesEvenement[0]);
	let dateTexte = $derived(isoVersAffichage(dateVersISO(date)));
	let notes = $state('');

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
				erreurCultures = "Impossible de charger les cultures.";
			})
			.finally(() => {
				chargementCultures = false;
			});
	});

	let formValide = $derived((cultureId !== '' || typeEvenement == "Journal") && typeEvenement !== '' && dateEvenementISO !== null);

	async function soumettre(event) {
		event.preventDefault();
		if (!formValide || envoiEnCours) return;

		envoiEnCours = true;
		erreurEnvoi = '';

		try {
			
            if (typeEvenement !== "Journal" && cultureId === "new_culture") {
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
            
            const evenementId = await invoke('ajouter_evenement_cmd', {
				cultureId: typeEvenement === "Journal" ? null : Number(cultureId),
				typeEvent: typeEvenement,
				date: dateEvenementISO,
				notes: notes.trim() === '' ? null : notes.trim()
			});

			for (const fichier of imagesSelectionnees) {
				const donnees = await lireFichierEnDataURL(fichier);
				await invoke('ajouter_image_cmd', {
					evenementId,
					nomFichier: fichier.name,
					donnees
				});
			}
			imagesSelectionnees = [];

			onsuccess();
		} catch (error) {
			console.error("Erreur lors de l'ajout de l'événement :", error);
			erreurEnvoi = "Impossible d'enregistrer l'événement. Veuillez réessayer.";
		} finally {
			envoiEnCours = false;
		}
	}
</script>

<form onsubmit={soumettre}>
    <h2>Ajouter un événement</h2>

    <div>
        <label for="type_evenement">Type d'événement</label>
        <select id="type_evenement" bind:value={typeEvenement} required>
            {#each typesEvenement as type}
                <option value={type}>{type}</option>
            {/each}
        </select>
    </div>

    <div>
		<label for="date">Date</label>
		<input
			id="date"
			type="text"
			inputmode="numeric"
			placeholder="JJ/MM/AAAA"
			bind:value={dateTexte}
			required
			class:border-red-400={dateInvalide}
		/>
		{#if dateInvalide}
			<p class="text-xs text-red-600 mt-1">Format attendu : JJ/MM/AAAA</p>
		{/if}
	</div>

    {#if typeEvenement !== "Journal"}
        <div>
            <label for="culture">Culture</label>
            <select id="culture" bind:value={cultureId} required>
                {#each cultures as culture}
                    <option value={String(culture.id)}>
                        {culture.nom}{culture.variete ? ` (${culture.variete})` : ''}
                    </option>
                {/each}
                <option value={"new_culture"}>Ajouter une nouvelle culture</option>
            </select>
        </div>

        {#if cultureId === "new_culture"}
            <div>
                <label for="new_culture_nom">Nom de la nouvelle culture</label>
                <input
                    id="new_culture_nom"
                    type="text"
                    bind:value={newCultureNom}
                    required
                />
            </div>

            <div>
                <label for="new_culture_variete">Variété (optionnel)</label>
                <input
                    id="new_culture_variete"
                    type="text"
                    bind:value={newCultureVariete}
                />
            </div>

            <div>
                <label for="new_culture_couleur">Couleur</label>
                <input
                    id="new_culture_couleur"
                    type="color"
                    bind:value={newCultureCouleur}
                    required
                />
            </div>

        {/if}

    {/if}

    <div>
        <label for="notes">Notes (optionnel)</label>
        <textarea
            id="notes"
            bind:value={notes}
            rows="3"
            placeholder="Détails, quantités, observations…"
        ></textarea>
    </div>

    <div>
        <label for="images">Photos (optionnel)</label>
        <input
            id="images"
            type="file"
            accept="image/*"
            multiple
            onchange={surChangementImages}
        />
        {#if imagesSelectionnees.length > 0}
            <div class="flex flex-wrap gap-2 mt-2">
                {#each previsualisations as url, index}
                    <div class="relative w-16 h-16">
                        <img src={url} alt="" class="w-full h-full object-cover rounded" />
                        <button
                            type="button"
                            onclick={() => retirerImage(index)}
                            class="absolute -top-1.5 -right-1.5 bg-white border border-gray-300 rounded-full w-5 h-5 flex items-center justify-center text-xs text-gray-600 hover:bg-gray-100"
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
		<p class="text-sm text-red-600">{erreurCultures}</p>
	{/if}
	{#if erreurEnvoi}
		<p class="text-sm text-red-600">{erreurEnvoi}</p>
	{/if}

	<div class="flex justify-end gap-2 pt-2">
		<button
			type="button"
			onclick={oncancel}
			class="px-4 py-2 rounded text-gray-600 hover:bg-gray-100"
		>
			Annuler
		</button>
		<button
			type="submit"
			disabled={!formValide || envoiEnCours} 
			class="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed"
		>
			{envoiEnCours ? 'Enregistrement…' : "Enregistrer"}
		</button>
	</div>

</form>
