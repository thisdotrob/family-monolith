import { ApolloProvider } from '@apollo/client';
import { useMemo } from 'react';
import { createApolloClient } from '@shared/apollo/createApolloClient';
import GlobalLoading from './components/GlobalLoading';
import { useAuth } from '@shared/contexts/AuthContext';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';

type AppContentProps = { isAuthenticating: boolean; isLoggedIn: boolean };
const AppContent = ({ isAuthenticating, isLoggedIn }: AppContentProps) => {
  if (isAuthenticating) {
    return <GlobalLoading />;
  } else if (isLoggedIn) {
    return <HomePage />;
  } else {
    return <LoginPage />;
  }
};

const App = () => {
  const { isAuthenticating, isLoggedIn, setIsAuthenticating, getTokens, saveTokens, logout } =
    useAuth();

  const client = useMemo(() => {
    return createApolloClient({
      isDev: import.meta.env.DEV,
      setIsAuthenticating,
      getTokens,
      saveTokens,
      logout,
    });
  }, [setIsAuthenticating, getTokens, saveTokens, logout]);

  return (
    <ApolloProvider client={client}>
      <AppContent isAuthenticating={isAuthenticating} isLoggedIn={isLoggedIn} />
    </ApolloProvider>
  );
};

export default App;
