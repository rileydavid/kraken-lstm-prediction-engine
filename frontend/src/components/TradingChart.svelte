<script>
	import { onMount } from 'svelte';
	import * as d3 from 'd3';
	var data = [30, 86, 168, 281, 303, 365];
	
	let chart;

</script>

<div bind:this={chart} class="chart"></div>