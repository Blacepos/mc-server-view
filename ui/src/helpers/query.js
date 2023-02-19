

function getLastEvent(setStatus, setError, setLoading) {
	setLoading(true);
  fetch("http://162.232.250.170/api/last-event")
    .then(res => {
      if (res.ok) {
        return res.json();
      }
      throw res;
    })
    .then(res_json => {
      if (res_json.ok) {
        console.log("Parsed json in getLastEvent: ", res_json);
        setStatus(res_json.last_event);
      } else {
        console.log("res_json.ok: ", res_json);
        throw "Server failed to return last event"
      }
    })
	  .catch(err => {
      console.log("Failed to make a fetch: ", err)
      setError(err);
	  })
	  .finally(() => {
		  setLoading(false)
    });
}

export default getLastEvent;