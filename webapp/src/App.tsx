import { ApolloProvider } from '@apollo/client';
import { useMemo } from 'react';
import { createApolloClient } from './api/apollo';
import GlobalLoading from './components/GlobalLoading';
import { useAuth } from './contexts/AuthContext';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';

const AppContent = ({ isAuthenticating, isLoggedIn }) => {
  if (isAuthenticating) {
    return (<GlobalLoading />);
  } else if (isLoggedIn) {
    return (<HomePage />);
  } else {
    return (<LoginPage />);
  }
}

const App = () => {
  const { isAuthenticating, isLoggedIn, setIsAuthenticating, getTokens, saveTokens, logout } = useAuth();

  const client = useMemo(() => {
    return createApolloClient(setIsAuthenticating, getTokens, saveTokens, logout);
  }, [setIsAuthenticating, getTokens, saveTokens, logout]);

  return (
    <ApolloProvider client={client}>
      <AppContent isAuthenticating={isAuthenticating} isLoggedIn={isLoggedIn} />
    </ApolloProvider>
  );
}

export default App;
