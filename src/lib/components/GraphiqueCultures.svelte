<script>
	import { libelleCulture } from '$lib/utils.js';
	let {
		cultures = [],
		evenementsParCulture = new Map(),
		type = null,
		enChargement = false,
		anneeMin = null,
		anneeMax = null,
		moisSelectionnes = []
	} = $props();

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
		if (moisSelectionnes.length < 12 && !moisSelectionnes.includes(mois)) return false;
		return true;
	}

	let series = $derived(
		cultures.map((culture) => {
			const evenements = (evenementsParCulture.get(culture.id) ?? []).filter(
				(e) => (type === null || e.type_evenement === type) && dansPeriode(e.date)
			);
			const compteParMois = new Map();
			for (const evenement of evenements) {
				const cle = cleMois(evenement.date);
				compteParMois.set(cle, (compteParMois.get(cle) ?? 0) + 1);
			}
			return { culture, compteParMois };
		})
	);

	let moisAxe = $derived([...new Set(series.flatMap((s) => [...s.compteParMois.keys()]))].sort());

	let valeurMax = $derived(
		Math.max(1, ...series.flatMap((s) => moisAxe.map((m) => s.compteParMois.get(m) ?? 0)), 0)
	);

	let yMax = $derived(Math.max(4, Math.ceil(valeurMax / 2) * 2));

	const LARGEUR = 320;
	const HAUTEUR = 160;
	const MARGE_GAUCHE = 40;
	const MARGE_DROITE = 8;
	const MARGE_HAUT = 10;
	const MARGE_BAS = 52;
	const LARGEUR_TRACE = LARGEUR - MARGE_GAUCHE - MARGE_DROITE;
	const HAUTEUR_TRACE = HAUTEUR - MARGE_HAUT - MARGE_BAS;

	function x(index) {
		if (moisAxe.length <= 1) return MARGE_GAUCHE + LARGEUR_TRACE / 2;
		return MARGE_GAUCHE + (index / (moisAxe.length - 1)) * LARGEUR_TRACE;
	}

	function y(valeur) {
		return MARGE_HAUT + HAUTEUR_TRACE - (valeur / yMax) * HAUTEUR_TRACE;
	}

	let pointsParSerie = $derived(
		series.map((s) => ({
			...s,
			points: moisAxe.map((m, i) => {
				const valeur = s.compteParMois.get(m) ?? 0;
				return { mois: m, valeur, x: x(i), y: y(valeur) };
			})
		}))
	);

	let ticksY = $derived.by(() => {
		const nombre = 2;
		return Array.from({ length: nombre + 1 }, (_, i) => Math.round((yMax / nombre) * i));
	});

	let pivoterLibellesX = $derived(moisAxe.length >= 6);
	let tailleLibellesX = $derived(Math.max(4, (8 * 6) / Math.max(6, moisAxe.length)));

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

<div>
	{#if cultures.length === 0}
		<p class="text-gray-500 text-lg">
			Cochez une culture ci-dessus pour afficher son évolution.
		</p>
	{:else if moisAxe.length === 0}
		{#if enChargement}
			<p class="text-gray-500 text-lg">Chargement…</p>
		{:else}
			<p class="text-gray-500 text-lg">
				Aucun événement enregistré pour {cultures.length > 1 ? 'ces cultures' : 'cette culture'}
				{anneeMin !== null || anneeMax !== null || moisSelectionnes.length < 12 ? 'sur cette période.' : 'pour le moment.'}
			</p>
		{/if}
	{:else}
		<div class="relative">
			<svg
				viewBox="0 0 {LARGEUR} {HAUTEUR}"
				class="w-full h-auto"
				role="img"
				aria-label="Nombre d'événements par mois et par culture"
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
					<text x={MARGE_GAUCHE - 6} y={y(valeur)} text-anchor="end" dominant-baseline="middle" class="fill-gray-400" font-size="8">
						{valeur}
					</text>
				{/each}

				{#each moisAxe as mois, i}
					<text
						x={x(i)}
						y={HAUTEUR - MARGE_BAS + 13}
						text-anchor={pivoterLibellesX || i === moisAxe.length - 1 ? 'end' : 'middle'}
						transform={pivoterLibellesX ? `rotate(-45 ${x(i)} ${HAUTEUR - MARGE_BAS + 13})` : undefined}
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
						stroke={serie.culture.couleur}
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
					{#each serie.points as p}
						<circle cx={p.x} cy={p.y} r="3" fill={serie.culture.couleur} stroke="white" stroke-width="1.5" />
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
					class="absolute top-1 bg-white border border-gray-200 rounded shadow-sm px-4 py-3 pointer-events-none w-max max-w-[18rem] space-y-1"
					style={versLaGauche ? `right: ${100 - pourcentGauche}%` : `left: ${pourcentGauche}%`}
				>
					<p class="font-medium text-lg text-gray-700 mb-1">{libelleMois(moisAxe[survolIndex])}</p>
					{#each pointsParSerie as serie}
						<div class="flex items-center gap-2">
							<span class="inline-block w-4 h-0.5 flex-none" style="background-color: {serie.culture.couleur}"></span>
							<span class="text-gray-500 text-lg break-words min-w-0">
								{libelleCulture(serie.culture)}
							</span>
							<span class="font-semibold text-lg text-gray-800">{serie.points[survolIndex].valeur}</span>
						</div>
					{/each}
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
							<th class="text-left text-gray-500 font-medium pr-3 py-1">Mois</th>
							{#each series as serie}
								<th class="text-left text-gray-500 font-medium pr-3 py-1">{libelleCulture(serie.culture)}</th>
							{/each}
						</tr>
					</thead>
					<tbody>
						{#each moisAxe as mois}
							<tr class="border-t border-gray-100">
								<td class="pr-3 py-1 text-gray-700">{libelleMois(mois)}</td>
								{#each series as serie}
									<td class="pr-3 py-1 text-gray-700">{serie.compteParMois.get(mois) ?? 0}</td>
								{/each}
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	{/if}
</div>
