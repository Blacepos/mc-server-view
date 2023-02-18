

function getMcStatus(setStatus, setError, setLoading) {
	fetch("http://192.168.1.249:3000/api/online")
      .then(res => {
        if (res.ok) {
          return res.json();
        }
        throw response;
      })
      .then(res_json => {
        setStatus(res_json);
      })
	  .catch(err => {
		console.log("Failed to make a fetch: ", error)
		setError(err);
	  })
	  .finally(() => {
		setLoading(false)
	  });
}

export default getMcStatus;

/*
enum Status {
    Offline,
    Starting,
    Online(Idle),
}

enum Idle {
    Idle,
    Occupied
}

struct Status2 {
    state: String,
    idle: String,
}
*/