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
  const { token, setIsRefreshingToken, saveTokens, logout, } = useAuth();

  const client = useMemo(() => {
    return createApolloClient(setIsRefreshingToken, saveTokens, logout);
  }, [saveTokens, logout, setIsRefreshingToken]);

  return (
    <ApolloProvider client={client}>
      <PaperProvider>
        <SafeAreaView style={styles.container}>
          {token ? <HomePage /> : <LoginPage />}
          <GlobalLoading />
        </SafeAreaView>
      </PaperProvider>
    </ApolloProvider>
  );
}

const styles = StyleSheet.create({ container: { flex: 1 }});

export default App;
