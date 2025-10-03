import { View, StyleSheet } from 'react-native';
import { Text, Button } from 'react-native-paper';
import { useAuth } from '@shared/contexts/AuthContext';
import { useApolloClient } from '@apollo/client';

const ProjectsScreen = () => {
  const { logout } = useAuth();
  const client = useApolloClient();

  const logoutOnPress = async () => {
    try {
      await logout();
      client.clearStore();
    } catch (err) {
      console.log(err);
    }
  };

  return (
    <View style={styles.container}>
      <Text variant="headlineMedium" style={styles.title}>
        Projects
      </Text>
      <Text variant="bodyLarge" style={styles.description}>
        Project management will be implemented here.
      </Text>
      <Text variant="bodyMedium" style={styles.placeholder}>
        📁 This is a placeholder screen for the Projects tab.
      </Text>
      
      <Button mode="contained" onPress={logoutOnPress} style={styles.button} icon="logout">
        Logout
      </Button>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  },
  title: {
    marginBottom: 16,
    textAlign: 'center',
  },
  description: {
    marginBottom: 12,
    textAlign: 'center',
  },
  placeholder: {
    marginBottom: 20,
    textAlign: 'center',
    fontStyle: 'italic',
  },
  button: {
    marginTop: 20,
  },
});

export default ProjectsScreen;