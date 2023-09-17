<script>
    /**
	 * @type {any}
	 */
     export let data;
  </script>
  
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
  
  <style>
    /* Add CSS styling for your table here if needed */
  </style>
  