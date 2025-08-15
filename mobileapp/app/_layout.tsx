import React from 'react';
import { PaperProvider } from 'react-native-paper';
import { SafeAreaView, StyleSheet, View, ActivityIndicator } from 'react-native';
import { ApolloProvider } from '@apollo/client';
import { AuthProvider, useAuth } from '../src/contexts/AuthContext';
import client from './api/apollo';
import HomePage from '../src/pages/HomePage';
import LoginPage from '../src/pages/LoginPage';

// We need a component that is a child of AuthProvider to use the useAuth hook.
const AppContent = () => {
  const { token, isLoading } = useAuth();

  if (isLoading) {
    return (
      <View style={styles.centered}>
        <ActivityIndicator size="large" />
      </View>
    );
  }

  return token ? <HomePage /> : <LoginPage />;
}

export default function RootLayout() {
  return (
    <AuthProvider>
      <ApolloProvider client={client}>
        <PaperProvider>
          <SafeAreaView style={styles.container}>
            <AppContent />
          </SafeAreaView>
        </PaperProvider>
      </ApolloProvider>
    </AuthProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  centered: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
});
