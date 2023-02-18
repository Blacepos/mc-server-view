

function QueryData(data) {

	console.log(data);
	if (data.status) {
		return (
			<>
				<h2>Server is online</h2>
				<ul>
					<li>MOTD: {data.status.description.text}</li>
					<li>Players online: {data.status.players.online}</li>
				</ul>
			</>
		);
	} else {
		return (
			<>
				<h2>Server appears to be offline</h2>
			</>
		);
	}
}

export default QueryData;