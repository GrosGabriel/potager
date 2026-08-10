<script>
	import { page } from '$app/state';

	const onglets = [
		
		{ href: '/calendrier', label: 'Calendrier' },
		{ href: '/statistiques', label: 'Statistiques' }
	];
</script>

<nav class="border-b border-gray-200 bg-white">
	<ul class="flex gap-1 px-4">
		{#each onglets as onglet}
			{@const actif = page.url.pathname === onglet.href}
			<li>
				<a
					href={onglet.href}
					class="inline-block px-4 py-3 text-xl font-medium border-b-2 transition-colors"
					class:border-green-600={actif}
					class:text-green-700={actif}
					class:border-transparent={!actif}
					class:text-gray-500={!actif}
				>
					{onglet.label}
				</a>
			</li>
		{/each}
	</ul>
</nav>
