export const TYPES_EVENEMENT = [
	'Journal',
	'Récolte',
	'Plantation',
	'Arrosage',
	'Retrait',
	'Semis',
	'Température',
	'Floraison'
];

export const TYPES_COULEUR_CULTURE = new Set(['Semis', 'Plantation', 'Récolte', 'Floraison']);

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

// Couleurs fixes pour le graphique "premières dates de la saison" : là, la
// couleur distingue le TYPE d'événement (pas la culture, qui est déjà le
// panneau/facette), donc volontairement différentes de la couleur propre à
// chaque culture utilisée ailleurs (calendrier, autres graphiques).
export const COULEURS_PREMIERES_DATES_SAISON_EVENEMENTS = {
	Semis: '#22c55e',
	Plantation: '#0ea5e9',
	Floraison: '#8b5cf6',
	Récolte: '#f97316'
};

function tronquer(texte, longueurMax) {
	return texte.length > longueurMax ? `${texte.slice(0, longueurMax - 1)}…` : texte;
}

// Le CSS d'ellipsis n'est pas fiable sur les <option> d'un <select> natif
// selon les navigateurs : on tronque directement la chaîne. Nom et variété
// sont tronqués séparément pour que la variété reste visible même quand le
// nom seul dépasserait déjà la longueur totale.
export function libelleCulture(culture, longueurMaxNom = 16, longueurMaxVariete = 12) {
	const nom = tronquer(culture.nom, longueurMaxNom);
	if (!culture.variete) return nom;
	return `${nom} (${tronquer(culture.variete, longueurMaxVariete)})`;
}

// "Récolte" -> "recolte", "Température" -> "temperature" : fait correspondre
// le nom affiché d'un type d'événement au nom du champ Rust correspondant
// (minuscules, sans accents), sans dictionnaire à maintenir à chaque ajout.
export function cleNombre(type) {
	return type.normalize('NFD').replace(/[̀-ͯ]/g, '').toLowerCase();
}









