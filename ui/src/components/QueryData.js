

function QueryData(data) {

	console.log(data);
	if (data.Success) {
		return (
			<>
				<h2>Server is online</h2>
				<ul>
					<li>MOTD: {data.Success.status.description.text}</li>
					<li>Players online: {data.Success.status.players.online}</li>
				</ul>
			</>
		);
	} else {
		return (
			<>
				<h2>Server appears to be offline</h2>
				<p>
					{data.Failure.message}
				</p>
			</>
		);
	}
}

export default QueryData;