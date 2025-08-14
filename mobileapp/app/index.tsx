import React from 'react';
import { View, StyleSheet } from 'react-native';
import { TextInput, Button, Text } from 'react-native-paper';

const LoginPage = () => {
  return (
    <View style={styles.container}>
      <View style={styles.content}>
        <Text variant="headlineLarge" style={styles.title}>Login</Text>
        
        {/* This View will be used for success/failure messages */}
        <View style={styles.messageContainer}></View>

        <TextInput
          label="Username"
          accessibilityLabel="Username"
          mode="outlined"
          style={styles.input}
          autoCapitalize="none"
        />
        <TextInput
          label="Password"
          accessibilityLabel="Password"
          mode="outlined"
          style={styles.input}
          secureTextEntry
        />
        <Button
          mode="contained"
          style={styles.button}
        >
          Sign In
        </Button>
      </View>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    backgroundColor: '#f5f5f5',
  },
  content: {
    padding: 20,
  },
  title: {
    textAlign: 'center',
    marginBottom: 24,
  },
  messageContainer: {
    marginBottom: 16,
    minHeight: 20,
  },
  input: {
    marginBottom: 16,
  },
  button: {
    marginTop: 8,
  },
});

export default LoginPage;
