import logo from './logo.svg';
import './App.css';
import useFetch from 'react-fetch-hook';
import QueryData from './components/QueryData';

function App() {
  const { isLoading, data, error } = useFetch("http://192.168.1.249:3000/api/query", {
    formatter: (response) => response.json()
  });
  
  const startServer = async () => {
    await fetch("http://192.168.1.249:3000/api/start", { method: "POST" });
  };

  let content = (error
    ? <p>Unable to contact server: "{error.name}: {error.message}"</p>
    : (isLoading
      ? <h2>Querying server...</h2>
      : <QueryData {...data}></QueryData>
      )
    );

  return (
    <main>
      <h1>Minecraft Server Viewer</h1>
      {content}
      <button onClick={startServer}>Start Minecraft!</button>
    </main>
  );
}

export default App;