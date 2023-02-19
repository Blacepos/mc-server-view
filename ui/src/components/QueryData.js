import useFetch from 'react-fetch-hook';

function QueryData() {
	const { isLoading, data, error } = useFetch("http://162.232.250.170/api/query", {
	  formatter: (response) => response.json()
	})
  
	if (error) {
		return (
			<>
				<h2>Failed to get server status</h2>
				<p>Error: {error.name}: {error.message}</p>
			</>
		);
	} else if (isLoading) {
		return (
			<>
				<h2>Loading server status...</h2>
			</>
		);
	} else {
    return (
      <>
        <h2>Server is online</h2>
        <ul>
          <li>MOTD: {data.status.description.text}</li>
          <li>Players online: {data.status.players.online}</li>
        </ul>
      </>
    );
  }
}

export default QueryData;