import { useState } from 'react';
import { View, StyleSheet } from 'react-native';
import { TextInput as PaperTextInput, Button, Text } from 'react-native-paper';
const AnyTextInput = PaperTextInput as any;
import { useMutation } from '@apollo/client';
import { useAuth } from '@shared/contexts/AuthContext';
import { LOGIN_MUTATION } from '@shared/graphql/mutations';
const LoginPage = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [message, setMessage] = useState<{ text: string; type: 'success' | 'error' } | null>(null);

  const { saveTokens } = useAuth();

  const [login, { loading }] = useMutation(LOGIN_MUTATION, {
    onCompleted: async (data) => {
      if (data.login.success) {
        const { token, refreshToken } = data.login;
        await saveTokens(token, refreshToken);
        setMessage({ text: 'Login successful!', type: 'success' });
      } else {
        setMessage({ text: 'Login failed', type: 'error' });
      }
    },
    onError: (error) => {
      setMessage({ text: error.message, type: 'error' });
    },
  });

  const handleLogin = () => {
    login({ variables: { username, password }, context: { unauthenticated: true } });
  };

  return (
    <View style={styles.container}>
      <AnyTextInput
        label="Username"
        value={username}
        onChangeText={setUsername}
        style={styles.input}
        autoCapitalize="none"
        testID="username-input"
      />
      <AnyTextInput
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
