import { ApolloProvider } from '@apollo/client';
import { useMemo } from 'react';
import { SafeAreaView, StyleSheet } from 'react-native';
import { PaperProvider } from 'react-native-paper';
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

const App: React.FC = () => {
  const { isAuthenticating, isLoggedIn, setIsAuthenticating, getTokens, saveTokens, logout } =
    useAuth();

  const client = useMemo(() => {
    return createApolloClient({
      isDev: __DEV__,
      setIsAuthenticating,
      getTokens,
      saveTokens,
      logout,
    });
  }, [setIsAuthenticating, getTokens, saveTokens, logout]);

  return (
    <ApolloProvider client={client}>
      <PaperProvider>
        <SafeAreaView style={styles.container}>
          <AppContent isAuthenticating={isAuthenticating} isLoggedIn={isLoggedIn} />
        </SafeAreaView>
      </PaperProvider>
    </ApolloProvider>
  );
};

const styles = StyleSheet.create({ container: { flex: 1 } });

export default App;
