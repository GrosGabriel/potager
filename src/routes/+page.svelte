<script>
	import IconEvenement from '$lib/components/IconEvenement.svelte';
	import { couleurType } from '$lib/utils.js';
</script>

<div class="max-w-4xl mx-auto px-6 py-12">
	<div class="text-center mb-12">
		<span
			class="inline-flex items-center justify-center w-16 h-16 rounded-full mb-4"
			style="background-color: {couleurType('Plantation')}"
		>
			<span class="w-8 h-8">
				<IconEvenement type="Plantation" />
			</span>
		</span>
		<h1 class="text-4xl font-semibold text-gray-800 mb-2">PotagerLog</h1>
		<p class="text-lg text-gray-500">Journal des événements et des cultures de votre potager.</p>
	</div>

	<div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
		<a
			href="/calendrier"
			class="group flex flex-col items-center gap-3 border border-gray-200 rounded-xl bg-gray-50 p-8 text-center transition-all hover:bg-white hover:border-green-300 hover:shadow-md"
		>
			<span
				class="w-14 h-14 rounded-full bg-green-100 flex items-center justify-center transition-colors group-hover:bg-green-200"
			>
				<svg viewBox="0 0 16 16" fill="none" stroke="#15803d" stroke-width="1.4" class="w-7 h-7">
					<rect x="2.5" y="3" width="11" height="10.5" rx="1.5" />
					<line x1="2.5" y1="6" x2="13.5" y2="6" />
					<line x1="5" y1="1.75" x2="5" y2="4" stroke-linecap="round" />
					<line x1="11" y1="1.75" x2="11" y2="4" stroke-linecap="round" />
					<circle cx="5.4" cy="9" r="0.9" fill="#15803d" stroke="none" />
					<circle cx="8.5" cy="9" r="0.9" fill="#15803d" stroke="none" />
					<circle cx="5.4" cy="11.4" r="0.9" fill="#15803d" stroke="none" />
				</svg>
			</span>
			<span class="text-2xl font-semibold text-gray-800">Calendrier</span>
		</a>

		<a
			href="/statistiques"
			class="group flex flex-col items-center gap-3 border border-gray-200 rounded-xl bg-gray-50 p-8 text-center transition-all hover:bg-white hover:border-green-300 hover:shadow-md"
		>
			<span
				class="w-14 h-14 rounded-full bg-green-100 flex items-center justify-center transition-colors group-hover:bg-green-200"
			>
				<svg viewBox="0 0 16 16" fill="none" stroke="#15803d" stroke-width="1.4" class="w-7 h-7">
					<line x1="2.5" y1="13.5" x2="13.5" y2="13.5" stroke-linecap="round" />
					<rect x="3.5" y="8.5" width="2.4" height="5" rx="0.5" fill="#15803d" stroke="none" />
					<rect x="6.8" y="5.5" width="2.4" height="8" rx="0.5" fill="#15803d" stroke="none" />
					<rect x="10.1" y="2.5" width="2.4" height="11" rx="0.5" fill="#15803d" stroke="none" />
				</svg>
			</span>
			<span class="text-2xl font-semibold text-gray-800">Statistiques</span>
		</a>
	</div>
</div>
