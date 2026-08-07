<script>
	let { type } = $props();
</script>

{#if type === 'Arrosage'}
	<svg viewBox="0 0 16 16" fill="black" class="w-full h-full">
		<path d="M8 1.75 L12.5 9.75 A4.5 4.5 0 1 1 3.5 9.75 Z" />
	</svg>
{:else if type === 'Semis'}
	<svg viewBox="0 0 16 16" fill="black" class="w-full h-full">
		<path d="M8 11.8V6.8" stroke="black" stroke-width="1.8" stroke-linecap="round" fill="none" />
		<path d="M8 6.8C8 6.8 4.5 6.8 4.5 3.3C8 3.3 8 6.8 8 6.8Z" />
		<path d="M8 6.8C8 6.8 11.5 6.8 11.5 3.3C8 3.3 8 6.8 8 6.8Z" />
	</svg>
{:else if type === 'Plantation'}
	<svg viewBox="0 0 16 16" fill="black" class="w-full h-full">
		<path d="M4.5 13.25H11.5L10.5 9.75H5.5L4.5 13.25Z" />
		<path d="M8 9.75V4.25" stroke="black" stroke-width="1.8" stroke-linecap="round" fill="none" />
		<path d="M8 5.75C8 5.75 5 5.75 5 2.75C8 2.75 8 5.75 8 5.75Z" />
		<path d="M8 5.75C8 5.75 11 5.75 11 2.75C8 2.75 8 5.75 8 5.75Z" />
	</svg>
{:else if type === 'Récolte'}
	<svg viewBox="0 0 16 16" fill="none" stroke="black" stroke-width="1.8" class="w-full h-full">
		<path d="M3 6H13L11.3 13.5H4.7L3 6Z" />
		<path d="M5.2 6C5.2 6 5.2 2.2 8 2.2C10.8 2.2 10.8 6 10.8 6" />
		<path d="M5.6 8.7H10.4" />
	</svg>
{:else if type === 'Journal'}
	<svg viewBox="0 0 16 16" fill="none" stroke="black" stroke-width="1.7" class="w-full h-full">
		<rect x="3" y="2" width="10" height="12" rx="1.2" />
		<line x1="5.2" y1="5" x2="10.8" y2="5" />
		<line x1="5.2" y1="8" x2="10.8" y2="8" />
		<line x1="5.2" y1="11" x2="9" y2="11" />
	</svg>
{:else if type === 'Retrait'}
	<svg viewBox="0 0 16 16" fill="none" stroke="black" stroke-width="1.8" class="w-full h-full">
		<circle cx="8" cy="8" r="6" />
		<line x1="5" y1="8" x2="11" y2="8" />
	</svg>
{:else if type === 'Température'}
	<svg viewBox="0 0 16 16" fill="none" stroke="black" stroke-width="1.8" class="w-full h-full">
		<path d="M8 2V10" />
		<circle cx="8" cy="12" r="2" />
	</svg>
{:else if type === 'Supprimer'}
	<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" class="w-full h-full">
		<path d="M3 4.5H13" />
		<path d="M5.5 4.5V3C5.5 2.4 6 2 6.5 2H9.5C10 2 10.5 2.4 10.5 3V4.5" />
		<path d="M4.5 4.5L5 13C5 13.6 5.5 14 6 14H10C10.5 14 11 13.6 11 13L11.5 4.5" />
		<path d="M6.5 7V11" />
		<path d="M9.5 7V11" />
	</svg>
{:else}
	<svg viewBox="0 0 16 16" fill="black" class="w-full h-full">
		<circle cx="8" cy="8" r="2.4" />
	</svg>
{/if}
