import logo from './logo.svg';
import './App.css';
import React, { useState, useEffect } from 'react';
import useFetch from 'react-fetch-hook';
import QueryData from './components/QueryData';
import getLastEvent from './helpers/query'

function App() {
  // const { isLoading, data, error } = useFetch("http://192.168.1.249:3000/api/query", {
  //   formatter: (response) => response.json()
  // });
  
  const startServer = async () => {
    await fetch("http://192.168.1.249:3000/api/start", { method: "POST" });
  };

  const [mc_last_event, setMcLastEvent] = useState("offline");
  const [mc_last_event_err, setMcLastEventErr] = useState(null);
  const [init_loading, setInitLoading] = useState(true);

  useEffect(() => {
    // perform initial loading to get React in sync with Minecraft
    getLastEvent(setMcLastEvent, setMcLastEventErr, setInitLoading);

    // keep React up-to-date with 
    const source = new EventSource("http://192.168.1.249:3000/api/events");
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


  let content = (mc_last_event_err
    ? <p>Unable to contact server: "{mc_last_event_err}"</p>
    : (init_loading
      ? <h2>Querying server...</h2>
      : <QueryData {...mc_last_event}></QueryData>
    ));

  return (
    <main>
      <h1>Minecraft Server Viewer</h1>
      {content}
      <button onClick={startServer}>Start Minecraft!</button>
    </main>
  );
}

export default App;