<script>
	import { COULEURS_PREMIERES_DATES_SAISON_EVENEMENTS } from '$lib/utils.js';

	let { culture, evenements = [], anneeMin = null, anneeMax = null, enChargement = false } = $props();

	const TYPES = ['Semis', 'Plantation', 'Floraison', 'Récolte'];
	const COULEURS = COULEURS_PREMIERES_DATES_SAISON_EVENEMENTS;

	// Positions des débuts de mois en "jour de l'année", calculées sur une
	// année de référence non bissextile (les repères de l'axe ne bougent
	// pas selon les données affichées).
	const ANNEE_REFERENCE = 2025;
	const NOMS_MOIS_COURT = ['Jan', 'Fév', 'Mar', 'Avr', 'Mai', 'Jun', 'Jul', 'Aoû', 'Sep', 'Oct', 'Nov', 'Déc'];
	const DEBUT_MOIS = NOMS_MOIS_COURT.map((_, i) => {
		const debutAnnee = new Date(ANNEE_REFERENCE, 0, 0);
		const debutMois = new Date(ANNEE_REFERENCE, i, 1);
		return Math.round((debutMois - debutAnnee) / 86400000);
	});

	function jourDeLAnnee(dateISO) {
		const [annee, mois, jour] = dateISO.split('-').map(Number);
		const debutAnnee = new Date(annee, 0, 0);
		const date = new Date(annee, mois - 1, jour);
		return Math.round((date - debutAnnee) / 86400000);
	}

	const formatteurDate = new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'long' });
	function libelleDate(dateISO) {
		const [annee, mois, jour] = dateISO.split('-').map(Number);
		return formatteurDate.format(new Date(annee, mois - 1, jour));
	}

	function dansPeriode(annee) {
		if (anneeMin !== null && annee < anneeMin) return false;
		if (anneeMax !== null && annee > anneeMax) return false;
		return true;
	}

	// Pour chaque année et chaque type, la date (et le jour de l'année) du
	// tout premier événement de ce type cette année-là.
	let premieresDates = $derived.by(() => {
		const carte = new Map(); // "annee-type" -> { date, jour }
		for (const evenement of evenements) {
			if (!TYPES.includes(evenement.type_evenement)) continue;
			const annee = Number(evenement.date.slice(0, 4));
			if (!dansPeriode(annee)) continue;
			const cle = `${annee}-${evenement.type_evenement}`;
			const existant = carte.get(cle);
			if (!existant || evenement.date < existant.date) {
				carte.set(cle, { date: evenement.date, jour: jourDeLAnnee(evenement.date) });
			}
		}
		return carte;
	});

	let annees = $derived(
		[...new Set(evenements.map((e) => Number(e.date.slice(0, 4))).filter(dansPeriode))].sort()
	);

	const LARGEUR = 400;
	const HAUTEUR = 220;
	const MARGE_GAUCHE = 32;
	const MARGE_DROITE = 8;
	const MARGE_HAUT = 10;
	const MARGE_BAS = 20;
	const LARGEUR_TRACE = LARGEUR - MARGE_GAUCHE - MARGE_DROITE;
	const HAUTEUR_TRACE = HAUTEUR - MARGE_HAUT - MARGE_BAS;
	const JOUR_MAX = 366;

	function y(jour) {
		return MARGE_HAUT + HAUTEUR_TRACE - (jour / JOUR_MAX) * HAUTEUR_TRACE;
	}

	let largeurGroupe = $derived(annees.length > 0 ? LARGEUR_TRACE / annees.length : 0);
	const LARGEUR_BARRE_MAX = 14;
	const ESPACE_BARRE = 2;

	let barres = $derived.by(() => {
		const largeurBarre = Math.min(LARGEUR_BARRE_MAX, (largeurGroupe - 2 * ESPACE_BARRE) / TYPES.length);
		const largeurTotaleBarres = largeurBarre * TYPES.length;
		return annees.flatMap((annee, indexAnnee) => {
			const debutGroupe = MARGE_GAUCHE + indexAnnee * largeurGroupe + (largeurGroupe - largeurTotaleBarres) / 2;
			return TYPES.map((type, indexType) => {
				const info = premieresDates.get(`${annee}-${type}`);
				if (!info) return null;
				return {
					annee,
					type,
					date: info.date,
					jour: info.jour,
					x: debutGroupe + indexType * largeurBarre,
					largeur: largeurBarre,
					y: y(info.jour),
					hauteur: HAUTEUR - MARGE_BAS - y(info.jour)
				};
			}).filter((b) => b !== null);
		});
	});

	let survol = $state(null);
	let afficherTableau = $state(false);
