<script>
	import IconEvenement from './IconEvenement.svelte';
	import { couleurEvenement, COULEUR_PAR_TYPE, COULEUR_CULTURE_PAR_DEFAUT } from '$lib/evenementsColor.js';

	let { selected = $bindable(null), evenements = [] } = $props();

	function styleEvenement(evenement) {
		return couleurEvenement(evenement);
	}

	function dateVersISO(d) {
		const annee = d.getFullYear();
		const mois = String(d.getMonth() + 1).padStart(2, '0');
		const jour = String(d.getDate()).padStart(2, '0');
		return `${annee}-${mois}-${jour}`;
	}

	let evenementsParDate = $derived.by(() => {
		const map = new Map();
		for (const evenement of evenements) {
			const liste = map.get(evenement.date) ?? [];
			liste.push(evenement);
			map.set(evenement.date, liste);
		}
		return map;
	});

	const depart = selected ? new Date(selected) : new Date();

	let moisAffiche = $state(depart.getMonth());

	let anneeAffichee = $state(depart.getFullYear());



	let viewDate = $derived(new Date(anneeAffichee, moisAffiche, 1));

	const joursSemaine = ['Lun', 'Mar', 'Mer', 'Jeu', 'Ven', 'Sam', 'Dim'];
	const nomsMois = [
		'Janvier', 'Février', 'Mars', 'Avril', 'Mai', 'Juin',
		'Juillet', 'Août', 'Septembre', 'Octobre', 'Novembre', 'Décembre'
	];

	const anneeCourante = new Date().getFullYear();
	const annees = Array.from({ length: 101 }, (_, i) => anneeCourante - 50 + i);

	function estMemeJour(a, b) {
		return (
			a instanceof Date &&
			b instanceof Date &&
			a.getFullYear() === b.getFullYear() &&
			a.getMonth() === b.getMonth() &&
			a.getDate() === b.getDate()
		);
	}

	let cellules = $derived.by(() => {
		const annee = viewDate.getFullYear();
		const mois = viewDate.getMonth();
		const premierJour = new Date(annee, mois, 1);
		const decalage = (premierJour.getDay() + 6) % 7; // semaine commence le lundi
		const joursDansMois = new Date(annee, mois + 1, 0).getDate();
		const totalCellules = Math.ceil((decalage + joursDansMois) / 7) * 7;

		const aujourdHui = new Date();

		return Array.from({ length: totalCellules }, (_, i) => {
			const date = new Date(annee, mois, 1 - decalage + i);
			return {
				date,
				horsMois: date.getMonth() !== mois,
				estAujourdHui: estMemeJour(date, aujourdHui),
				estSelectionne: estMemeJour(date, selected),
				evenementsDuJour: evenementsParDate.get(dateVersISO(date)) ?? []
			};
		});
	});

	let nombreLignes = $derived(cellules.length / 7);

	function moisPrecedent() {
		if (moisAffiche === 0) {
			moisAffiche = 11;
			anneeAffichee -= 1;
		} else {
			moisAffiche -= 1;
		}
	}

	function moisSuivant() {
		if (moisAffiche === 11) {
			moisAffiche = 0;
			anneeAffichee += 1;
		} else {
			moisAffiche += 1;
		}
	}

	function allerAujourdhui() {
		const aujourdHui = new Date();
		moisAffiche = aujourdHui.getMonth();
		anneeAffichee = aujourdHui.getFullYear();
		selected = aujourdHui;
	}

	function choisirJour(date) {
		selected = date;
	}

	const legende = [
		{ type: 'Semis', couleur: COULEUR_CULTURE_PAR_DEFAUT, note: '(couleur de la culture)' },
		{ type: 'Plantation', couleur: COULEUR_CULTURE_PAR_DEFAUT, note: '(couleur de la culture)' },
		{ type: 'Récolte', couleur: COULEUR_CULTURE_PAR_DEFAUT, note: '(couleur de la culture)' },
		{ type: 'Arrosage', couleur: COULEUR_PAR_TYPE.Arrosage },
		{ type: 'Journal', couleur: COULEUR_PAR_TYPE.Journal },
		{ type: 'Retrait', couleur: COULEUR_PAR_TYPE.Retrait }
	];
</script>

