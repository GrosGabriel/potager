<script>
	import { COULEUR_PAR_TYPE } from '$lib/utils.js';

	let { evenements = [], anneeMin = null, anneeMax = null, moisSelectionnes = [], enChargement = false } = $props();

	const COULEUR_EXTERIEURE = COULEUR_PAR_TYPE.Température;
	const COULEUR_INTERIEURE = '#0ea5e9';

	let survolIndex = $state(null);
	let afficherTableau = $state(false);

	function cleMois(dateISO) {
		return dateISO.slice(0, 7); // "AAAA-MM"
	}

	const formatteurMois = new Intl.DateTimeFormat('fr-FR', { month: 'short', year: 'numeric' });
	function libelleMois(cle) {
		const [annee, mois] = cle.split('-').map(Number);
		return formatteurMois.format(new Date(annee, mois - 1, 1));
	}

	function dansPeriode(dateISO) {
		const annee = Number(dateISO.slice(0, 4));
		const mois = Number(dateISO.slice(5, 7));
		if (anneeMin !== null && annee < anneeMin) return false;
		if (anneeMax !== null && annee > anneeMax) return false;
		if (moisSelectionnes.length > 0 && !moisSelectionnes.includes(mois)) return false;
		return true;
	}

	let evenementsFiltres = $derived(evenements.filter((e) => dansPeriode(e.date)));

	// Min et max, par mois, des relevés intérieurs et extérieurs.
	let statsParMois = $derived.by(() => {
		const maxInt = new Map();
		const minInt = new Map();
		const maxExt = new Map();
		const minExt = new Map();
		for (const evenement of evenementsFiltres) {
			const cle = cleMois(evenement.date);
			if (evenement.temperature_int !== null && evenement.temperature_int !== undefined) {
				maxInt.set(cle, Math.max(maxInt.get(cle) ?? -Infinity, evenement.temperature_int));
				minInt.set(cle, Math.min(minInt.get(cle) ?? Infinity, evenement.temperature_int));
			}
			if (evenement.temperature_ext !== null && evenement.temperature_ext !== undefined) {
				maxExt.set(cle, Math.max(maxExt.get(cle) ?? -Infinity, evenement.temperature_ext));
				minExt.set(cle, Math.min(minExt.get(cle) ?? Infinity, evenement.temperature_ext));
			}
		}
		return { maxInt, minInt, maxExt, minExt };
	});

	let series = $derived([
		{ label: 'Intérieure max', couleur: COULEUR_INTERIEURE, pointille: false, valeurs: statsParMois.maxInt },
		{ label: 'Intérieure min', couleur: COULEUR_INTERIEURE, pointille: true, valeurs: statsParMois.minInt },
		{ label: 'Extérieure max', couleur: COULEUR_EXTERIEURE, pointille: false, valeurs: statsParMois.maxExt },
		{ label: 'Extérieure min', couleur: COULEUR_EXTERIEURE, pointille: true, valeurs: statsParMois.minExt }
	]);

	let moisAxe = $derived(
		[...new Set(series.flatMap((s) => [...s.valeurs.keys()]))].sort()
	);

	let toutesValeurs = $derived(series.flatMap((s) => moisAxe.map((m) => s.valeurs.get(m)).filter((v) => v !== undefined)));

	let yMin = $derived(toutesValeurs.length ? Math.floor(Math.min(...toutesValeurs)) - 1 : 0);
	let yMax = $derived(toutesValeurs.length ? Math.ceil(Math.max(...toutesValeurs)) + 1 : 10);

	const LARGEUR = 640;
	const HAUTEUR = 260;
	const MARGE_GAUCHE = 40;
	const MARGE_DROITE = 12;
	const MARGE_HAUT = 12;
	const MARGE_BAS = 52;
	const LARGEUR_TRACE = LARGEUR - MARGE_GAUCHE - MARGE_DROITE;
	const HAUTEUR_TRACE = HAUTEUR - MARGE_HAUT - MARGE_BAS;

	function x(index) {
		if (moisAxe.length <= 1) return MARGE_GAUCHE + LARGEUR_TRACE / 2;
		return MARGE_GAUCHE + (index / (moisAxe.length - 1)) * LARGEUR_TRACE;
	}

	function y(valeur) {
		return MARGE_HAUT + HAUTEUR_TRACE - ((valeur - yMin) / (yMax - yMin)) * HAUTEUR_TRACE;
	}

	let pointsParSerie = $derived(
		series.map((s) => ({
			...s,
			points: moisAxe
				.map((m, i) => {
					const valeur = s.valeurs.get(m);
					return valeur === undefined ? null : { mois: m, valeur, x: x(i), y: y(valeur) };
				})
				.filter((p) => p !== null)
		}))
	);

	let ticksY = $derived.by(() => {
		const nombre = 4;
		return Array.from({ length: nombre + 1 }, (_, i) => Math.round(yMin + ((yMax - yMin) / nombre) * i));
	});

	let pivoterLibellesX = $derived(moisAxe.length >= 8);
	let tailleLibellesX = $derived(Math.max(4, (8 * 8) / Math.max(8, moisAxe.length)));

	function surSurvol(event) {
		if (moisAxe.length === 0) return;
		const rect = event.currentTarget.getBoundingClientRect();
		const ratio = (event.clientX - rect.left) / rect.width;
		const xSvg = ratio * LARGEUR;
		let meilleurIndex = 0;
		let meilleureDistance = Infinity;
		moisAxe.forEach((_, i) => {
			const distance = Math.abs(x(i) - xSvg);
			if (distance < meilleureDistance) {
				meilleureDistance = distance;
				meilleurIndex = i;
			}
		});
		survolIndex = meilleurIndex;
	}

	function surSortie() {
		survolIndex = null;
	}