</script>

<div>
	{#if enChargement}
		<p class="text-gray-500 text-lg">Chargement…</p>
	{:else if annees.length === 0}
		<p class="text-gray-500 text-lg">Aucune donnée sur cette période.</p>
	{:else}
		<div class="relative">
			<svg viewBox="0 0 {LARGEUR} {HAUTEUR}" class="w-full h-auto" role="img" aria-label="Première plantation, semis et récolte par année pour {culture.nom}">
				{#each DEBUT_MOIS as jour, i}
					<line
						x1={MARGE_GAUCHE}
						x2={LARGEUR - MARGE_DROITE}
						y1={y(jour)}
						y2={y(jour)}
						class="stroke-gray-200"
						stroke-width="1"
					/>
					<text x={MARGE_GAUCHE - 6} y={y(jour)} text-anchor="end" dominant-baseline="middle" class="fill-gray-400" font-size="8">
						{NOMS_MOIS_COURT[i]}
					</text>
				{/each}

				{#each annees as annee, i}
					<text
						x={MARGE_GAUCHE + i * largeurGroupe + largeurGroupe / 2}
						y={HAUTEUR - MARGE_BAS + 12}
						text-anchor="middle"
						class="fill-gray-400"
						font-size="8"
					>
						{annee}
					</text>
				{/each}

				{#each barres as barre}
					<rect
						x={barre.x}
						y={barre.y}
						width={barre.largeur}
						height={Math.max(barre.hauteur, 1)}
						rx="2"
						fill={COULEURS[barre.type]}
						aria-hidden="true"
						onpointerenter={() => (survol = barre)}
						onpointerleave={() => (survol = null)}
					/>
				{/each}
			</svg>

			{#if survol}
				{@const pourcentGauche = ((survol.x + survol.largeur / 2) / LARGEUR) * 100}
				{@const versLaGauche = pourcentGauche > 50}
				<div
					class="absolute top-1 bg-white border border-gray-200 rounded shadow-sm px-4 py-3 pointer-events-none w-max max-w-[18rem] space-y-1"
					style={versLaGauche ? `right: ${100 - pourcentGauche}%` : `left: ${pourcentGauche}%`}
				>
					<div class="flex items-center gap-2">
						<span class="inline-block w-3 h-3 rounded-full flex-none" style="background-color: {COULEURS[survol.type]}"></span>
						<span class="text-gray-500 text-lg">{survol.type}</span>
					</div>
					<p class="font-semibold text-lg text-gray-800">{libelleDate(survol.date)} {survol.annee}</p>
				</div>
			{/if}
		</div>

		<button
			type="button"
			class="text-green-700 hover:underline text-md mt-1.5"
			onclick={() => (afficherTableau = !afficherTableau)}
		>
			{afficherTableau ? 'Masquer' : 'Afficher'} les données en tableau
		</button>

		{#if afficherTableau}
			<div class="overflow-x-auto mt-2">
				<table class="w-full text-lg">
					<thead>
						<tr>
							<th class="text-left text-gray-500 font-medium pr-3 py-1">Année</th>
							{#each TYPES as type}
								<th class="text-left text-gray-500 font-medium pr-3 py-1">{type}</th>
							{/each}
						</tr>
					</thead>
					<tbody>
						{#each annees as annee}
							<tr class="border-t border-gray-100">
								<td class="pr-3 py-1 text-gray-700">{annee}</td>
								{#each TYPES as type}
									{@const info = premieresDates.get(`${annee}-${type}`)}
									<td class="pr-3 py-1 text-gray-700">{info ? libelleDate(info.date) : '—'}</td>
								{/each}
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	{/if}
</div>
