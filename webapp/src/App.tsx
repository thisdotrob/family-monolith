import { useAuth } from './contexts/AuthContext';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';

function App() {
  const { token } = useAuth();

  return token ? <HomePage /> : <LoginPage />;
}

export default App;
