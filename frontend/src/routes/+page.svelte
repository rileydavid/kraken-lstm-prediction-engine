<script>
	import TradeTable from '../components/TradeTable.svelte';
	import RealTimeChart from '../components/TradeTable.svelte';
	import TradingChart from '../components/TradingChart.svelte';

    import * as d3 from 'd3';

	const trades = async () => {
		const response = await fetch('localhost:8080/trades/15');
		return await response.json();
	};

    const data = [
		{
			time: '2023-09-17T13:47:44.307784Z',
			price: '0.49797000',
			volume: '1608.45934512',
			side: 's',
			order_type: 'l',
			symbol: 'XRPUSD'
		},
		{
			time: '2023-09-17T13:47:44.307868Z',
			price: '0.49797000',
			volume: '562.5000',
			side: 's',
			order_type: 'l',
			symbol: 'XRPUSD'
		},
		{
			time: '2023-09-17T13:47:44.307919Z',
			price: '0.49796000',
			volume: '855',
			side: 's',
			order_type: 'l',
			symbol: 'XRPUSD'
		},
		{
			time: '2023-09-17T13:47:44.307966Z',
			price: '0.49793000',
			volume: '562.5000',
			side: 's',
			order_type: 'l',
			symbol: 'XRPUSD'
		},
		{
			time: '2023-09-17T13:47:44.308008Z',
			price: '0.49788000',
			volume: '3011.74580865',
			side: 's',
			order_type: 'l',
			symbol: 'XRPUSD'
		},
		{
			time: '2023-09-17T13:47:44.308050Z',
			price: '0.49787000',
			volume: '3988.79484623',
			side: 's',
			order_type: 'l',
			symbol: 'XRPUSD'
		},
		{
			time: '2023-09-17T13:47:44.308176Z',
			price: '0.49787000',
			volume: '1032.55152846',
			side: 's',
			order_type: 'l',
			symbol: 'XRPUSD'
		},
		{
			time: '2023-09-17T13:47:44.308225Z',
			price: '0.49787000',
			volume: '1980',
			side: 's',
			order_type: 'l',
			symbol: 'XRPUSD'
		},
		{
			time: '2023-09-17T13:47:44.308269Z',
			price: '0.49787000',
			volume: '562.5000',
			side: 's',
			order_type: 'l',
			symbol: 'XRPUSD'
		},
		{
			time: '2023-09-17T13:47:44.308304Z',
			price: '0.49783000',
			volume: '4428.58077726',
			side: 's',
			order_type: 'l',
			symbol: 'XRPUSD'
		},
		{
			time: '2023-09-17T13:47:44.308342Z',
			price: '0.49783000',
			volume: '1407.36769428',
			side: 's',
			order_type: 'l',
			symbol: 'XRPUSD'
		}
	];

	var margin = {top: 10, right: 30, bottom: 30, left: 60},
    width = 460 - margin.left - margin.right,
    height = 400 - margin.top - margin.bottom;
	// append the svg object to the body of the page
	var svg = d3
		.select('#my_dataviz')
		.append('svg')
		.attr('width', width + margin.left + margin.right)
		.attr('height', height + margin.top + margin.bottom)
		.append('g')
		.attr('transform', 'translate(' + margin.left + ',' + margin.top + ')');

	//Read the data
	d3.csv(
		'https://raw.githubusercontent.com/holtzy/data_to_viz/master/Example_dataset/3_TwoNumOrdered_comma.csv',

		// When reading the csv, I must format variables:
		function (d) {
			return { date: d3.timeParse('%YYYY-%mm-%dd')(d.time), value: d.price };
		},

		// Now I can use this dataset:
		function (data) {
			// Add X axis --> it is a date format
			var x = d3
				.scaleTime()
				.domain(
					d3.extent(data, function (d) {
						return d.time;
					})
				)
				.range([0, width]);
			svg
				.append('g')
				.attr('transform', 'translate(0,' + height + ')')
				.call(d3.axisBottom(x));

			// Add Y axis
			var y = d3
				.scaleLinear()
				.domain([
					0,
					d3.max(data, function (d) {
						return +d.price;
					})
				])
				.range([height, 0]);
			svg.append('g').call(d3.axisLeft(y));

			// Add the line
			svg
				.append('path')
				.datum(data)
				.attr('fill', 'none')
				.attr('stroke', 'steelblue')
				.attr('stroke-width', 1.5)
				.attr(
					'd',
					d3
						.line()
						.x(function (d) {
							return x(d.time);
						})
						.y(function (d) {
							return y(d.price);
						})
				);
		}
	);


</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

{#await trades}
	<p>...waiting</p>
{:then}
	<table>
		<thead>
			<tr>
				<th>Time</th>
				<th>Price</th>
				<th>Volume</th>
				<th>Side</th>
				<th>Order Type</th>
				<th>Symbol</th>
			</tr>
		</thead>
		<tbody>
			{#each data as item (item.time)}
				<tr>
					<td>{item.time}</td>
					<td>{item.price}</td>
					<td>{item.volume}</td>
					<td>{item.side}</td>
					<td>{item.order_type}</td>
					<td>{item.symbol}</td>
				</tr>
			{/each}
		</tbody>
	</table>
{:catch error}
	<p>An error occurred!</p>
{/await}

<!-- <TradingChart {data} /> -->

<!--  <TradeTable {data} /> -->
<TradingChart />
