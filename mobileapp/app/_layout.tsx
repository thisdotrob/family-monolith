import 'react-native-url-polyfill/auto';
import React from 'react';
import { PaperProvider } from 'react-native-paper';
import { Slot } from 'expo-router';
import { ApolloProvider } from '@apollo/client';
import client from './api/apollo';

export default function RootLayout() {
  return (
    <ApolloProvider client={client}>
      <PaperProvider>
        <Slot />
      </PaperProvider>
    </ApolloProvider>
  );
}
