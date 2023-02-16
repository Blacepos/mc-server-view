import logo from './logo.svg';
import './App.css';
import QueryData from './components/QueryData'

async function App() {
  let query = await fetch("192.168.1.249:3000/api/query");
  return (
    <main>
      <h1>Test</h1>
      <QueryData data={query}></QueryData>
    </main>
  );
}

export default App;
