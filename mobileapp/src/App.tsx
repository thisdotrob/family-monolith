import { ApolloProvider } from '@apollo/client';
import { useMemo } from 'react';
import { SafeAreaView, StyleSheet } from 'react-native';
import { PaperProvider } from 'react-native-paper';
import { createApolloClient } from '@shared/apollo/createApolloClient';
import GlobalLoading from './components/GlobalLoading';
import { useAuth } from '@shared/contexts/AuthContext';
import selectMobileApp from './selectMobileApp';
const HomePage = selectMobileApp();
import LoginPage from './pages/LoginPage';

import { useLastSelection } from './contexts/LastSelectionContext';

type AppContentProps = { isAuthenticating: boolean; isLoggedIn: boolean };
const AppContent = ({ isAuthenticating, isLoggedIn }: AppContentProps) => {
  const { isRestoring } = useLastSelection();
  if (isAuthenticating || isRestoring) {
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
