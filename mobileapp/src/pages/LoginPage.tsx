import React, { useState } from 'react';
import { View, StyleSheet } from 'react-native';
import { TextInput, Button, Text } from 'react-native-paper';
import { useMutation } from '@apollo/client';
import { useAuth } from '../contexts/AuthContext';
import { LOGIN_MUTATION } from '../../graphql/mutations';
import { ME_QUERY } from '../../graphql/queries';

const LoginPage = () => {
  const { saveTokens } = useAuth();
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [message, setMessage] = useState<{ text: string; type: 'success' | 'error' } | null>(null);

  const [login, { loading }] = useMutation(LOGIN_MUTATION, {
    onCompleted: async (data) => {
      if (data.login.success) {
        setMessage({ text: 'Login successful!', type: 'success' });
        await saveTokens(data.login.token, data.login.refreshToken);
      } else {
        setMessage({ text: 'Login failed', type: 'error' });
      }
    },
    onError: (error) => {
      setMessage({ text: error.message, type: 'error' });
    },
    refetchQueries: [{ query: ME_QUERY }],
  });

  const handleLogin = () => {
    login({ variables: { username, password }, context: { unauthenticated: true } });
  };

  return (
    <View style={styles.container}>
      <TextInput
        label="Username"
        value={username}
        onChangeText={setUsername}
        style={styles.input}
        autoCapitalize="none"
      />
      <TextInput
        label="Password"
        value={password}
        onChangeText={setPassword}
        secureTextEntry
        style={styles.input}
      />
      <Button mode="contained" onPress={handleLogin} loading={loading} disabled={loading}>
        Login
      </Button>
      {message && (
        <Text style={message.type === 'success' ? styles.successText : styles.errorText}>
          {message.text}
        </Text>
      )}
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    padding: 16,
  },
  input: {
    marginBottom: 16,
  },
  successText: {
    color: 'green',
    marginTop: 16,
    textAlign: 'center',
  },
  errorText: {
    color: 'red',
    marginTop: 16,
    textAlign: 'center',
  },
});

export default LoginPage;
