

function QueryData(data) {
	if (data.Success) {
		return (
			<ul>
				<li>{data.Success.motd}</li>
			</ul>
		);
	} else {
		return (<p>The server appears to be offline.</p>);
	}
}

export default QueryData;