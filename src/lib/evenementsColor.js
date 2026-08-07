export const TYPES_EVENEMENT = [
	'Journal',
	'Récolte',
	'Plantation',
	'Arrosage',
	'Retrait',
	'Semis',
	'Température'
];

export const TYPES_COULEUR_CULTURE = new Set(['Semis', 'Plantation', 'Récolte']);

export const COULEUR_PAR_TYPE = {
	Arrosage: '#3b82f6',
	Journal: '#8b5cf6',
	Retrait: '#f97316',
	Température: '#dcaf33'
};

export const COULEUR_CULTURE_PAR_DEFAUT = '#22c55e';
export const COULEUR_TYPE_PAR_DEFAUT = '#6b7280';

export function couleurEvenement(evenement) {
	const type = evenement.type_evenement;
	const couleur = TYPES_COULEUR_CULTURE.has(type)
		? (evenement.culture_couleur ?? COULEUR_CULTURE_PAR_DEFAUT)
		: (COULEUR_PAR_TYPE[type] ?? COULEUR_TYPE_PAR_DEFAUT);
	return { type, couleur };
}

export function couleurType(type) {
	return TYPES_COULEUR_CULTURE.has(type)
		? COULEUR_CULTURE_PAR_DEFAUT
		: (COULEUR_PAR_TYPE[type] ?? COULEUR_TYPE_PAR_DEFAUT);
}





const anneeCourante = new Date().getFullYear();
export const ANNEES = Array.from({ length: 121 }, (_, i) => anneeCourante - 60 + i);

export const NOMS_MOIS = [
	'Janvier', 'Février', 'Mars', 'Avril', 'Mai', 'Juin',
	'Juillet', 'Août', 'Septembre', 'Octobre', 'Novembre', 'Décembre'
];









