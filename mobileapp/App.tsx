import { registerRootComponent } from 'expo';
import React from 'react';
import { PaperProvider } from 'react-native-paper';
import { View, Text, StyleSheet } from 'react-native';

function App() {
  return (
    <PaperProvider>
      <View style={styles.container}>
        <Text>Mobile App Placeholder</Text>
      </View>
    </PaperProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
});

registerRootComponent(App);