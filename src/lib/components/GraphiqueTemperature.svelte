<script>
	import { COULEUR_PAR_TYPE } from '$lib/evenementsColor.js';

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

	// Moyenne, par mois, des relevés intérieurs et extérieurs.
	let moyennesParMois = $derived.by(() => {
		const sommeInt = new Map();
		const sommeExt = new Map();
		const nombreInt = new Map();
		const nombreExt = new Map();
		for (const evenement of evenementsFiltres) {
			const cle = cleMois(evenement.date);
			if (evenement.temperature_int !== null && evenement.temperature_int !== undefined) {
				sommeInt.set(cle, (sommeInt.get(cle) ?? 0) + evenement.temperature_int);
				nombreInt.set(cle, (nombreInt.get(cle) ?? 0) + 1);
			}
			if (evenement.temperature_ext !== null && evenement.temperature_ext !== undefined) {
				sommeExt.set(cle, (sommeExt.get(cle) ?? 0) + evenement.temperature_ext);
				nombreExt.set(cle, (nombreExt.get(cle) ?? 0) + 1);
			}
		}
		const moyInt = new Map([...sommeInt].map(([cle, somme]) => [cle, somme / nombreInt.get(cle)]));
		const moyExt = new Map([...sommeExt].map(([cle, somme]) => [cle, somme / nombreExt.get(cle)]));
		return { moyInt, moyExt };
	});

	let series = $derived([
		{ label: 'Intérieure', couleur: COULEUR_INTERIEURE, valeurs: moyennesParMois.moyInt },
		{ label: 'Extérieure', couleur: COULEUR_EXTERIEURE, valeurs: moyennesParMois.moyExt }
	]);

	let moisAxe = $derived(
		[...new Set(series.flatMap((s) => [...s.valeurs.keys()]))].sort()
	);

	let toutesValeurs = $derived(series.flatMap((s) => moisAxe.map((m) => s.valeurs.get(m)).filter((v) => v !== undefined)));

	let yMin = $derived(toutesValeurs.length ? Math.floor(Math.min(...toutesValeurs)) - 1 : 0);
	let yMax = $derived(toutesValeurs.length ? Math.ceil(Math.max(...toutesValeurs)) + 1 : 10);

	const LARGEUR = 640;
	const HAUTEUR = 260;
	const MARGE_GAUCHE = 32;
	const MARGE_DROITE = 12;
	const MARGE_HAUT = 12;
	const MARGE_BAS = 28;
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
				aria-label="Température moyenne intérieure et extérieure par mois"
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
						text-anchor={i === 0 ? 'start' : i === moisAxe.length - 1 ? 'end' : 'middle'}
						class="fill-gray-400"
						font-size="10"
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
				<div
					class="absolute top-2 -translate-x-1/2 bg-white border border-gray-200 rounded shadow-sm px-3 py-2 text-xs pointer-events-none"
					style="left: {pourcentGauche}%"
				>
					<p class="font-medium text-gray-700 mb-1">{libelleMois(moisAxe[survolIndex])}</p>
					{#each pointsParSerie as serie}
						{@const point = serie.points.find((p) => p.mois === moisAxe[survolIndex])}
						<div class="flex items-center gap-1.5">
							<span class="inline-block w-3 h-0.5" style="background-color: {serie.couleur}"></span>
							<span class="text-gray-500">{serie.label}</span>
							<span class="font-semibold text-gray-800">{point ? `${point.valeur.toFixed(1)}°C` : '—'}</span>
						</div>
					{/each}
				</div>
			{/if}
		</div>

		<div class="flex flex-wrap gap-x-4 gap-y-1.5 mt-3">
			{#each series as serie}
				<span class="flex items-center gap-1.5 text-xs text-gray-600">
					<span class="inline-block w-4 h-0.5" style="background-color: {serie.couleur}"></span>
					{serie.label}
				</span>
			{/each}
		</div>

		<button
			type="button"
			class="text-xs text-green-700 hover:underline mt-3"
			onclick={() => (afficherTableau = !afficherTableau)}
		>
			{afficherTableau ? 'Masquer' : 'Afficher'} les données en tableau
		</button>

		{#if afficherTableau}
			<div class="overflow-x-auto mt-2">
				<table class="text-xs w-full">
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
