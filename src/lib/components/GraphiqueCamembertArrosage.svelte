<script>
	import { libelleCulture } from '$lib/utils.js';
	let {
		cultures = [],
		evenementsParCulture = new Map(),
		anneeMin = null,
		anneeMax = null,
		moisSelectionnes = [],
		enChargement = false
	} = $props();

	function dansPeriode(dateISO) {
		const annee = Number(dateISO.slice(0, 4));
		const mois = Number(dateISO.slice(5, 7));
		if (anneeMin !== null && annee < anneeMin) return false;
		if (anneeMax !== null && annee > anneeMax) return false;
		if (moisSelectionnes.length < 12 && !moisSelectionnes.includes(mois)) return false;
		return true;
	}

	let parts = $derived(
		cultures
			.map((culture) => {
				const evenements = evenementsParCulture.get(culture.id) ?? [];
				const total = evenements
					.filter((e) => e.type_evenement === 'Arrosage' && e.temps_arrosage && dansPeriode(e.date))
					.reduce((somme, e) => somme + e.temps_arrosage, 0);
				return { culture, total };
			})
			.filter((p) => p.total > 0)
	);

	let totalGeneral = $derived(parts.reduce((somme, p) => somme + p.total, 0));

	const CX = 100;
	const CY = 100;
	const RAYON = 80;

	let tranches = $derived.by(() => {
		let angle = 0;
		return parts.map((p) => {
			const angleDebut = angle;
			const angleFin = angle + (p.total / totalGeneral) * 360;
			angle = angleFin;
			return { ...p, angleDebut, angleFin };
		});
	});

	function point(angleDeg) {
		const rad = ((angleDeg - 90) * Math.PI) / 180;
		return { x: CX + RAYON * Math.cos(rad), y: CY + RAYON * Math.sin(rad) };
	}

	function tracerTranche(angleDebut, angleFin) {
		const p1 = point(angleDebut);
		const p2 = point(angleFin);
		const grandArc = angleFin - angleDebut > 180 ? 1 : 0;
		return `M ${CX} ${CY} L ${p1.x} ${p1.y} A ${RAYON} ${RAYON} 0 ${grandArc} 1 ${p2.x} ${p2.y} Z`;
	}

	let survolId = $state(null);
	let survolPosition = $state({ x: 50, y: 50 });

	function surSurvolConteneur(event) {
		const rect = event.currentTarget.getBoundingClientRect();
		survolPosition = {
			x: ((event.clientX - rect.left) / rect.width) * 100,
			y: ((event.clientY - rect.top) / rect.height) * 100
		};
	}

	function surSortieConteneur() {
		survolId = null;
	}

	let trancheSurvolee = $derived(tranches.find((t) => t.culture.id === survolId) ?? null);
</script>

<div class="min-h-[22rem] flex flex-col items-center justify-center">
	{#if enChargement}
		<p class="text-gray-500 text-lg text-center w-full">Chargement…</p>
	{:else if parts.length === 0}
		<p class="text-gray-500 text-lg text-center w-full">Aucun arrosage enregistré sur cette période.</p>
	{:else}
		<div
			class="relative w-64 h-64"
			onpointermove={surSurvolConteneur}
			onpointerleave={surSortieConteneur}
			aria-hidden="true"
		>
			<svg viewBox="0 0 200 200" class="w-full h-full" role="img" aria-label="Répartition du temps d'arrosage par culture">
				{#if tranches.length === 1}
					<circle
						cx={CX}
						cy={CY}
						r={RAYON}
						fill={tranches[0].culture.couleur}
						aria-hidden="true"
						onpointerenter={() => (survolId = tranches[0].culture.id)}
					/>
				{:else}
					{#each tranches as tranche}
						<path
							d={tracerTranche(tranche.angleDebut, tranche.angleFin)}
							fill={tranche.culture.couleur}
							stroke="white"
							stroke-width="2"
							aria-hidden="true"
							onpointerenter={() => (survolId = tranche.culture.id)}
						/>
					{/each}
				{/if}
			</svg>

			{#if trancheSurvolee}
				<div
					class="absolute bg-white border border-gray-200 rounded shadow-sm px-4 py-3 pointer-events-none w-max max-w-[18rem] space-y-1"
					style="left: {survolPosition.x}%; top: {survolPosition.y}%; transform: translate({survolPosition.x > 50 ? '-100%' : '0%'}, {survolPosition.y > 50 ? '-100%' : '0%'});"
				>
					<div class="flex items-center gap-2">
						<span class="inline-block w-3 h-3 rounded-full flex-none" style="background-color: {trancheSurvolee.culture.couleur}"></span>
						<span class="text-gray-500 text-lg break-words min-w-0">
							{libelleCulture(trancheSurvolee.culture)}
						</span>
					</div>
					<p class="font-semibold text-lg text-gray-800 mt-0.5">
						{trancheSurvolee.total} minutes
						<span class="font-normal text-gray-500">({Math.round((trancheSurvolee.total / totalGeneral) * 100)} %)</span>
					</p>
				</div>
			{/if}
		</div>

		<p class="text-center text-lg text-gray-600 mt-3">
			Durée totale d'arrosage : <span class="font-semibold text-lg text-gray-800">{totalGeneral} minutes</span>
		</p>
	{/if}
</div>
