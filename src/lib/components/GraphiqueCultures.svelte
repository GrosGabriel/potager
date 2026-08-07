<script>
	let {
		cultures = [],
		evenementsParCulture = new Map(),
		type = null,
		compact = false,
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
		if (moisSelectionnes.length > 0 && !moisSelectionnes.includes(mois)) return false;
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

	let LARGEUR = $derived(compact ? 320 : 640);
	let HAUTEUR = $derived(compact ? 160 : 260);
	const MARGE_GAUCHE = 28;
	const MARGE_DROITE = 8;
	const MARGE_HAUT = 10;
	const MARGE_BAS = 22;
	let LARGEUR_TRACE = $derived(LARGEUR - MARGE_GAUCHE - MARGE_DROITE);
	let HAUTEUR_TRACE = $derived(HAUTEUR - MARGE_HAUT - MARGE_BAS);

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
		const nombre = compact ? 2 : 4;
		return Array.from({ length: nombre + 1 }, (_, i) => Math.round((yMax / nombre) * i));
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

<div class:border={!compact} class:border-gray-200={!compact} class:rounded-lg={!compact} class:p-4={!compact}>
	{#if cultures.length === 0}
		<p class="text-gray-500" class:text-sm={!compact} class:text-xs={compact}>
			Cochez une culture ci-dessus pour afficher son évolution.
		</p>
	{:else if moisAxe.length === 0}
		{#if enChargement}
			<p class="text-gray-500" class:text-sm={!compact} class:text-xs={compact}>Chargement…</p>
		{:else}
			<p class="text-gray-500" class:text-sm={!compact} class:text-xs={compact}>
				Aucun événement enregistré pour {cultures.length > 1 ? 'ces cultures' : 'cette culture'}
				{anneeMin !== null || anneeMax !== null || moisSelectionnes.length > 0 ? 'sur cette période.' : 'pour le moment.'}
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
					<text x={MARGE_GAUCHE - 6} y={y(valeur)} text-anchor="end" dominant-baseline="middle" class="fill-gray-400" font-size={compact ? 8 : 10}>
						{valeur}
					</text>
				{/each}

				{#each moisAxe as mois, i}
					<text
						x={x(i)}
						y={HAUTEUR - MARGE_BAS + (compact ? 13 : 16)}
						text-anchor={i === 0 ? 'start' : i === moisAxe.length - 1 ? 'end' : 'middle'}
						class="fill-gray-400"
						font-size={compact ? 8 : 10}
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
						<circle cx={p.x} cy={p.y} r={compact ? 3 : 4} fill={serie.culture.couleur} stroke="white" stroke-width="1.5" />
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
					class="absolute top-1 -translate-x-1/2 bg-white border border-gray-200 rounded shadow-sm px-2 py-1.5 text-xs pointer-events-none"
					style="left: {pourcentGauche}%"
				>
					<p class="font-medium text-gray-700 mb-1">{libelleMois(moisAxe[survolIndex])}</p>
					{#each pointsParSerie as serie}
						<div class="flex items-center gap-1.5">
							<span class="inline-block w-3 h-0.5" style="background-color: {serie.culture.couleur}"></span>
							{#if !compact}
								<span class="text-gray-500">{serie.culture.nom}</span>
							{/if}
							<span class="font-semibold text-gray-800">{serie.points[survolIndex].valeur}</span>
						</div>
					{/each}
				</div>
			{/if}
		</div>

		{#if !compact}
			{#if cultures.length > 1}
				<div class="flex flex-wrap gap-x-4 gap-y-1.5 mt-3">
					{#each series as serie}
						<span class="flex items-center gap-1.5 text-xs text-gray-600">
							<span class="inline-block w-4 h-0.5" style="background-color: {serie.culture.couleur}"></span>
							{serie.culture.nom}{serie.culture.variete ? ` (${serie.culture.variete})` : ''}
						</span>
					{/each}
				</div>
			{:else}
				<p class="text-xs text-gray-500 mt-2">
					{cultures[0].nom}{cultures[0].variete ? ` (${cultures[0].variete})` : ''}
				</p>
			{/if}
		{/if}

		<button
			type="button"
			class="text-green-700 hover:underline"
			class:text-xs={!compact}
			class:mt-3={!compact}
			class:text-[10px]={compact}
			class:mt-1.5={compact}
			onclick={() => (afficherTableau = !afficherTableau)}
		>
			{afficherTableau ? 'Masquer' : 'Afficher'} les données en tableau
		</button>

		{#if afficherTableau}
			<div class="overflow-x-auto mt-2">
				<table class="w-full" class:text-xs={!compact} class:text-[10px]={compact}>
					<thead>
						<tr>
							<th class="text-left text-gray-500 font-medium pr-3 py-1">Mois</th>
							{#each series as serie}
								<th class="text-left text-gray-500 font-medium pr-3 py-1">{serie.culture.nom}</th>
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
