import { ApolloProvider } from '@apollo/client';
import { useMemo } from 'react';
import { createApolloClient } from './api/apollo';
import GlobalLoading from './components/GlobalLoading';
import { useAuth } from './contexts/AuthContext';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';

function App() {
  const { token, saveTokens, logout, setIsRefreshingToken } = useAuth();

  const client = useMemo(() => {
    return createApolloClient(setIsRefreshingToken, saveTokens, logout);
  }, [saveTokens, logout, setIsRefreshingToken]);

  // This effect will reset the store when the token changes
  // which is what we want after login/logout.
  useMemo(() => {
    client.clearStore();
  }, [token, client]);

  return (
    <ApolloProvider client={client}>
      {token ? <HomePage /> : <LoginPage />}
      <GlobalLoading />
    </ApolloProvider>
  );
}

export default App;
