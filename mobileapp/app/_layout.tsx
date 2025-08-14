import React from 'react';
import { PaperProvider } from 'react-native-paper';
import { ApolloProvider } from '@apollo/client';
import { AuthProvider } from '../src/contexts/AuthContext';
import LoginPage from '../src/pages/LoginPage';
import client from './api/apollo';
import { SafeAreaView, StyleSheet } from 'react-native';

export default function RootLayout() {
  return (
    <AuthProvider>
      <ApolloProvider client={client}>
        <PaperProvider>
          <SafeAreaView style={styles.container}>
            <LoginPage />
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
});
