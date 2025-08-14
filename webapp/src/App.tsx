import { useAuth } from './contexts/AuthContext';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';
import LoadingOverlay from './components/LoadingOverlay';

function App() {
  const { token } = useAuth();

  return (
    <>
      <LoadingOverlay />
      {token ? <HomePage /> : <LoginPage />}
    </>
  );
}

export default App;