</script>

<div class="border border-gray-200 rounded-lg p-4">
	{#if enChargement}
		<p class="text-gray-500 text-sm">Chargement…</p>
	{:else if moisAxe.length === 0}
		<p class="text-gray-500 text-sm">Aucun relevé de température enregistré sur cette période.</p>
	{:else}
		<div class="relative">
			<svg
				viewBox="0 0 {LARGEUR} {HAUTEUR}"
				class="w-full h-auto"
				role="img"
				aria-label="Températures minimale et maximale, intérieure et extérieure, par mois"
			>
				{#each ticksY as valeur}
					<line
						x1={MARGE_GAUCHE}
						x2={LARGEUR - MARGE_DROITE}
						y1={y(valeur)}
						y2={y(valeur)}
						class="stroke-gray-200"
						stroke-width="1"
					/>
					<text x={MARGE_GAUCHE - 6} y={y(valeur)} text-anchor="end" dominant-baseline="middle" class="fill-gray-400" font-size="10">
						{valeur}°
					</text>
				{/each}

				{#each moisAxe as mois, i}
					<text
						x={x(i)}
						y={HAUTEUR - MARGE_BAS + 16}
						text-anchor={pivoterLibellesX || i === moisAxe.length - 1 ? 'end' : 'middle'}
						transform={pivoterLibellesX ? `rotate(-45 ${x(i)} ${HAUTEUR - MARGE_BAS + 16})` : undefined}
						class="fill-gray-400"
						font-size={tailleLibellesX}
					>
						{libelleMois(mois)}
					</text>
				{/each}

				{#if survolIndex !== null}
					<line
						x1={x(survolIndex)}
						x2={x(survolIndex)}
						y1={MARGE_HAUT}
						y2={HAUTEUR - MARGE_BAS}
						class="stroke-gray-400"
						stroke-width="1"
					/>
				{/if}

				{#each pointsParSerie as serie}
					<path
						d={serie.points.map((p, i) => `${i === 0 ? 'M' : 'L'} ${p.x} ${p.y}`).join(' ')}
						fill="none"
						stroke={serie.couleur}
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-dasharray={serie.pointille ? '5,4' : undefined}
					/>
					{#each serie.points as p}
						<circle cx={p.x} cy={p.y} r="4" fill={serie.couleur} stroke="white" stroke-width="2" />
					{/each}
				{/each}

				<rect
					x={MARGE_GAUCHE}
					y={MARGE_HAUT}
					width={LARGEUR_TRACE}
					height={HAUTEUR_TRACE}
					fill="transparent"
					aria-hidden="true"
					onpointermove={surSurvol}
					onpointerleave={surSortie}
				/>
			</svg>

			{#if survolIndex !== null}
				{@const pourcentGauche = (x(survolIndex) / LARGEUR) * 100}
				{@const versLaGauche = pourcentGauche > 50}
				<div
					class="absolute top-2 bg-white border border-gray-200 rounded shadow-sm px-4 py-3 pointer-events-none w-max max-w-[18rem] space-y-1"
					style={versLaGauche ? `right: ${100 - pourcentGauche}%` : `left: ${pourcentGauche}%`}
				>
					<p class="font-medium text-lg text-gray-700 mb-1">{libelleMois(moisAxe[survolIndex])}</p>
					{#each pointsParSerie as serie}
						{@const point = serie.points.find((p) => p.mois === moisAxe[survolIndex])}
						<div class="flex items-center gap-2">
							<span
								class="inline-block w-4 shrink-0"
								style="height: 0; border-top: 2px {serie.pointille ? 'dashed' : 'solid'} {serie.couleur};"
							></span>
							<span class="text-gray-500 text-lg">{serie.label}</span>
							<span class="font-semibold text-lg text-gray-800">{point ? `${point.valeur.toFixed(1)}°C` : '—'}</span>
						</div>
					{/each}
				</div>
			{/if}
		</div>

		<!--Légende-->
		<div class="flex flex-wrap gap-x-4 gap-y-1.5 mt-3">
			{#each series as serie}
				<span class="flex items-center gap-1.5 text-md text-gray-600">
					<span
						class="inline-block w-8 "
						style="
							border-top: 4px {serie.pointille ? 'dashed' : 'solid'} {serie.couleur};
						"
					></span>
					{serie.label}
				</span>
			{/each}
		</div>
		<!--<span class="inline-block w-4 h-1" style="background-color: {culture.couleur}"></span>
						{culture.nom}{culture.variete ? ` (${culture.variete})` : ''}-->

		<button
			type="button"
			class="text-md text-green-700 hover:underline mt-3"
			onclick={() => (afficherTableau = !afficherTableau)}
		>
			{afficherTableau ? 'Masquer' : 'Afficher'} les données en tableau
		</button>

		{#if afficherTableau}
			<div class="overflow-x-auto mt-2">
				<table class="text-lg w-full">
					<thead>
						<tr>
							<th class="text-left text-gray-500 font-medium pr-3 py-1">Mois</th>
							{#each series as serie}
								<th class="text-left text-gray-500 font-medium pr-3 py-1">{serie.label}</th>
							{/each}
						</tr>
					</thead>
					<tbody>
						{#each moisAxe as mois}
							<tr class="border-t border-gray-100">
								<td class="pr-3 py-1 text-gray-700">{libelleMois(mois)}</td>
								{#each series as serie}
									<td class="pr-3 py-1 text-gray-700">
										{serie.valeurs.has(mois) ? `${serie.valeurs.get(mois).toFixed(1)}°C` : '—'}
									</td>
								{/each}
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	{/if}
</div>
