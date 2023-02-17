import logo from './logo.svg';
import './App.css';
import useFetch from 'react-fetch-hook';
import QueryData from './components/QueryData';

function App() {
  // let query = await fetch("192.168.1.249:3000/api/query");
  // <main>
  //     <h1>Test</h1>
  //     {/* <QueryData data={query}></QueryData> */}
  //   </main>

  const { isLoading, data, error } = useFetch("http://192.168.1.249:3000/api/query");

  if (error) {
    return (
      <main>
        <p>Here's the data object anyways: {data}</p>
        <p>Error: {error.message}</p>
        <p>Error: {error.name}</p>
      </main>
    )
  }

  return (
    <main>
      <h1>Test</h1>
      {isLoading && <p>Querying server...</p>}
      {!isLoading && <QueryData data={data}></QueryData>}
    </main>
  );
}



export default App;
