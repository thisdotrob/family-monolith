import { useAuth } from './contexts/AuthContext';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';
import GlobalLoading from './components/GlobalLoading';

function App() {
  const { token } = useAuth();

  return (
    <>
      <GlobalLoading />
      {token ? <HomePage /> : <LoginPage />}
    </>
  );
}

export default App;
