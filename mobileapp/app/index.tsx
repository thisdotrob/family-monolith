import React, { useState } from 'react';
import { View, StyleSheet } from 'react-native';
import { TextInput, Button, Text, HelperText } from 'react-native-paper';
import { useMutation } from '@apollo/client';
import { LOGIN_MUTATION } from '../graphql/mutations';

interface MessageState {
  text: string;
  type: 'success' | 'error' | '';
}

const LoginPage = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [message, setMessage] = useState<MessageState>({ text: '', type: '' });

  const [login, { loading }] = useMutation(LOGIN_MUTATION, {
    onCompleted: (data) => {
      if (data.login.success) {
        setMessage({ text: 'Login successful!', type: 'success' });
      } else {
        const errorMessage = data.login.errors?.join(', ') || 'An unknown error occurred.';
        setMessage({ text: `Login failed: ${errorMessage}`, type: 'error' });
      }
    },
    onError: (error) => {
      setMessage({ text: `An error occurred: ${error.message}`, type: 'error' });
    },
  });

  const handleSubmit = () => {
    setMessage({ text: '', type: '' });
    login({ variables: { username, password } });
  };

  return (
    <View style={styles.container}>
      <View style={styles.content}>
        <Text variant="headlineLarge" style={styles.title}>Login</Text>

        <View style={styles.messageContainer}>
          {message.text ? (
            <HelperText type={message.type === 'error' ? 'error' : 'info'} visible={true}>
              {message.text}
            </HelperText>
          ) : null}
        </View>

        <TextInput
          label="Username"
          value={username}
          onChangeText={setUsername}
          mode="outlined"
          style={styles.input}
          autoCapitalize="none"
        />
        <TextInput
          label="Password"
          value={password}
          onChangeText={setPassword}
          mode="outlined"
          style={styles.input}
          secureTextEntry
        />
        <Button
          mode="contained"
          onPress={handleSubmit}
          loading={loading}
          disabled={loading}
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