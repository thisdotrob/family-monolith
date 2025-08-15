import React from 'react';
import { View, StyleSheet } from 'react-native';
import { useQuery } from '@apollo/client';
import { useAuth } from '../contexts/AuthContext';
import { ME_QUERY } from '../../graphql/queries';
import { Text, Button, ActivityIndicator } from 'react-native-paper';

const HomePage = () => {
  const { logout } = useAuth();
  const { data, loading, error } = useQuery(ME_QUERY);

  if (loading) {
    return <ActivityIndicator animating={true} style={styles.centered} />;
  }
  
  if (error) {
    return <Text style={styles.centered}>Error: {error.message}</Text>;
  }

  return (
    <View style={styles.container}>
      <Text variant="headlineLarge">Welcome, {data?.me?.firstName || 'User'}!</Text>
      <Text variant="bodyLarge" style={styles.username}>Your username is: {data?.me?.username}</Text>
      <Button
        mode="contained"
        onPress={logout}
        style={styles.button}
        icon="logout"
      >
        Logout
      </Button>
    </View>
  );
};

const styles = StyleSheet.create({
  centered: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  },
  username: {
    marginVertical: 16,
  },
  button: {
    marginTop: 20,
  },
});

export default HomePage;
