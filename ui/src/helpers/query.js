

function getLastEvent(setStatus, setError, setLoading) {
	setLoading(true);
  fetch("http://192.168.1.249:3000/api/last-event")
    .then(res => {
      if (res.ok) {
        return res.json();
      }
      throw res;
    })
    .then(res_json => {
      if (res_json) {
        console.log("Parsed json in getLastEvent: ", res_json)
        setStatus(res_json);
      } else {
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