import { useState } from 'react';
import { View, StyleSheet } from 'react-native';
import { Text, Button, ActivityIndicator } from 'react-native-paper';
import { useApolloClient, useQuery } from '@apollo/client';
import { useAuth } from '@shared/contexts/AuthContext';
import { ME_QUERY } from '@shared/graphql/queries';
import TasksScreen from '../../../mobileapp/src/components/TasksScreen';
import SavedViewsScreen from '../../../mobileapp/src/components/SavedViewsScreen';

const HomePage = () => {
  const { logout } = useAuth();
  const [currentScreen, setCurrentScreen] = useState<string>('Home');

  const client = useApolloClient();

  const { data, loading, error } = useQuery(ME_QUERY);

  if (loading) {
    return <ActivityIndicator animating={true} style={styles.centered} />;
  }

  if (error) {
    return (
      <View style={styles.container}>
        <Text style={styles.centered}>Error: {error.message}</Text>
      </View>
    );
  }

  const logoutOnPress = async () => {
    try {
      await logout();
      client.clearStore();
    } catch (err) {
      console.log(err);
    }
  };

  const go = (dest: string) => () => {
    setCurrentScreen(dest);
  };

  const navigate = (screen: string) => {
    setCurrentScreen(screen);
  };

  // Render different screens based on current selection
  if (currentScreen === 'Tasks') {
    return <TasksScreen onNavigate={navigate} />;
  }
  
  if (currentScreen === 'SavedViews') {
    return <SavedViewsScreen onNavigate={navigate} />;
  }

  return (
    <View style={styles.container}>
      <Text variant="headlineLarge">Family Takenlijst</Text>
      <Text variant="bodyLarge" style={styles.username}>
        Welcome, {data?.me?.firstName || 'User'} ({data?.me?.username})
      </Text>

      <View style={styles.links}>
        <Text variant="titleMedium" style={styles.linksTitle}>
          Quick links
        </Text>
        <Button mode="outlined" icon="format-list-checkbox" onPress={go('Tasks')} style={styles.linkBtn}>
          Tasks (with Tag Manager)
        </Button>
        <Button mode="outlined" icon="bookmark" onPress={go('SavedViews')} style={styles.linkBtn}>
          Saved Views (with Tag Manager)
        </Button>
        <Button mode="outlined" icon="folder" onPress={go('Projects')} style={styles.linkBtn}>
          Projects
        </Button>
        <Button mode="outlined" icon="history" onPress={go('History')} style={styles.linkBtn}>
          History
        </Button>
      </View>

      <Button mode="contained" onPress={logoutOnPress} style={styles.button} icon="logout">
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
    textAlign: 'center',
  },
  links: {
    width: '100%',
    maxWidth: 360,
    marginVertical: 12,
  },
  linksTitle: {
    textAlign: 'center',
    marginBottom: 4,
  },
  linkBtn: {
    marginVertical: 2,
  },
  button: {
    marginTop: 20,
  },
});

export default HomePage;