<div class="w-full h-full flex flex-col @container">
	<div class="flex items-center justify-between mb-4">
		<button
			type="button"
			onclick={moisPrecedent}
			class="px-3 py-1.5 rounded hover:bg-gray-100 text-gray-600 text-lg"
			aria-label="Mois précédent"
		>
			←
		</button>
		<div class="flex items-center gap-2">
			<select bind:value={moisAffiche} class="text-base font-medium rounded border-gray-200 py-1">
				{#each nomsMois as nom, i}
					<option value={i}>{nom}</option>
				{/each}
			</select>
			<select bind:value={anneeAffichee} class="text-base font-medium rounded border-gray-200 py-1">
				{#each annees as annee}
					<option value={annee}>{annee}</option>
				{/each}
			</select>
		</div>
		<button
			type="button"
			onclick={moisSuivant}
			class="px-3 py-1.5 rounded hover:bg-gray-100 text-gray-600 text-lg"
			aria-label="Mois suivant"
		>
			→
		</button>
	</div>

	<div class="text-right mb-2">
		<button type="button" onclick={allerAujourdhui} class="text-sm text-green-700 hover:underline">
			Aujourd'hui
		</button>
	</div>

	<div class="grid grid-cols-7 gap-1.5 text-center text-sm font-medium text-gray-500 mb-1">
		{#each joursSemaine as jour}
			<div>{jour}</div>
		{/each}
	</div>

	<div
		class="grid grid-cols-7 gap-1.5 flex-1 min-h-0"
		style="grid-template-rows: repeat({nombreLignes}, minmax(0, 1fr));"
	>
		{#each cellules as cellule}
			<button
				type="button"
				onclick={() => choisirJour(cellule.date)}
				class="@container rounded-lg transition-colors flex flex-col items-center justify-center gap-[6%] p-[6%] overflow-hidden"
				class:text-gray-300={cellule.horsMois}
				class:text-gray-700={!cellule.horsMois && !cellule.estSelectionne}
				class:bg-green-600={cellule.estSelectionne}
				class:text-white={cellule.estSelectionne}
				class:ring-1={cellule.estAujourdHui && !cellule.estSelectionne}
				class:ring-green-500={cellule.estAujourdHui && !cellule.estSelectionne}
				class:hover:bg-gray-100={!cellule.estSelectionne}
			>
				<span class="leading-none text-[clamp(0.65rem,26cqw,1.5rem)]">{cellule.date.getDate()}</span>
				{#if cellule.evenementsDuJour.length > 0}
					<span class="flex items-center justify-center gap-[4%] w-full min-w-0 leading-none">
						{#each cellule.evenementsDuJour.slice(0, 3) as evenement (evenement.id)}
							{@const style = styleEvenement(evenement)}
							<span
								class="aspect-square flex-1 min-w-0 max-w-[clamp(0.75rem,30cqw,2rem)] rounded-full flex items-center justify-center leading-none"
								style="background-color: {style.couleur}"
								title="{evenement.type_evenement}{evenement.culture_nom ? ' – ' + evenement.culture_nom : ''}"
							>
								<span class="w-[72%] h-[72%]">
									<IconEvenement type={style.type} />
								</span>
							</span>
						{/each}
						{#if cellule.evenementsDuJour.length > 3}
							<span class="flex-none text-[clamp(0.55rem,16cqw,0.85rem)] leading-none" class:text-black={cellule.estSelectionne} class:text-gray-500={!cellule.estSelectionne}>
								+{cellule.evenementsDuJour.length - 3}
							</span>
						{/if}
					</span>
				{/if}
			</button>
		{/each}
	</div>

	<div class="flex flex-wrap gap-x-4 gap-y-1.5 mt-4 text-[clamp(0.7rem,1.6cqw,0.85rem)] text-gray-500">
		{#each legende as item}
			<span class="flex items-center gap-1.5">
				<span
					class="inline-flex items-center justify-center rounded-full leading-none shrink-0 w-[clamp(0.9rem,3.5cqw,1.5rem)] h-[clamp(0.9rem,3.5cqw,1.5rem)]"
					style="background-color: {item.couleur}"
				>
					<span class="w-[72%] h-[72%]">
						<IconEvenement type={item.type} />
					</span>
				</span>
				{item.type} {item.note ?? ''}
			</span>
		{/each}
	</div>
</div>
