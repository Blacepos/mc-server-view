import logo from './logo.svg';
import './App.css';
import React, { useState, useEffect } from 'react';
import QueryData from './components/QueryData';
import getLastEvent from './helpers/query'

function App() {
    
  const startServer = async () => {
    await fetch("http://162.232.250.170/api/start", { method: "POST" });
  };

  const [mc_last_event, setMcLastEvent] = useState("offline");
  const [mc_last_event_err, setMcLastEventErr] = useState(null);
  const [init_loading, setInitLoading] = useState(true);


  useEffect(() => {
    // perform initial loading to get React in sync with Minecraft
    getLastEvent(setMcLastEvent, setMcLastEventErr, setInitLoading);

    // keep React up-to-date with 
    const source = new EventSource("http://162.232.250.170/api/events");
    source.addEventListener("offline",  () => setMcLastEvent("offline"));
    source.addEventListener("starting", () => setMcLastEvent("starting"));
    source.addEventListener("online",   () => setMcLastEvent("online"));
    source.addEventListener("empty",    () => setMcLastEvent("empty"));
    source.addEventListener("occupied", () => setMcLastEvent("occupied"));
    source.addEventListener("crashed",  () => setMcLastEvent("crashed"));
    
    // Error occurs:
    // source.onerror
    return () => {
      source.close();
    };
  }, []);


  let content =
    (mc_last_event_err
    ? <>
        <p>Unable to contact server: "{mc_last_event_err}"</p>
      </>
    : (init_loading
      ? <>
          <h2>Querying server...</h2>
        </>
      : (mc_last_event === "online" || mc_last_event === "empty" || mc_last_event === "occupied"
        ? <>
            <p>most recent event: <i>{mc_last_event}</i></p>
            <QueryData></QueryData>
          </>
        : <>
            <p>most recent event: <i>{mc_last_event}</i></p>
            <h2>Server is offline</h2>
            <button onClick={startServer}>Start Minecraft!</button>
          </>
      )
    ));

  return (
    <main>
      <h1>Minecraft Server Viewer</h1>
      {content}
    </main>
  );
}

export default App;