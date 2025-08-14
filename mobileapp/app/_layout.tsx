import 'react-native-url-polyfill/auto';
import React from 'react';
import { PaperProvider } from 'react-native-paper';
import { Slot } from 'expo-router';
import { MaterialCommunityIcons } from '@expo/vector-icons';

export default function RootLayout() {
  return (
    <PaperProvider
      settings={{
        icon: (props) => <MaterialCommunityIcons {...props} />,
      }}
    >
      <Slot />
    </PaperProvider>
  );
}
