import { ApolloProvider } from '@apollo/client';
import { useMemo } from 'react';
import { SafeAreaView, StyleSheet } from 'react-native';
import { PaperProvider } from 'react-native-paper';
import { createApolloClient } from './api/apollo';
import GlobalLoading from './components/GlobalLoading';
import { useAuth } from './contexts/AuthContext';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';

function App() {
  const { isLoggedIn, setIsAuthenticating, saveTokens, logout } = useAuth();

  const client = useMemo(() => {
    return createApolloClient(setIsAuthenticating, saveTokens, logout);
  }, [setIsAuthenticating, saveTokens, logout]);

  return (
    <ApolloProvider client={client}>
      <PaperProvider>
        <SafeAreaView style={styles.container}>
          {isLoggedIn ? <HomePage /> : <LoginPage />}
          <GlobalLoading />
        </SafeAreaView>
      </PaperProvider>
    </ApolloProvider>
  );
}

const styles = StyleSheet.create({ container: { flex: 1 }});

export default App;
